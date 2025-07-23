pub mod constants;
mod cr_property;

use crate::angle_unit::AngleUnit;
use crate::constructive_real::ConstructiveReal;
use crate::constructive_real::constants::{LN_10, ONE, PI};
use crate::error::DomainViolation::{
    AsinDomainViolation, DivisionByZero, FactorialDomainViolation, NthRoot, OrdinalDomainViolation,
    TanDomainViolation,
};
use crate::error::FactorialDomainViolation::{NegativeBase, NonIntegerBase};
use crate::error::InternalError::UnconstructableFloat;
use crate::error::NumError::{DomainViolation, Overflow};
use crate::error::OrdinalDomainViolation::{
    NegativeBaseNonIntegerOrder, ZeroBaseNegativeOrder, ZeroBaseZeroOrder,
};
use crate::error::{CancelCheckable, NumError, NumResult};
use crate::rational_extensions::RationalExtensions;
use crate::real;
use crate::real::constants::{
    E, HALF, HALF_SQRT_2, HALF_SQRT_3, PI_OVER_2, PI_OVER_3, PI_OVER_4, PI_OVER_6, SQRT_3,
    THIRD_SQRT_3, TWO, ZERO,
};
use crate::real::cr_property::{CRProperty, OptionalCRProperty};
use cancellation_token::CancellationToken;
use num::bigint::Sign;
use num::complex::ComplexFloat;
use num::traits::Inv;
use num::{BigInt, BigRational, FromPrimitive, Integer, One, Signed, ToPrimitive, Zero};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::sync::LazyLock;

static COMMON_POWER_LENGTH_LIMIT: u64 = 200;

/// Number of extra bits used in {@link toStringTruncated} evaluation to prefer truncation to
/// rounding. Must be <= 30.
static EXTRA_PREC: i32 = 10;

/// Default comparison tolerances, in bits
static DEFAULT_INITIAL_TOLERANCE: i32 = -100; // For rough comparison.
static DEFAULT_RELATIVE_TOLERANCE: i32 = -1000; // Used only in is_comparable.
static DEFAULT_COMPARISON_TOLERANCE: i32 = -3500; // Absolute tolerance.
/// Roughly the number of leading zeroes we're willing to accept in comparisons.
static ZERO_COMPARISON_TOLERANCE: i32 = -5000; // Absolute tolerance.

/// Don't track ln() or log() arguments whose representation is larger than this.
static LOG_ARG_BITS: i32 = 100;
/// Don't even attempt to simplify ln() or log() arguments larger than this.
static LOG_ARG_CANDIDATE_BITS: f64 = 2000.;

/// Small integers for which we try to recognize ln(small_int^n), so we can simplify it to
/// n*ln(small_int).
static SMALL_NON_POWERS: [i32; 6] = [2, 3, 5, 6, 7, 10];

/// The (in abs value) integral exponent for which we attempt to use a recursive
/// algorithm for evaluating pow(). The recursive algorithm works independent of the sign of the
/// base, and can produce rational results. But it can become slow for very large exponents.
static RECURSIVE_POW_LIMIT: LazyLock<BigInt> = LazyLock::new(|| BigInt::from_i32(1000).unwrap());

/// The corresponding limit when we're using rational arithmetic. This should fail fast
/// anyway, but we avoid ridiculously deep recursion.
static HARD_RECURSIVE_POW_LIMIT: LazyLock<BigInt> = LazyLock::new(|| BigInt::one() << 1000);

/// In some cases we cowardly refuse to compute answers longer than BIT_LIMIT, normally because
/// doing so is likely to cause us to run out of space in unpleasant ways.
static BIT_LIMIT: i32 = 2_000_000;
static BIT_LIMIT_AS_REAL: LazyLock<Real> =
    LazyLock::new(|| Real::new_from_rational(BigRational::from_i32(BIT_LIMIT).unwrap()));

#[derive(Clone)]
pub struct Real {
    rat: BigRational,
    cr: ConstructiveReal,
    cr_property: Option<CRProperty>,
}

impl Real {
    pub fn new(rat: BigRational, cr: ConstructiveReal, cr_property: Option<CRProperty>) -> Self {
        Self {
            rat,
            cr,
            cr_property,
        }
    }

    /// Shorthand constructor; computes non-null property only for a few special cases.
    pub fn new_from_rat_cr(rat: BigRational, cr: ConstructiveReal) -> Self {
        Self {
            rat,
            cr_property: Option::<CRProperty>::from(cr.clone()),
            cr,
        }
    }

    pub fn new_from_cr(cr: ConstructiveReal) -> Self {
        Self::new_from_rat_cr(BigRational::one(), cr)
    }

    pub fn new_from_cr_property(cr: ConstructiveReal, cr_property: Option<CRProperty>) -> Self {
        Self::new(BigRational::one(), cr, cr_property)
    }

    pub fn new_from_rat_property(rat: BigRational, cr_property: CRProperty) -> Self {
        Self::new(rat, cr_property.cr().unwrap().unwrap(), Some(cr_property))
    }

    pub fn new_from_property(cr_property: CRProperty) -> Self {
        Self::new_from_rat_property(BigRational::one(), cr_property)
    }

    pub fn new_from_rational(rat: BigRational) -> Self {
        Self::new(rat, ONE.clone(), Some(CRProperty::one()))
    }

    /// Check that if crProperty uniquely defines a constructive real, then crProperty
    /// and crFactor both describe approximately the same number.
    pub fn property_correct(&self, prec: i32) -> NumResult<bool> {
        let Some(cr_property) = self.cr_property.clone() else {
            return Ok(true);
        };

        let property_cr = cr_property.cr()?;
        if let Some(property_cr) = property_cr {
            let bound = cr_property.msb_bound();
            if bound != i32::MIN
                && property_cr
                    .clone()
                    .abs()
                    .compare_to_absolute(&(ONE.clone() << bound)?, prec)?
                    == Ordering::Less
            {
                // msb_bound produced incorrect result.
                Ok(false)
            } else {
                Ok(self.cr.compare_to_absolute(&property_cr, prec)? == Ordering::Equal)
            }
        } else {
            Ok(true)
        }
    }

    pub fn definitely_algebraic(&self) -> bool {
        self.cr_property.definitely_algebraic() || self.rat.is_zero()
    }

    pub fn definitely_rational(&self) -> bool {
        self.cr_property.is_one() || self.rat.is_zero()
    }

    pub fn definitely_irrational(&self) -> bool {
        !self.cr_property.is_one()
    }

    pub fn definitely_transcendental(&self) -> bool {
        if self.definitely_rational() {
            return false;
        }

        let Some(cr_property) = self.cr_property.clone() else {
            return false;
        };

        match cr_property {
            CRProperty::Pi => true,
            CRProperty::Sqrt(_) => false,
            CRProperty::Ln(_) => {
                // arg > 1
                // Follows from Lindemann-Weierstrass theorem. If ln(r) = a, where r is rational, and a
                // algebraic, then r = e^a. But if a is nonzero algebraic, then e^a is transcendental.
                true
            }
            CRProperty::Log(_) => {
                // If this is rational, then n ln(arg) = m ln(10), n and m integers.
                // TODO: Can we do better?
                false
            }
            CRProperty::Exp(_) => {
                // arg != 0
                // Simple application of Lindemann-Weierstrass theorem.
                true
            }
            CRProperty::SinPi(_, _) | CRProperty::TanPi(_, _) => {
                // Always algebraic for rational multiples of pi.
                false
            }
            CRProperty::Asin(_) | CRProperty::Atan(_) => {
                // If asin(r) = a, r rational, a algebraic, then r = sin(a). It follows from
                // Lindemann-Weierstrass that this can happen only if a is zero, i.e. if r is zero.
                // We don't use this representation for asin(0). The atan argument is similar.
                true
            }
            CRProperty::Irrational => {
                // Not enough information to tell.
                false
            }
            _ => unreachable!(),
        }
    }

    /// Do we know that this.crFactor is an irrational nonzero multiple of u.crFactor? If this returns
    /// true, then a comparison of the two UnifiedReals cannot diverge, though we don't know of a good
    /// runtime bound. Note that if both values are really tiny, it still may be completely impractical
    /// to compare them.
    pub fn definitely_independent(&self, other: &Self) -> bool {
        // We always return false if either crFactor might be zero.
        let Some(p1) = &self.cr_property else {
            return false;
        };
        let Some(p2) = &other.cr_property else {
            return false;
        };
        if p1 == p2 {
            return false;
        }

        // Halve the number of cases. ONE < PI < SQRT < EXP < LN.
        if p1 > p2 {
            return other.definitely_independent(self);
        }

        match p1 {
            CRProperty::One => other.definitely_irrational(),
            CRProperty::Pi => {
                // It appears to be unknown whether pi is a rational multiple of an exponential or log.
                // If we were brave, we could say true, and hope for an infinite loop, which would
                // probably prove an interesting theorem. But we are not ...
                // IS_ONE case is already handled, since p1 <= p2.
                matches!(p2, CRProperty::Sqrt(_))
            }
            CRProperty::Sqrt(p1_arg) => {
                if other.definitely_transcendental() {
                    true
                } else if let CRProperty::Sqrt(p2_arg) = p2 {
                    // The argument is not necessarily minimal.
                    p1_arg.clone().irreducible_sqrt()
                        && p2_arg.clone().irreducible_sqrt()
                        && p1 != p2
                } else {
                    false
                }
            }
            CRProperty::Exp(p1_arg) => {
                if let CRProperty::Exp(p2_arg) = p2 {
                    // Lindemann-Weierstrass theorem gives us algebraic independence.
                    p1_arg != p2_arg
                } else if let CRProperty::Ln(_) = p2 {
                    // If e^a = cln(b), then e^e^a = b^c. The r.h.s is an algebraic multiple of e^0.
                    // By Lindemann-Weierstrass, this can only happen if e^a = 0, which is impossible.
                    true
                } else {
                    other.definitely_algebraic()
                }
            }
            CRProperty::Ln(p1_arg) => {
                if let CRProperty::Irrational = p2 {
                    false // Not enough information.
                } else if let CRProperty::Ln(p2_arg) = p2 {
                    // If ln(a) = cln(b), then a = b^c, a, b, and c rational, or equivalently a^c1 = b^c2,
                    // with c1 and c2 integers. C must be nonzero, since a > 1.  A necessary condition for
                    // this is that the numerator and denominator separately have to have a common integral
                    // power.
                    !have_common_power(&p1_arg.clone(), &p2_arg.clone())
                } else {
                    // Assume ln(r) = a is algebraic. Then e^a is rational. By Lindemann-Weierstrass, this
                    // implies a = 0 and r = 1. We know that the argument is not one, so any algebraic
                    // number must be linearly independent over the rationals.
                    other.definitely_algebraic()
                    // TODO: Can we do better for IS_LOG?
                }
            }
            CRProperty::Log(p1_arg) => {
                // In the irrational case, with u rational, we would have checked in the other order.
                if let CRProperty::Log(p2_arg) = p2 {
                    // We're asking if ln(a)/ln(10) = r ln(b)/ln(10), which is true iff ln(a) = r ln(b).
                    // Use the same algorithm as for IS_LN.
                    !have_common_power(&p1_arg.clone(), &p2_arg.clone())
                } else {
                    false
                }
            }
            CRProperty::SinPi(_, _) | CRProperty::TanPi(_, _) => {
                // Always algebraic. We already handled the other rational case above.
                other.definitely_transcendental()
            }
            CRProperty::Asin(_) => {
                // As we argued above, this is transcendental.
                other.definitely_algebraic()
            }
            CRProperty::Atan(_) => {
                // The case of other rational is handled above. Can we do better?
                false
            }
            CRProperty::Irrational => false,
        }
    }

    pub fn to_nice_string(&self, ang: AngleUnit, subsuperscript: bool) -> NumResult<String> {
        if self.cr_property.is_one() || self.rat.is_zero() {
            return Ok(self.rat.to_nice_string(subsuperscript));
        }

        let symbolic = self.cr_property.cr_symbolic(ang, subsuperscript);
        if let Some(symbolic) = symbolic {
            if self.rat.is_integer() {
                if self.rat.is_one() {
                    return Ok(symbolic);
                } else if self.rat == BigRational::from_i32(-1).unwrap() {
                    return Ok(format!("-{symbolic}"));
                }
                return Ok(format!("{}{}", self.rat.to_integer(), symbolic));
            }
            let bi_inverse = self.rat.clone().inv();
            if bi_inverse.is_integer() {
                let bi_inverse = bi_inverse.to_integer();
                // Use spaces to reduce ambiguity with square roots.
                return Ok(format!(
                    "{}{} / {}",
                    if bi_inverse.is_negative() { "-" } else { "" },
                    symbolic,
                    bi_inverse.abs()
                ));
            }
            return if subsuperscript {
                Ok(format!(
                    "{}{}",
                    self.rat.to_nice_string(subsuperscript),
                    symbolic
                ))
            } else {
                Ok(format!(
                    "({}){}",
                    self.rat.to_nice_string(subsuperscript),
                    symbolic
                ))
            };
        }
        if self.rat.is_one() {
            return self.cr.to_string(10, 10);
        }
        self.cr.to_string(10, 10)
    }

    pub fn exactly_displayable(&self) -> bool {
        if let Some(cr_property) = &self.cr_property {
            cr_property.determines_cr()
        } else {
            false
        }
    }

    /// Returns a truncated representation of the result.
    /// If exactlyTruncatable(), we round correctly towards zero. Otherwise the resulting digit
    /// string may occasionally be rounded up instead.
    /// Always includes a decimal point in the result.
    /// The result includes n digits to the right of the decimal point.
    ///
    /// Parameters:
    /// n: result precision, >= 0
    pub fn to_string_truncated(&self, n: u32) -> NumResult<String> {
        if self.cr_property.is_one() || self.rat.is_zero() {
            return Ok(self.rat.to_string_truncated(n));
        }

        let scaled =
            ConstructiveReal::from(BigInt::from_i32(10).unwrap().pow(n)) * self.cr_value().clone();
        let mut negative = false;
        let mut int_scaled;
        if self.exactly_truncatable() {
            int_scaled = scaled.get_appr(0)?;
            if int_scaled.is_negative() {
                negative = true;
                int_scaled = -int_scaled;
            }

            if ConstructiveReal::from(int_scaled.clone()).compare_to(&scaled.clone().abs())?
                == Ordering::Greater
            {
                int_scaled -= BigInt::one();
            }

            assert_eq!(
                ConstructiveReal::from(int_scaled.clone()).compare_to(&scaled.abs())?,
                Ordering::Less
            );
        } else {
            // Approximate case.  Exact comparisons are impossible.
            int_scaled = scaled.get_appr(-EXTRA_PREC)?;
            if int_scaled.is_negative() {
                negative = true;
                int_scaled = -int_scaled;
            }
            int_scaled >>= EXTRA_PREC;
        }

        let mut digits = int_scaled.to_string();
        let mut len = digits.len();
        if len < (n as usize) + 1 {
            digits = format!("{}{}", "0".repeat((n as usize) + 1 - len), digits).to_string();
            len = (n as usize) + 1;
        }

        Ok(format!(
            "{}{}.{}",
            if negative { "-" } else { "" },
            &digits[0..len - (n as usize)],
            &digits[len - (n as usize)..]
        ))
    }

    /// Can we compute correctly truncated approximations of this number?
    pub fn exactly_truncatable(&self) -> bool {
        // If the value is known rational, we can do exact comparisons.
        // If the value is known irrational, then we can safely compare to rational approximations;
        // equality is impossible; hence the comparison must converge.
        // The only problem cases are the ones in which we don't know.
        self.cr_property.is_one() || self.rat.is_zero() || self.definitely_irrational()
    }

    pub fn cr_value(&self) -> ConstructiveReal {
        if self.rat == BigRational::one() {
            self.cr.clone()
        } else {
            ConstructiveReal::from(self.rat.clone()) * self.cr.clone()
        }
    }

    pub fn same_cr_factor(&self, other: &Self) -> bool {
        self.cr == other.cr || {
            if let Some(cr_property) = &self.cr_property {
                cr_property.determines_cr() && self.cr_property == other.cr_property
            } else {
                false
            }
        }
    }

    /// Do both numbers have properties of the same kind describing either a constant or
    /// a strictly monotonic function?
    pub fn same_monotonic_cr_kind(&self, other: &Self) -> bool {
        if let Some(self_cr_property) = &self.cr_property
            && let Some(other_cr_property) = &other.cr_property
            && self_cr_property == other_cr_property
            && self_cr_property.determines_cr()
        {
            // All of our kinds other than IS_IRRATIONAL currently qualify.
            true
        } else {
            false
        }
    }

    /// Are this and other exactly comparable?
    pub fn is_comparable(&self, other: &Self) -> NumResult<bool> {
        // We check for ONE only to speed up the common case.
        // The use of a tolerance here means we can spuriously return false, not true.
        Ok(
            (self.same_cr_factor(other) && self.cr_property.is_nonzero())
                || self.rat.is_zero() && other.rat.is_zero()
                || (self.definitely_independent(other)
                // One of the operands also needs to be non-tiny for the comparison to be practical.
                && (self.leading_binary_zeros() < -ZERO_COMPARISON_TOLERANCE
                || other.leading_binary_zeros() < -ZERO_COMPARISON_TOLERANCE
                || self.cr_value().sign_precision(DEFAULT_INITIAL_TOLERANCE)? != Sign::NoSign  // Try cheaper test first.
                || other.cr_value().sign_precision(DEFAULT_INITIAL_TOLERANCE)? != Sign::NoSign
                || self.cr_value().sign_precision(ZERO_COMPARISON_TOLERANCE)? != Sign::NoSign
                || other.cr_value().sign_precision(ZERO_COMPARISON_TOLERANCE)? != Sign::NoSign))
                || (self.same_monotonic_cr_kind(other)
                    && (self.rat == other.rat
                        || matches!(self.cr_property, Some(CRProperty::Sqrt(_)))))
                || self.cr_value().compare_to_relative(
                    &other.cr_value(),
                    DEFAULT_RELATIVE_TOLERANCE,
                    DEFAULT_COMPARISON_TOLERANCE,
                )? != Ordering::Equal,
        )
    }

    /// Return an upper bound on the number of leading zero bits. These are the number of 0 bits to the
    /// right of the binary point and to the left of the most significant digit. Return
    /// i32::MAX if we cannot bound it based only on the rational factor and property.
    pub fn leading_binary_zeros(&self) -> i32 {
        let cr_bound = self.cr_property.clone().unwrap().msb_bound(); // lower bound on binary log.
        if cr_bound != i32::MIN {
            let whole_bits = self.rat.whole_number_bits();
            if whole_bits == i32::MIN {
                i32::MAX
            } else if whole_bits + cr_bound >= 3 {
                0
            } else {
                -(whole_bits + cr_bound) + 3
            }
        } else {
            i32::MAX
        }
    }

    /// Return Greater if this is greater than other, Less if this is less than r, or Equal if the two are known to be
    /// equal. May diverge if the two are equal and !isComparable(r).
    pub fn compare_to(&self, other: &Real) -> NumResult<Ordering> {
        fn multiply_ordering(sign: Sign, ordering: Ordering) -> Ordering {
            if sign == Sign::NoSign {
                return Ordering::Equal;
            }

            match ordering {
                Ordering::Less => match sign {
                    Sign::Minus => Ordering::Greater,
                    Sign::Plus => Ordering::Less,
                    _ => unreachable!(),
                },
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => match sign {
                    Sign::Minus => Ordering::Less,
                    Sign::Plus => Ordering::Greater,
                    _ => unreachable!(),
                },
            }
        }

        if self.definitely_zero() && other.definitely_zero() {
            return Ok(Ordering::Equal);
        }

        if self.same_cr_factor(other) {
            let sign = self.cr.sign()?; // Can diverge if crFactor == 0.
            if sign == Sign::NoSign {
                return Ok(Ordering::Equal);
            }

            let other = self.rat.cmp(&other.rat);
            return Ok(multiply_ordering(sign, other));
        }

        if self.same_monotonic_cr_kind(other) {
            if self.rat == other.rat {
                // kind cannot be IS_PI or IS_ONE, since same_cr_factor() would have been true.
                // same_monotonic_cr_kind() precludes IS_IRRATIONAL.
                // All other kinds represent monotonically increasing functions over the range we allow.
                // Just compare the arguments.
                return Ok(multiply_ordering(
                    self.rat.sign(),
                    self.cr_property
                        .clone()
                        .unwrap()
                        .get_arg()
                        .clone()
                        .unwrap()
                        .cmp(
                            &other
                                .cr_property
                                .clone()
                                .unwrap()
                                .get_arg()
                                .clone()
                                .unwrap(),
                        ),
                ));
            }
            if let Some(CRProperty::Sqrt(cr_property_arg)) = &self.cr_property {
                // Compare the squares. We promise to compare these accrurately, so we force
                // the multiplications to succeed by letting the result exceed BoundedRational
                // size bounds.
                let signum = self.rat.sign();
                let other_signum = other.rat.sign();
                if signum < other_signum {
                    return Ok(Ordering::Less);
                } else if signum > other_signum {
                    return Ok(Ordering::Greater);
                }

                let squared = self.rat.clone() * self.rat.clone() * cr_property_arg.clone();
                let other_squared = other.rat.clone() * other.rat.clone() * cr_property_arg.clone();

                return Ok(multiply_ordering(signum, squared.cmp(&other_squared)));
            }
        }
        self.cr_value().compare_to(&other.cr_value()) // Can also diverge.
    }

    /// Return Greater if this is greater than r, Less if this is less than r, Equal if the two are equal, or
    /// possibly Equal if the two are within 2^a of each other, and not comparable.
    pub fn compare_to_prec(&self, other: &Real, a: i32) -> NumResult<Ordering> {
        if self.is_comparable(other)? {
            self.compare_to(other)
        } else {
            // See if we can resolve comparison with lower precision first.
            let mut prec = DEFAULT_INITIAL_TOLERANCE;
            while prec * 2 <= a {
                let result = self
                    .cr_value()
                    .compare_to_absolute(&other.cr_value(), prec)?;
                if result != Ordering::Equal {
                    return Ok(result);
                }
                prec *= 2;
            }

            self.cr_value().compare_to_absolute(&other.cr_value(), a)
        }
    }

    pub fn sign_prec(&self, a: i32) -> NumResult<Sign> {
        Ok(match self.compare_to_prec(&ZERO, a)? {
            Ordering::Less => Sign::Minus,
            Ordering::Equal => Sign::NoSign,
            Ordering::Greater => Sign::Plus,
        })
    }

    pub fn sign(&self) -> NumResult<Sign> {
        Ok(match self.compare_to(&ZERO)? {
            Ordering::Less => Sign::Minus,
            Ordering::Equal => Sign::NoSign,
            Ordering::Greater => Sign::Plus,
        })
    }

    /// Equality comparison. May erroneously return true if values differ by less than 2^a, and
    /// !is_comparable(other).
    pub fn approx_equals(&self, other: &Real, a: i32) -> NumResult<bool> {
        Ok(if self.is_comparable(other)? {
            if self.definitely_independent(other) && (self.rat.is_zero() || other.rat.is_zero()) {
                // No need to actually evaluate, though we don't know which is larger.
                false
            } else {
                self.compare_to(other)? == Ordering::Equal
            }
        } else {
            self.cr_value().compare_to_absolute(&other.cr_value(), a)? == Ordering::Equal
        })
    }

    /// Returns true if values are definitely known to be equal, false in all other cases.
    pub fn definitely_equals(&self, other: &Real) -> NumResult<bool> {
        Ok(self.is_comparable(other)? && self.compare_to(other)? == Ordering::Equal)
    }

    pub fn definitely_not_equals(&self, other: &Real) -> bool {
        if self.rat.is_zero() {
            return other.cr_property.is_nonzero() && !other.rat.is_zero();
        }

        if other.rat.is_zero() {
            return self.cr_property.is_nonzero() && !self.rat.is_zero();
        }

        if self.definitely_independent(other) {
            return !self.rat.is_zero() || !other.rat.is_zero();
        } else if self.same_cr_factor(other) && self.cr_property.is_nonzero() {
            return self.rat.clone() != other.rat;
        }

        false
    }

    pub fn definitely_zero(&self) -> bool {
        // If crFactor were known to be zero, we would have used a different representation.
        self.rat.is_zero()
    }

    pub fn definitely_one(&self) -> bool {
        self.cr_property.is_one() && self.rat == BigRational::one()
    }

    /// Can this number be determined to be definitely nonzero without performing approximate
    /// evaluation?
    pub fn definitely_nonzero(&self) -> bool {
        self.cr_property.is_nonzero() && self.rat.sign() != Sign::NoSign
    }

    /// Returns a suitable representation of ln(arg) or log(arg). arg is positive and not one. kind is
    /// IS_LN or IS_LOG.
    pub fn log_rep(kind: CRProperty, arg: BigRational) -> NumResult<Self> {
        if !matches!(kind, CRProperty::Ln(_) | CRProperty::Log(_)) {
            panic!("log_rep called with invalid kind");
        }

        if arg < BigRational::one() {
            // Convert to an argument larger than one. Normalizing arguments in this way increases the
            // chance of repeat occurrences of the same argument, and makes them cleaner to display.

            return Ok(-Self::log_rep(kind, arg.inv())?);
        }

        if arg.is_integer()
            && let Some(small_power_log) = Self::lg_small_power(kind.clone(), arg.to_integer())?
        {
            return Ok(small_power_log);
        }

        Ok(if arg.bit_length() > LOG_ARG_BITS {
            if matches!(kind, CRProperty::Ln(_)) {
                Self::new_from_cr(ConstructiveReal::from(arg).ln()?)
            } else {
                Self::new_from_cr(ConstructiveReal::from(arg).ln()? / LN_10.clone())
            }
        } else {
            Self::new_from_rat_property(
                BigRational::one(),
                CRProperty::new(match kind {
                    CRProperty::Ln(_) => CRProperty::Ln(arg),
                    CRProperty::Log(_) => CRProperty::Log(arg),
                    _ => unreachable!(),
                }),
            )
        })
    }

    fn lg_small_power(kind: CRProperty, arg: BigInt) -> NumResult<Option<Self>> {
        for m in SMALL_NON_POWERS {
            let int_log = get_int_log(arg.clone(), m);
            let new_cr_value;
            if int_log != 0 {
                if matches!(kind, CRProperty::Log(_)) {
                    if m == 10 {
                        return Ok(Some(Real::new_from_rational(
                            BigRational::from_i64(int_log).unwrap(),
                        )));
                    }
                    new_cr_value = ConstructiveReal::from(m).ln()? / LN_10.clone();
                } else {
                    new_cr_value = ConstructiveReal::from(m).ln()?;
                }

                return Ok(Some(Real::new(
                    BigRational::from_i64(int_log).unwrap(),
                    new_cr_value,
                    Some(CRProperty::new(match kind {
                        CRProperty::Ln(_) => CRProperty::Ln(BigRational::from_i32(m).unwrap()),
                        CRProperty::Log(_) => CRProperty::Log(BigRational::from_i32(m).unwrap()),
                        _ => unreachable!(),
                    })),
                )));
            }
        }

        Ok(None)
    }

    /// Return sqrt(x*y) as a Real
    fn multiply_sqrts(x: BigRational, y: BigRational) -> Self {
        if x == y {
            Real::new_from_rational(x)
        } else {
            let product = x * y;
            if product.is_zero() {
                ZERO.clone()
            } else {
                let decomposed_product = product.extract_square_reduced();
                Real::new(
                    decomposed_product.0,
                    ConstructiveReal::from(decomposed_product.1.clone()).sqrt(),
                    Some(CRProperty::new(CRProperty::Sqrt(decomposed_product.1))),
                )
            }
        }
    }

    fn sqrt(self) -> NumResult<Self> {
        if self.sign_prec(DEFAULT_COMPARISON_TOLERANCE)? == Sign::Minus {
            return Err(DomainViolation(NthRoot(2.)));
        }
        if self.definitely_zero() {
            return Ok(ZERO.clone());
        }

        let mut new_cr_property = None;
        if self.cr_property.is_one() && self.rat.extract_square_will_succeed() {
            // Avoid generating IS_SQRT property for rational values.
            let decomposed_product = self.rat.extract_square_reduced();
            if decomposed_product.1 == BigRational::one() {
                new_cr_property = Some(CRProperty::one());
            } else {
                new_cr_property = Some(CRProperty::new(CRProperty::Sqrt(
                    decomposed_product.1.clone(),
                )));
            }
            return Ok(Real::new(
                decomposed_product.0,
                ConstructiveReal::from(decomposed_product.1).sqrt(),
                new_cr_property,
            ));
        } // else don't track; we don't know if it's rational.

        // If this is exp(a), result is exp(a/2). Track that.
        if let Some(CRProperty::Exp(exp_arg)) = &self.cr_property {
            let new_arg = exp_arg / BigRational::from_i32(2).unwrap();
            if !new_arg.too_big() {
                new_cr_property = Some(CRProperty::new(CRProperty::Exp(new_arg)));
            }
        }

        Ok(Real::new_from_cr_property(
            self.cr_value().sqrt(),
            new_cr_property,
        ))
    }

    /// Return (this mod 2pi)/(pi/6) as a BigInteger, or None if that isn't easily possible.
    fn get_pi_twelfths(&self) -> Option<BigInt> {
        if self.definitely_zero() {
            Some(BigInt::zero())
        } else if self.cr_property.is_pi() {
            let quotient =
                (self.rat.clone() * BigRational::from_i32(12).unwrap()).try_as_integer()?;
            Some(quotient % BigInt::from_i32(24).unwrap())
        } else {
            None
        }
    }

    pub fn sin(&self) -> NumResult<Self> {
        if let Some(pi_twelfths) = self.get_pi_twelfths()
            && let Some(result) = sin_pi_twelfths(pi_twelfths.to_i32().unwrap())
        {
            return Ok(result);
        };

        if self.cr_property.is_pi()
            && let Some(new_cr_property) = CRProperty::new_sin_pi(self.rat.clone())
        {
            let CRProperty::SinPi(_, neg) = new_cr_property else {
                panic!("new_sin_pi returned not CRProperty::SinPi")
            };
            return Ok(Real::new_from_rat_property(
                if neg {
                    -BigRational::one()
                } else {
                    BigRational::one()
                },
                new_cr_property,
            ));
        }

        if let Some(CRProperty::Asin(arg)) = &self.cr_property
            && self.rat == BigRational::one()
        {
            return Ok(Real::new_from_rational(arg.clone()));
        }

        Ok(Real::new_from_cr_property(
            self.cr_value().sin()?,
            if self.definitely_algebraic() && self.definitely_nonzero() {
                Some(CRProperty::Irrational)
            } else {
                None
            },
        ))
    }

    /// Return a copy of the argument that is at least marked is irrational.
    fn tag_irrational(&self) -> Self {
        if self.cr_property.is_none() {
            Real::new(
                self.rat.clone(),
                self.cr.clone(),
                Some(CRProperty::irrational()),
            )
        } else {
            self.clone()
        }
    }

    pub fn cos(&self) -> NumResult<Self> {
        if self.definitely_algebraic() && self.definitely_nonzero() {
            // We know from Lindemann-Weierstrass that the result is transcendental, and therefore
            // irrational.
            Ok((self.clone() + PI_OVER_2.clone())?.sin()?.tag_irrational())
        } else {
            (self.clone() + PI_OVER_2.clone())?.sin()
        }
    }

    pub fn tan(&self) -> NumResult<Self> {
        if let Some(pi_twelfths) = self.get_pi_twelfths() {
            let i = pi_twelfths.to_i32().unwrap();
            if i == 6 || i == 18 {
                return Err(DomainViolation(TanDomainViolation));
            }

            if let Some(top) = sin_pi_twelfths(i)
                && let Some(bottom) = cos_pi_twelfths(i)
            {
                return top / bottom;
            }
        }

        if let Some(CRProperty::Pi) = self.cr_property
            && let Some(new_cr_property) = CRProperty::new_tan_pi(self.rat.clone())
        {
            let CRProperty::TanPi(_, neg) = new_cr_property else {
                panic!("new_tan_pi returned not CRProperty::TanPi")
            };
            return Ok(Real::new_from_rat_property(
                if neg {
                    -BigRational::one()
                } else {
                    BigRational::one()
                },
                new_cr_property,
            ));
        }

        if let Some(CRProperty::Atan(atan_arg)) = &self.cr_property
            && self.rat == BigRational::one()
        {
            return Ok(Real::new_from_rational(atan_arg.clone()));
        }

        todo!()

        // Ok(Real::new_from_cr_property(
        //     self.cr_value().tan()?,
        //     if self.definitely_algebraic() && self.definitely_nonzero() {
        //         Some(CRProperty::Irrational)
        //     } else {
        //         None
        //     },
        // ))
    }

    pub fn check_asin_domain(&self) -> NumResult<()> {
        if self.is_comparable(&constants::ONE)?
            && self.compare_to(&constants::ONE)? == Ordering::Greater
            && self.compare_to(&constants::M_ONE)? == Ordering::Less
        {
            Err(DomainViolation(AsinDomainViolation))
        } else {
            Ok(())
        }
    }

    /// Return asin(n/2). n is between -2 and 2.
    pub fn asin_halves(n: i32) -> Real {
        if n < 0 {
            -Self::asin_halves(-n)
        } else {
            match n {
                0 => ZERO.clone(),
                1 => Real::new_from_rat_cr(BigRational::new(1.into(), 6.into()), PI.clone()),
                2 => Real::new_from_rat_cr(BigRational::new(1.into(), 2.into()), PI.clone()),
                _ => {
                    panic!("asin_halves called with invalid argument");
                }
            }
        }
    }

    pub fn asin(&self) -> NumResult<Self> {
        self.check_asin_domain()?;
        if let Ok(halves) = BigInt::try_from(self.clone() * TWO.clone()) {
            let n = halves.to_i32().unwrap();
            return Ok(Self::asin_halves(n));
        }

        if self.compare_to_prec(&ZERO, -10) == Ok(Ordering::Less) {
            return Ok(self.clone().neg().asin()?.neg());
        }

        if self.definitely_equals(&HALF_SQRT_2)? {
            return Ok(Real::new_from_rat_cr(
                BigRational::new(1.into(), 4.into()),
                PI.clone(),
            ));
        }

        if self.definitely_equals(&HALF_SQRT_3)? {
            return Ok(Real::new_from_rat_cr(
                BigRational::new(1.into(), 3.into()),
                PI.clone(),
            ));
        }

        if let Some(CRProperty::SinPi(arg, _)) = &self.cr_property {
            if self.rat == BigRational::one() {
                return Ok(Real::new_from_rat_cr(arg.clone(), PI.clone()));
            }

            if self.rat == BigRational::from_i32(-1).unwrap() {
                return Ok(Real::new(-arg.clone(), PI.clone(), Some(CRProperty::Pi)));
            }
        }

        if let Some(CRProperty::One) = self.cr_property {
            assert!(self.rat.is_positive());

            return Ok(Real::new_from_property(CRProperty::new(CRProperty::Asin(
                self.rat.clone(),
            ))));
        }

        todo!()
        // Real::new_from_cr(self.cr_value().asin())
    }

    pub fn acos(&self) -> NumResult<Self> {
        PI_OVER_2.clone() - self.asin()?
    }

    pub fn atan(&self) -> NumResult<Self> {
        if self.compare_to_prec(&ZERO, -10)? == Ordering::Less {
            return Ok(self.clone().neg().atan()?.neg());
        }

        if let Ok(as_bi) = BigInt::try_from(self.clone())
            && as_bi <= BigInt::one()
        {
            let as_int = as_bi.to_i32().unwrap();
            // These seem to be all rational cases:
            return Ok(match as_int {
                0 => ZERO.clone(),
                1 => PI_OVER_4.clone(),
                _ => unreachable!(),
            });
        }

        if self.definitely_equals(&THIRD_SQRT_3)? {
            return Ok(PI_OVER_6.clone());
        }
        if self.definitely_equals(&SQRT_3)? {
            return Ok(PI_OVER_3.clone());
        }

        if let Some(CRProperty::TanPi(tan_pi_arg, _)) = &self.cr_property {
            if self.rat == BigRational::one() {
                return Ok(Real::new_from_rat_cr(tan_pi_arg.clone(), PI.clone()));
            }

            if self.rat == BigRational::from_i32(-1).unwrap() {
                return Ok(Real::new_from_rat_cr(-tan_pi_arg.clone(), PI.clone()));
            }
        }

        if let Some(CRProperty::One) = &self.cr_property {
            assert!(self.rat.is_positive());
            return Ok(Real::new_from_property(CRProperty::new(CRProperty::Atan(
                self.rat.clone(),
            ))));
        }

        todo!()
        // Ok(Real::new_from_cr(self.cr_value().atan()))
    }

    /// Compute an integral power of a constructive real, using the standard recursive algorithm. exp
    /// is known to be positive.
    fn recursive_pow(base: ConstructiveReal, exp: BigInt) -> NumResult<ConstructiveReal> {
        if exp == BigInt::one() {
            return Ok(base);
        }

        if exp.bit(0) {
            return Ok(base.clone() * Self::recursive_pow(base, exp - BigInt::one())?);
        }

        let tmp = Self::recursive_pow(base, exp >> 1)?;
        tmp.cancellation_token.stop_if_cancelled()?;
        Ok(tmp.clone() * tmp)
    }

    /// Compute an integral power of a constructive real, using the exp function when we safely can.
    /// Use recursivePow when we can't. exp is known to be nozero.
    fn exp_ln_pow(&self, exp: &BigInt) -> NumResult<Real> {
        let sign = self.sign_prec(DEFAULT_COMPARISON_TOLERANCE)?;

        match sign {
            Sign::Plus => {
                // Safe to take the log. This avoids deep recursion for huge exponents, which
                // may actually make sense here.
                Ok(Real::new_from_cr(
                    (self.cr_value().ln()? * ConstructiveReal::from(exp.clone())).exp()?,
                ))
            }
            Sign::Minus => {
                let mut result =
                    (self.cr_value().neg().ln()? * ConstructiveReal::from(exp.clone())).exp()?;
                if exp.bit(0) {
                    result = -result;
                }
                Ok(Real::new_from_cr(result))
            }
            Sign::NoSign => {
                // Base of unknown sign with integer exponent. Use a recursive computation.
                // (Another possible option would be to use the absolute value of the base, and then
                // adjust the sign at the end.  But that would have to be done in the CR
                // implementation.)
                if exp.is_negative() {
                    // This may be very expensive if exp.negate() is large.
                    Ok(Real::new_from_cr(
                        Self::recursive_pow(self.cr_value(), -exp)?.inverse(),
                    ))
                } else {
                    Ok(Real::new_from_cr(Self::recursive_pow(
                        self.cr_value(),
                        exp.clone(),
                    )?))
                }
            }
        }
    }

    /// Compute an integral power of this. This recurses roughly as deeply as the number of bits in the
    /// exponent, and can, in ridiculous cases, result in a stack overflow.
    fn pow_int(&self, exp: &BigInt) -> NumResult<Self> {
        if exp.is_one() {
            return Ok(self.clone());
        }

        let exp_sign = exp.sign();
        if exp_sign == Sign::NoSign {
            // The following check may diverge, causing us to time out. This only happens
            // if we try to raise something that is zero, but not obviously so, to the
            // zeroth power.
            if self.sign()? != Sign::NoSign {
                return Ok(constants::ONE.clone());
            }
            // Base is known to be exactly zero.
            return Err(DomainViolation(OrdinalDomainViolation(ZeroBaseZeroOrder)));
        }

        if self.definitely_zero() && exp_sign == Sign::Minus {
            return Err(DomainViolation(OrdinalDomainViolation(
                ZeroBaseNegativeOrder,
            )));
        }
        let abs_exp = exp.abs();
        if let Some(CRProperty::One) = &self.cr_property {
            let result_len = exp.to_f64().unwrap() * self.rat.appr_log_2_abs();
            // Both multiplicands may be negative. That still implies a huge answer.
            if result_len > BIT_LIMIT as f64 {
                return Err(Overflow);
            }
            if abs_exp <= HARD_RECURSIVE_POW_LIMIT.clone() {
                // FIXME: Unwrapping here could be dangerous
                let rat_pow = self.rat.pow(exp.to_i32().unwrap());
                // We count on this to fail, e.g. for very large exponents, when it would
                // otherwise be too expensive.
                if !rat_pow.too_big() {
                    return Ok(Real::new_from_rational(rat_pow));
                }
            }
        }

        if abs_exp > RECURSIVE_POW_LIMIT.clone() {
            return self.exp_ln_pow(exp);
        }

        if let Some(CRProperty::Sqrt(square)) = &self.cr_property {
            // Compute powers as UnifiedReals, so we get the limit checking above.
            let result_factor1 = Real::new_from_rational(self.rat.clone()).pow_int(exp)?;
            let square_as_ur = Real::new_from_rational(square.clone());
            let result_factor2 = square_as_ur.pow_int(&(exp.clone() >> 1))?.clone();
            let product = result_factor1 * result_factor2;
            return if exp & BigInt::one() == BigInt::one() {
                // Odd power: Multiply by remaining square root.
                Ok(product * square_as_ur.sqrt()?)
            } else {
                Ok(product)
            };
        }
        self.exp_ln_pow(exp)
    }

    /// Return this ^ expon. This is really only well-defined for a positive base, particularly since
    /// 0^x is not continuous at zero. (0^0 = 1 (as is epsilon^0), but 0^epsilon is 0. We nonetheless
    /// try to do reasonable things at zero, when we recognize that case.
    pub fn pow(&self, expon: Self) -> NumResult<Self> {
        match &self.cr_property {
            Some(CRProperty::Exp(arg)) if arg == &BigRational::one() => {
                return if self.rat == BigRational::one() {
                    expon.exp()
                } else {
                    // (<ratFactor>e)^<expon> = <ratFactor>^<expon> * e^<expon>
                    let rat_part = Real::new_from_rational(self.rat.clone()).pow(expon.clone())?;
                    Ok(expon.exp()? * rat_part)
                };
            }
            Some(CRProperty::One) if self.rat == BigRational::from_i32(10).unwrap() => {
                if let Some(CRProperty::Log(expon_log_arg)) = expon.cr_property {
                    // 10^(r * log(expon_log_arg)) = expon_log_arg^r
                    return Real::new_from_rational(expon_log_arg)
                        .pow(Real::new_from_rational(expon.rat));
                }
            }
            _ => {}
        }

        let sign = self.sign_prec(DEFAULT_COMPARISON_TOLERANCE)?;
        let mut known_irrational = false;
        if let Ok(exp_as_br) = BigRational::try_from(expon.clone()) {
            if exp_as_br.denom().is_one() {
                return self.pow_int(exp_as_br.numer());
            }
            // Check for the case in which both arguments are rational, and there is an
            // exact rational answer.
            // We explicitly avoid returning a result for a negative base here,
            // even when that would make sense, as in (-8)^(1/3).
            // This is probably wrong if we're computing cube roots.
            // But note that we could never return a meaningful result for
            // (-8)^<1/3 computed so we can't recognize it as such>.
            if sign != Sign::Minus
                && let Some(CRProperty::One) = self.cr_property
                && exp_as_br.denom().bits() <= 30
            {
                let exp_den = exp_as_br.denom().to_i32().unwrap(); // Doesn't lose information.
                // Don't just use BigRational.pow(), since that would bypass above checks.
                let rt = self.rat.nth_root(exp_den)?;
                if !rt.too_big() {
                    return Real::new_from_rational(rt).pow_int(exp_as_br.numer());
                } else {
                    // We know that the root is irrational. Raising it to a power relatively prime to exp_den
                    // is not going to change that.
                    known_irrational = true;
                }
            }
            // Explicitly check for the square root case, in case the result is representable
            // as an integer multiple of a small square root.
            if exp_as_br.denom() == &BigInt::from_i32(2).unwrap() {
                return self.pow_int(exp_as_br.numer())?.sqrt();
            }
        }
        // If the exponent were known zero, we would have handled it above.
        if sign == Sign::NoSign && self.definitely_zero() {
            // Compute the exponent sign, at the risk of divergence. The result depends on it.
            let expon_sign = expon.sign()?;
            return if expon_sign == Sign::Plus {
                Ok(ZERO.clone())
            } else if expon_sign == Sign::Minus {
                Err(DomainViolation(OrdinalDomainViolation(
                    ZeroBaseNegativeOrder,
                )))
            } else {
                // Unclear we can get here.
                Err(DomainViolation(OrdinalDomainViolation(ZeroBaseZeroOrder)))
            };
        }
        if sign == Sign::Minus {
            return Err(DomainViolation(OrdinalDomainViolation(
                NegativeBaseNonIntegerOrder,
            )));
        }
        if known_irrational {
            Ok(Real::new_from_cr_property(
                (self.cr_value().ln()? * expon.cr_value()).exp()?,
                Some(CRProperty::irrational()),
            ))
        } else {
            Ok(Real::new_from_cr(
                (self.cr_value().ln()? * expon.cr_value()).exp()?,
            ))
        }
    }

    pub fn exp(&self) -> NumResult<Self> {
        if self.definitely_equals(&ZERO)? {
            return Ok(constants::ONE.clone());
        }
        if self.definitely_equals(&constants::ONE)? {
            // Avoid redundant computations, and ensure we recognize all instances as equal.
            return Ok(E.clone());
        }

        if let Some(CRProperty::Ln(lnArg)) = &self.cr_property {
            let mut need_sqrt = false;
            let mut rat_exponent = self.rat.clone();
            if rat_exponent.try_as_integer().is_some() {
                // check for multiple of one half.
                need_sqrt = true;
                rat_exponent *= BigRational::from_i32(2).unwrap();
            }

            let n_rat_factor = lnArg.pow(rat_exponent.to_i32().unwrap());
            if !n_rat_factor.too_big() {
                let result = Real::new_from_rational(n_rat_factor);
                return if need_sqrt { result.sqrt() } else { Ok(result) };
            }
        }

        if self.compare_to_prec(&BIT_LIMIT_AS_REAL, 0)? == Ordering::Greater {
            return Err(Overflow);
        }

        let mut new_cr_property = None;
        if let Some(CRProperty::One) = self.cr_property {
            new_cr_property = Some(CRProperty::new(CRProperty::Exp(self.rat.clone())));
        }
        Ok(Real::new_from_cr_property(
            self.cr_value().exp()?,
            new_cr_property,
        ))
    }

    /// Absolute Value
    pub fn abs(&self) -> NumResult<Self> {
        if self.is_comparable(&ZERO)? {
            if self.sign()? == Sign::Minus {
                Ok(self.clone().neg())
            } else {
                Ok(self.clone())
            }
        } else {
            Ok(Real::new_from_cr_property(
                self.cr_value().abs(),
                if self.cr_property.is_unknown_irrational() {
                    Some(CRProperty::irrational())
                } else {
                    None
                },
            ))
        }
    }

    /// Factorial function. Fails if argument is clearly not an integer. May round to nearest integer
    /// if value is close.
    pub fn fact(&self) -> NumResult<Self> {
        let as_bi = match BigInt::try_from(self.clone()) {
            Ok(as_bi) => as_bi,
            _ => {
                let as_bi = self.cr_value().get_appr(0)?; // Correct if it was an integer.
                if !self.approx_equals(
                    &Real::new_from_rational(as_bi.clone().into()),
                    DEFAULT_COMPARISON_TOLERANCE,
                )? {
                    return Err(DomainViolation(FactorialDomainViolation(NonIntegerBase)));
                } else {
                    as_bi
                }
            }
        };

        if as_bi.is_negative() {
            return Err(DomainViolation(FactorialDomainViolation(NegativeBase)));
        }
        if as_bi.bits() > 18 {
            // Several million digits. Will fail.  LongValue() may not work. Punt now.
            return Err(Overflow);
        }

        // TODO: Pass in cancellation token
        let bi_result = gen_factorial(as_bi.to_i64().unwrap(), 1, CancellationToken::new(false))?;
        let n_rat_factor = BigRational::from(bi_result);
        Ok(Real::new_from_rational(n_rat_factor))
    }

    /// Return the number of decimal digits to the right of the decimal point required to represent
    /// the argument exactly. Return usize::MAX if that's not possible. Never returns a value
    /// less than zero, even if r is a power of ten.
    pub fn digits_required(&self) -> usize {
        if self.cr_property.is_one() || self.rat.is_zero() {
            self.rat.digits_required()
        } else {
            usize::MAX
        }
    }

    /// Is the number of bits to the left of the decimal point greater than bound? The result is
    /// inexact: We roughly approximate the whole number bits. bound is non-negative.
    pub fn approx_whole_number_bits_greater_than(&self, bound: i32) -> NumResult<bool> {
        assert!(bound >= 0);
        let cr_bound = self.cr_property.clone().unwrap().msb_bound();
        let rat_bits = self.rat.whole_number_bits();
        if cr_bound != i32::MIN && rat_bits != i32::MIN {
            Ok(rat_bits + cr_bound > bound)
        } else {
            Ok(self.cr_value().get_appr(bound - 2)?.bits() > 2)
        }
    }
}

impl From<i32> for Real {
    fn from(i: i32) -> Self {
        Self::new_from_rational(BigRational::from_i32(i).unwrap())
    }
}

impl From<i64> for Real {
    fn from(i: i64) -> Self {
        Self::new_from_rational(BigRational::from_i64(i).unwrap())
    }
}

impl From<BigInt> for Real {
    fn from(i: BigInt) -> Self {
        Self::new_from_rational(BigRational::from(i))
    }
}

impl From<f64> for Real {
    fn from(value: f64) -> Self {
        if value == 0. || value == 1. {
            Real::from(value as i64)
        } else {
            Self::new_from_rational(BigRational::from_f64(value).unwrap())
        }
    }
}

impl TryFrom<Real> for f64 {
    type Error = NumError;

    /// Return a double approximation. Rational arguments are currently rounded to nearest, with ties
    /// away from zero. TODO: Improve rounding.
    fn try_from(value: Real) -> Result<Self, Self::Error> {
        if value.cr_property.is_one() {
            value
                .rat
                .to_f64()
                .map(Ok)
                .unwrap_or(Err(NumError::InternalError(UnconstructableFloat))) // Hopefully correctly rounded
        } else {
            value.cr_value().into() // Approximately correctly rounded
        }
    }
}

impl TryFrom<Real> for BigRational {
    type Error = ();

    /// Return equivalent BigRational, if known to exist, Err otherwise
    fn try_from(value: Real) -> Result<Self, Self::Error> {
        if value.cr_property.is_one() || value.rat.is_zero() {
            Ok(value.rat)
        } else {
            Err(())
        }
    }
}

impl TryFrom<Real> for BigInt {
    type Error = ();

    /// Returns equivalent BigInt result if it exists, null if not
    fn try_from(value: Real) -> Result<Self, Self::Error> {
        let r = BigRational::try_from(value)?;
        if r.is_integer() {
            Ok(r.to_integer())
        } else {
            Err(())
        }
    }
}

impl Debug for Real {
    /// Convert to String reflecting raw representation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}*{}", self.rat, self.cr)
    }
}

impl Display for Real {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.to_nice_string(AngleUnit::Radians, false)
                .unwrap_or("Error".to_string())
        )
    }
}

impl Add for Real {
    type Output = NumResult<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.same_cr_factor(&rhs) {
            let n_rat_factor = self.rat + rhs.rat;
            return Ok(Self::new(n_rat_factor, self.cr, self.cr_property));
        }

        if self.definitely_zero() {
            // Avoid creating new crFactor, even if they don't currently match.
            return Ok(rhs);
        }
        if rhs.definitely_zero() {
            return Ok(self);
        }

        // Consider "simplifying" sums of logs.
        if let Some(self_cr_property) = &self.cr_property
            && let Some(rhs_cr_property) = &rhs.cr_property
            && self_cr_property == rhs_cr_property
            && matches!(self_cr_property, CRProperty::Ln(_) | CRProperty::Log(_))
        {
            // a ln(b) + c ln(d) = ln(b^a * d^c)
            // a log(b) + c log(d) = log(b^a * d^c)
            // If the resulting ln argument is reasonably compact, compute the sum as the right side
            // instead, since that preserves the symbolic representation.

            if let Some(rat_as_int) = self.rat.try_as_integer()
                && let Some(u_rat_as_int) = rhs.rat.try_as_integer()
            {
                let rat_as_double = rat_as_int.to_f64().unwrap();
                let u_rat_as_double = u_rat_as_int.to_f64().unwrap();

                // Estimate size of resulting argument.
                let estimated_size = rat_as_double.abs()
                    * (self_cr_property.get_arg().clone().unwrap().bit_length() as f64)
                    + u_rat_as_double.abs()
                        * (rhs_cr_property.get_arg().clone().unwrap().bit_length() as f64);
                if estimated_size <= LOG_ARG_CANDIDATE_BITS {
                    let term1 = self_cr_property
                        .clone()
                        .get_arg()
                        .unwrap()
                        .pow(rat_as_int.to_i32().unwrap());
                    let term2 = rhs_cr_property
                        .clone()
                        .get_arg()
                        .unwrap()
                        .pow(u_rat_as_int.to_i32().unwrap());
                    let new_arg = term1 * term2;
                    return Self::log_rep(self_cr_property.clone(), new_arg);
                }
            }
        }

        // Since we got here, neither ratFactor is zero.
        // We can still conclude that the result is irrational, so long as the two arguments
        // are independent. But it can be counter-productive to track this if the arguments
        // are of greatly differing magnitude. We know that 1 + e^(-e^10000) is irrational,
        // but we still don't want to evaluate it sufficiently to distinguish it from 1.
        // Thus we want to treat 1 + e^(-e^10000) as not comparable to rationals.
        // We in fact don't track this if either argument might be ridiculously small, where
        // ridiculously small is < 10^-1000, and thus also way outside of IEEE exponent range.
        let result_prop = if self.definitely_independent(&rhs)
            && self.leading_binary_zeros() < -DEFAULT_COMPARISON_TOLERANCE
            && rhs.leading_binary_zeros() < -DEFAULT_COMPARISON_TOLERANCE
        {
            Some(CRProperty::irrational())
        } else {
            None
        };

        Ok(Real::new_from_cr_property(
            self.cr_value().add(rhs.cr_value()),
            result_prop,
        ))
    }
}

impl Neg for Real {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.rat, self.cr, self.cr_property)
    }
}

impl Sub for Real {
    type Output = NumResult<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for Real {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // Preserve a preexisting crFactor when we can.
        if self.cr_property.is_one() {
            return Real::new(self.rat * rhs.rat, rhs.cr, rhs.cr_property);
        }

        if rhs.cr_property.is_one() {
            return Real::new(self.rat * rhs.rat, self.cr, self.cr_property);
        }

        if self.definitely_zero() || rhs.definitely_zero() {
            return ZERO.clone();
        }

        let mut result_prop = None; // Property for product of crFactors.
        let n_rat_factor = self.rat.clone() * rhs.rat.clone();

        if let Some(self_cr_property) = &self.cr_property
            && let Some(rhs_cr_property) = &rhs.cr_property
        {
            if let CRProperty::Sqrt(self_cr_property_arg) = self_cr_property
                && let CRProperty::Sqrt(rhs_cr_property_arg) = rhs_cr_property
            {
                let cr_part =
                    Self::multiply_sqrts(self_cr_property_arg.clone(), rhs_cr_property_arg.clone());
                let rat_result = n_rat_factor * cr_part.rat;
                return Real::new(rat_result, cr_part.cr, cr_part.cr_property);
            }

            if let CRProperty::Exp(self_cr_property_arg) = self_cr_property
                && let CRProperty::Exp(rhs_cr_property_arg) = rhs_cr_property
            {
                // exp(a) * exp(b) is exp(a + b) .
                let sum = self_cr_property_arg + rhs_cr_property_arg;
                // we use this only for the property, since crFactors may already have been evaluated.
                result_prop = Some(CRProperty::new(CRProperty::Exp(sum)));
            }
        }

        // Probably a bit cheaper to multiply component-wise.
        // TODO: We should often be able to determine that the result is irrational.
        // But definitelyIndependent is not the right criterion. Consider e and e^-1.
        if n_rat_factor.too_big() {
            Real::new_from_cr(self.cr_value() * rhs.cr_value())
        } else {
            Real::new(n_rat_factor, self.cr * rhs.cr, result_prop)
        }
    }
}

impl Inv for Real {
    type Output = NumResult<Self>;

    fn inv(self) -> Self::Output {
        if self.definitely_zero() {
            return Err(DomainViolation(DivisionByZero));
        }

        if self.cr_property.is_one() {
            return Ok(Real::new_from_rational(self.rat.inv()));
        }

        if let Some(CRProperty::Sqrt(square)) = &self.cr_property
            && let Some(square) = square.try_as_integer()
        {
            // Prefer square roots of integers. 1/sqrt(n) = sqrt(n)/n
            let n_rat_factor = (self.rat.clone() * square).inv();
            if !n_rat_factor.too_big() {
                return Ok(Real::new(n_rat_factor, self.cr, self.cr_property));
            }
        }

        let mut new_property = None;
        if let Some(CRProperty::Exp(cr_property)) = &self.cr_property {
            new_property = Some(CRProperty::new(CRProperty::Exp(-cr_property.clone())));
        } else if self.definitely_irrational() {
            new_property = Some(CRProperty::irrational());
        }

        Ok(Real::new(self.rat.inv(), self.cr.inverse(), new_property))
    }
}

impl Div for Real {
    type Output = NumResult<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.same_cr_factor(&rhs.clone()) {
            if rhs.definitely_zero() {
                return Err(DomainViolation(DivisionByZero));
            }

            let n_rat_factor = self.rat.clone() / rhs.rat.clone();
            if !n_rat_factor.too_big() {
                return Ok(Real::new_from_rational(n_rat_factor));
            }
        }

        // Try to reduce ln(x)/ln(10) to log(x) to keep symbolic representation.
        if let Some(CRProperty::Ln(ln_arg)) = &self.cr_property
            && let Some(CRProperty::Ln(u_ln_arg)) = &rhs.cr_property
            && u_ln_arg == &BigRational::from_i32(10).unwrap()
        {
            let rat_quotient = self.rat.clone() / rhs.rat.clone();
            if !rat_quotient.too_big() {
                return Ok(Real::new_from_rat_property(
                    rat_quotient,
                    CRProperty::new(CRProperty::Log(ln_arg.clone())),
                ));
            }
        }

        Ok(self * rhs.inv()?)
    }
}

/// Return a rational r != 0, such that a = b^r, or None if we didn't find one. Effectively this
/// tests whether a and b have a common integral power and returns the two exponents as a rational.
/// A and b are presumed to be positive. Note that if a = b = 1, we return 1, but any rational
/// would do. We do not try very hard if any of the numbers involved are large.
pub fn common_power(a: &BigInt, b: &BigInt) -> Option<BigRational> {
    let compare_result = a.cmp(b);
    if compare_result == Ordering::Equal {
        Some(BigRational::one())
    } else if compare_result == Ordering::Less {
        Some(common_power(b, a)?.inv())
    } else if a.is_one() || b.is_one() {
        None
    } else if a.bits() > COMMON_POWER_LENGTH_LIMIT {
        // punt
        None
    } else {
        // We use a modified version of the Euclidean GCD algorithm, repeatedly dividing the larger
        // number by the smaller. If a = b^r, then (a/b) = b^(r-1).
        let (div, rem) = a.div_mod_floor(b);
        if rem.is_zero() {
            // If they're not divisible, there must be two primes, such that a is divisible by a larger
            // power of one than b and vice-versa. That makes it impossible that a^n = b^m, m and n
            // integers. thus we know r doesn't exist.
            Some(BigRational::from(div))
        } else {
            Some(common_power(&div, b)? + BigRational::one())
        }
    }
}

/// Do a and b have a common power? Considers negative exponents. Both a and b must be positive.
pub fn have_common_power(a: &BigRational, b: &BigRational) -> bool {
    // First consider the case in which one numerator and/or denominator pair consists
    // entirely of ones. This is special because commonPower() is not uniquely determined.
    if a.denom().is_one() {
        if b.denom().is_one() {
            return common_power(a.numer(), b.numer()).is_some();
        } else if b.numer().is_one() {
            return common_power(a.denom(), b.denom()).is_some();
        }
    } else if a.numer().is_one() {
        if b.numer().is_one() {
            return common_power(a.denom(), b.denom()).is_some();
        } else if b.denom().is_one() {
            return common_power(a.denom(), b.numer()).is_some();
        }
    }

    // In the general case, two commonPower computations must produce the same result.
    let common_power_na_nb = common_power(a.numer(), b.numer());
    let common_power_na_db = common_power(a.numer(), b.denom());
    if common_power_na_nb.is_some() && common_power_na_nb == common_power(a.denom(), b.denom()) {
        // They have a common power, and both exponents have the same sign.
        return true;
    }
    if common_power_na_db.is_some() && common_power_na_db == common_power(a.denom(), b.numer()) {
        // They have a common power, and exponents have different sign.
        return true;
    }

    false
}

/// Return the integral log with respect to the given base if it exists, 0 otherwise. n is presumed
/// positive. base is presumed to be at least 2.
fn get_int_log(n: BigInt, base: i32) -> i64 {
    let n_as_double = n.to_f64().unwrap();
    let approx = n_as_double.log(base as f64);

    // A relatively quick test first.
    // Try something else for values too big for a double.
    if n_as_double.is_infinite() {
        // Floating point test doesn't help. Try another quick test.
        if base % 2 != 0 && !n.bit(0) {
            // Has a divisor of 2. Can't be a power of an odd number.
            return 0;
        }
        if base % 3 != 0 && n.clone().rem(BigInt::from_i32(3).unwrap()).is_zero() {
            return 0;
        }
        if base % 5 != 0 && n.clone().rem(BigInt::from_i32(5).unwrap()).is_zero() {
            return 0;
        }
    } else if (approx - approx.round_ties_even()).abs() > 1.0e-6 {
        return 0;
    }

    // It's important to avoid allocating large numbers of large BigIntegers here.
    // In particular, we need to avoid e.g. repeatedly dividing by base.
    // Otherwise computations like log(100,000!) can behave very badly.
    // This algorithm performs worst case O(log(log n)) BigInteger operations.
    // We build a set of powers of base by repeated squaring, with powers[i] == base^(2^i).
    let mut result = 0;
    let mut powers = Vec::new();
    powers.push(BigInt::from_i32(base).unwrap());
    let mut n_reduced = n; // always equal to n/base^result .
    let mut i = 1;
    loop {
        let last = powers.get(i - 1).unwrap();
        let next = last * last; // base^(2^i)
        if next.bits() > n_reduced.bits() {
            break;
        }

        let q_and_r = n_reduced.div_mod_floor(&next.clone());
        if !q_and_r.1.is_zero() {
            // A power of base smaller than 2 * n_reduced didn't divide n_reduced.
            // n_reduced, and thus n, are clearly not a power of base.
            return 0;
        }
        powers.push(next);

        // Since we have the quotient, opportunistically reduce n.
        result += 1 << i;
        n_reduced = q_and_r.0;
        i += 1;
    }

    // We've computed all repeated squaring powers <= n_reduced.
    // Use those to divide repeatedly until we get to one, or determine it's not
    // a power of base.
    let mut i = powers.len() - 1;
    while !n_reduced.is_one() {
        let power = powers.get(i).unwrap();
        if power.bits() <= n_reduced.bits() {
            let q_and_r = n_reduced.div_mod_floor(power);
            if !q_and_r.1.is_zero() {
                return 0;
            }
            result += 1 << i;
            n_reduced = q_and_r.0;
            // Now power.bitLength() > n_reduced.bitLength() .
            // Otherwise we would have divided by the next bigger power, which is power^2.
        }
        i -= 1;
    }
    result
}

/// Compute the sin of an integer multiple n of pi/12, if easily representable.
///
/// Parameters:
/// n: value between 0 and 23 inclusive.
fn sin_pi_twelfths(n: i32) -> Option<Real> {
    if n >= 12 {
        let neg_result = sin_pi_twelfths(n - 12)?;
        return Some(-neg_result);
    }

    match n {
        0 => Some(ZERO.clone()),
        2 => Some(HALF.clone()),
        3 => Some(HALF_SQRT_2.clone()),
        4 => Some(HALF_SQRT_3.clone()),
        6 => Some(constants::ONE.clone()),
        8 => Some(HALF_SQRT_3.clone()),
        9 => Some(HALF_SQRT_2.clone()),
        10 => Some(HALF.clone()),
        _ => None,
    }
}

fn cos_pi_twelfths(n: i32) -> Option<Real> {
    let mut sin_arg = n + 6;
    if sin_arg >= 24 {
        sin_arg -= 24;
    }
    sin_pi_twelfths(sin_arg)
}

/// Generalized factorial. Compute n * (n - step) * (n - 2 * step) * etc. This can be used to
/// compute factorial a bit faster, especially if BigInteger uses sub-quadratic multiplication.
fn gen_factorial(n: i64, step: i64, cancellation_token: CancellationToken) -> NumResult<BigInt> {
    if n > 4 * step {
        let prod1 = gen_factorial(n, 2 * step, cancellation_token.clone())?;
        cancellation_token.stop_if_cancelled()?;

        let prod2 = gen_factorial(n - step, 2 * step, cancellation_token.clone())?;
        cancellation_token.stop_if_cancelled()?;

        Ok(prod1 * prod2)
    } else if n == 0 {
        Ok(BigInt::one())
    } else {
        let mut res = BigInt::from_i64(n).unwrap();
        let mut i = n - step;
        while i > 1 {
            res *= BigInt::from_i64(i).unwrap();
            i -= step
        }
        Ok(res)
    }
}

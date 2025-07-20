pub mod constants;
mod cr_property;
mod signed_property;

use crate::constructive_real::ConstructiveReal;
use crate::constructive_real::constants::ONE;
use crate::error::NumResult;
use crate::real::cr_property::{CRProperty, CRPropertyType};
use num::bigint::Sign;
use num::{BigInt, BigRational, FromPrimitive, Integer, One, Signed, Zero};
use std::cmp::Ordering;
use std::ops::Div;
use num::traits::Inv;
use crate::angle_unit::AngleUnit;
use crate::rational_extensions::RationalExtensions;

static COMMON_POWER_LENGTH_LIMIT: u64 = 200;

#[derive(Debug, Clone)]
pub struct Real {
    rat: BigRational,
    cr: ConstructiveReal,
    cr_property: CRProperty,
}

impl Real {
    pub fn new(rat: BigRational, cr: ConstructiveReal, cr_property: CRProperty) -> Self {
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
            cr_property: Option::<CRProperty>::from(cr.clone()).unwrap(),
            cr,
        }
    }

    pub fn new_from_cr(cr: ConstructiveReal) -> Self {
        Self::new_from_rat_cr(BigRational::one(), cr)
    }

    pub fn new_from_cr_property(cr: ConstructiveReal, cr_property: CRProperty) -> Self {
        Self::new(BigRational::one(), cr, cr_property)
    }

    pub fn new_from_rat_property(rat: BigRational, cr_property: CRProperty) -> Self {
        Self::new(rat, cr_property.cr().unwrap().unwrap(), cr_property)
    }

    pub fn new_from_property(cr_property: CRProperty) -> Self {
        Self::new_from_rat_property(BigRational::one(), cr_property)
    }

    pub fn new_from_rational(rat: BigRational) -> Self {
        Self::new(rat, ONE.clone(), CRProperty::one())
    }

    /// Check that if crProperty uniquely defines a constructive real, then crProperty
    /// and crFactor both describe approximately the same number.
    pub fn property_correct(&self, prec: i32) -> NumResult<bool> {
        let property_cr = self.cr_property.cr()?;
        if let Some(property_cr) = property_cr {
            let bound = self.cr_property.msb_bound();
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

        match self.cr_property.kind {
            CRPropertyType::Pi => true,
            CRPropertyType::Sqrt => false,
            CRPropertyType::Ln => {
                // arg > 1
                // Follows from Lindemann-Weierstrass theorem. If ln(r) = a, where r is rational, and a
                // algebraic, then r = e^a. But if a is nonzero algebraic, then e^a is transcendental.
                true
            }
            CRPropertyType::Log => {
                // If this is rational, then n ln(arg) = m ln(10), n and m integers.
                // TODO: Can we do better?
                false
            }
            CRPropertyType::Exp => {
                // arg != 0
                // Simple application of Lindemann-Weierstrass theorem.
                true
            }
            CRPropertyType::SinPi | CRPropertyType::TanPi => {
                // Always algebraic for rational multiples of pi.
                false
            }
            CRPropertyType::Asin | CRPropertyType::Atan => {
                // If asin(r) = a, r rational, a algebraic, then r = sin(a). It follows from
                // Lindemann-Weierstrass that this can happen only if a is zero, i.e. if r is zero.
                // We don't use this representation for asin(0). The atan argument is similar.
                true
            }
            CRPropertyType::Irrational => {
                // Not enough information to tell.
                false
            }
            _ => unreachable!()
        }
    }

    /// Do we know that this.crFactor is an irrational nonzero multiple of u.crFactor? If this returns
    /// true, then a comparison of the two UnifiedReals cannot diverge, though we don't know of a good
    /// runtime bound. Note that if both values are really tiny, it still may be completely impractical
    /// to compare them.
    pub fn definitely_independent(&self, other: &Self) -> bool {
        // We always return false if either crFactor might be zero.
        let p1 = &self.cr_property;
        let p2 = &other.cr_property;
        if p1 == p2 {
            return false;
        }

        // Halve the number of cases. ONE < PI < SQRT < EXP < LN.
        if p1.kind > p2.kind {
            return other.definitely_independent(self);
        }

        match p1.kind {
            CRPropertyType::One => other.definitely_irrational(),
            CRPropertyType::Pi => {
                // It appears to be unknown whether pi is a rational multiple of an exponential or log.
                // If we were brave, we could say true, and hope for an infinite loop, which would
                // probably prove an interesting theorem. But we are not ...
                // IS_ONE case is already handled, since p1 <= p2.
                return p2.kind == CRPropertyType::Sqrt;
            }
            CRPropertyType::Sqrt => {
                if other.definitely_transcendental() {
                    true
                } else if p2.kind == CRPropertyType::Sqrt {
                    // The argument is not necessarily minimal.
                    p1.arg.clone().unwrap().irreducible_sqrt() && p2.arg.clone().unwrap().irreducible_sqrt() && p1 != p2
                } else {
                    false
                }
            }
            CRPropertyType::Exp => {
                if p2.kind == CRPropertyType::Exp {
                    // Lindemann-Weierstrass theorem gives us algebraic independence.
                    p1.arg != p2.arg
                } else if p2.kind == CRPropertyType::Ln {
                    // If e^a = cln(b), then e^e^a = b^c. The r.h.s is an algebraic multiple of e^0.
                    // By Lindemann-Weierstrass, this can only happen if e^a = 0, which is impossible.
                    true
                } else {
                    other.definitely_algebraic()
                }
            }
            CRPropertyType::Ln => {
                if p2.kind == CRPropertyType::Irrational {
                    false // Not enough information.
                } else if p2.kind == CRPropertyType::Ln {
                    // If ln(a) = cln(b), then a = b^c, a, b, and c rational, or equivalently a^c1 = b^c2,
                    // with c1 and c2 integers. C must be nonzero, since a > 1.  A necessary condition for
                    // this is that the numerator and denominator separately have to have a common integral
                    // power.
                    !have_common_power(&p1.arg.clone().unwrap(), &p2.arg.clone().unwrap())
                } else {
                    // Assume ln(r) = a is algebraic. Then e^a is rational. By Lindemann-Weierstrass, this
                    // implies a = 0 and r = 1. We know that the argument is not one, so any algebraic
                    // number must be linearly independent over the rationals.
                    other.definitely_algebraic()
                    // TODO: Can we do better for IS_LOG?
                }
            }
            CRPropertyType::Log => {
                // In the irrational case, with u rational, we would have checked in the other order.
                if p2.kind == CRPropertyType::Log {
                    // We're asking if ln(a)/ln(10) = r ln(b)/ln(10), which is true iff ln(a) = r ln(b).
                    // Use the same algorithm as for IS_LN.
                    !have_common_power(&p1.arg.clone().unwrap(), &p2.arg.clone().unwrap())
                } else {
                    false
                }
            }
            CRPropertyType::SinPi | CRPropertyType::TanPi => {
                // Always algebraic. We already handled the other rational case above.
                other.definitely_transcendental()
            }
            CRPropertyType::Asin => {
                // As we argued above, this is transcendental.
                other.definitely_algebraic()
            }
            CRPropertyType::Atan => {
                // The case of other rational is handled above. Can we do better?
                false
            }
            CRPropertyType::Irrational => false
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
    } else {
        if a.is_one() || b.is_one() {
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
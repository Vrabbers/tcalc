use crate::constructive_real::add_constructive::AddConstructive;
use crate::constructive_real::assumed_int_constructive::AssumedIntConstructive;
use crate::constructive_real::big_integer_constructive::BigIntegerConstructive;
use crate::constructive_real::invalid_constructive::InvalidConstructive;
use crate::constructive_real::inverse_tan_reciporical_constructive::InverseTanReciprocalConstructive;
use crate::constructive_real::inverted_constructive::InvertedConstructive;
use crate::constructive_real::multiply_constructive::MultiplyConstructive;
use crate::constructive_real::negated_constructive::NegatedConstructive;
use crate::constructive_real::prescaled_cos_constructive::PrescaledCosConstructive;
use crate::constructive_real::prescaled_exp_constructive::PrescaledExpConstructive;
use crate::constructive_real::prescaled_ln_constructive::PrescaledLnConstructive;
use crate::constructive_real::select_constructive::SelectConstructive;
use crate::constructive_real::shift_constructive::ShiftConstructive;
use crate::constructive_real::square_root_constructive::SquareRootConstructive;
use crate::error::InternalError::{ConstructiveRealFromInf, ConstructiveRealFromNan};
use crate::error::NumError::{DomainViolation, InternalError, PrecisionOverflow};
use crate::error::{CancelCheckable, NumError, NumResult};
use cancellation_token::CancellationToken;
use num::bigint::Sign;
use num::{BigInt, BigRational, Integer, One, Signed, ToPrimitive, Zero};
use std::clone::Clone;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Shl, Shr, Sub};
use std::ptr;
use std::sync::{Arc, RwLock};
use crate::error::DomainViolation::LogarithmDomainViolation;
use crate::error::LogarithmDomainViolation::LogOfNegative;

mod add_constructive;
mod assumed_int_constructive;
mod big_integer_constructive;
mod invalid_constructive;
mod inverse_tan_reciporical_constructive;
mod inverted_constructive;
mod multiply_constructive;
mod negated_constructive;
mod prescaled_cos_constructive;
mod prescaled_exp_constructive;
mod prescaled_ln_constructive;
mod select_constructive;
mod shift_constructive;
mod square_root_constructive;

pub mod constants;
// https://android.googlesource.com/platform/external/crcalc/+/6db978c639e9bd5ac63fd88cbf3765d8c0fb3271/src/com/hp/creals/CR.java

#[derive(Copy, Clone, Debug)]
pub enum ConstructiveRealKnownValue {
    One,
    Pi,
    Sqrt2,
    Sqrt3,
    E,
    Ln10,
}

#[derive(Clone, Debug)]
pub struct ConstructiveReal {
    t: Arc<dyn ConstructiveRealType>,
    current_approximation: Arc<RwLock<Option<ConstructiveRealApproximation>>>,
    pub cancellation_token: CancellationToken,
    pub known_value: Option<ConstructiveRealKnownValue>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConstructiveRealApproximation {
    pub min_prec: i32,
    pub max_appr: BigInt,
}

trait ConstructiveRealType
where
    Self: Debug,
    Self: Send,
    Self: Sync,
{
    /// Should be true for implementations for which approximate calls are
    /// somewhat expensive. Default implementation just returns false.
    /// If we need to (re)evaluate, we speculatively evaluate to slightly
    /// higher precision, miminimizing reevaluations.
    /// Note that this requires any arguments to be evaluated to higher
    /// precision than absolutely necessary.  It can thus potentially
    /// result in lots of wasted effort, and should be used judiciously.
    /// This assumes that the order of magnitude of the number is roughly one.
    fn is_slow(&self) -> bool {
        false
    }

    ///  Must be defined in implementors of ConstructiveRealApproximation
    ///  Returns value / 2 ** precision rounded to an integer.
    ///  The error in the result is strictly < 1.
    ///  Informally, approximate(n) gives a scaled approximation
    ///  accurate to 2**n.
    ///  Implementations may safely assume that precision is
    ///  at least a factor of 8 away from overflow.
    fn approximate(&self, precision: i32, ct: CancellationToken) -> NumResult<BigInt>;
}

impl ConstructiveReal {
    pub(crate) fn get_appr(&self, precision: i32) -> NumResult<BigInt> {
        check_prec(precision)?;

        let mut current_approximation_borrow = self.current_approximation.write().unwrap();
        let current_approximation = current_approximation_borrow.as_ref();

        if self.t.is_slow() {
            let max_prec = -64;
            let prec_incr = 32;

            if let Some(current_approximation) = current_approximation
                && precision >= current_approximation.min_prec
            {
                Ok(scale(
                    current_approximation.max_appr.clone(),
                    current_approximation.min_prec - precision,
                ))
            } else {
                let eval_prec = if precision >= max_prec {
                    max_prec
                } else {
                    (precision - prec_incr + 1) & !(prec_incr - 1)
                };
                let result = self
                    .t
                    .approximate(eval_prec, self.cancellation_token.clone())?;

                if let Some(current_approximation) = current_approximation_borrow.as_mut() {
                    current_approximation.min_prec = precision;
                    current_approximation.max_appr = result.clone();
                } else {
                    current_approximation_borrow.replace(ConstructiveRealApproximation {
                        min_prec: precision,
                        max_appr: result.clone(),
                    });
                }
                Ok(scale(result, eval_prec - precision))
            }
        } else if let Some(current_approximation) = current_approximation
            && precision >= current_approximation.min_prec
        {
            Ok(scale(
                current_approximation.max_appr.clone(),
                current_approximation.min_prec - precision,
            ))
        } else {
            let result = self
                .t
                .approximate(precision, self.cancellation_token.clone())?;
            if let Some(current_approximation) = current_approximation_borrow.as_mut() {
                current_approximation.min_prec = precision;
                current_approximation.max_appr = result.clone();
            } else {
                current_approximation_borrow.replace(ConstructiveRealApproximation {
                    min_prec: precision,
                    max_appr: result.clone(),
                });
            }
            Ok(result)
        }
    }

    /// Return the position of the msd.
    /// If x.msd() == n then
    /// 2**(n-1) < abs(x) < 2**(n+1)
    /// This initial version assumes that max_appr is valid
    /// and sufficiently removed from zero
    /// that the msd is determined.
    fn known_msd(&self) -> i32 {
        let current_approximation_borrow = self.current_approximation.read().unwrap();
        let current_approximation = current_approximation_borrow.as_ref().unwrap();

        let length = if current_approximation.max_appr.sign() != Sign::Minus {
            current_approximation.max_appr.bits()
        } else {
            current_approximation.max_appr.abs().bits()
        } as i32;

        current_approximation.min_prec + length - 1
    }

    /// This version may return i32::MIN if the correct
    /// answer is < n.
    fn msd_n(&self, n: i32) -> NumResult<i32> {
        let current_approximation_borrow = self.current_approximation.read().unwrap();
        let current_approximation = current_approximation_borrow.as_ref();
        if current_approximation.is_none() || {
            let current_approximation = current_approximation.unwrap();
            current_approximation.max_appr <= BigInt::one()
                && current_approximation.max_appr >= BigInt::one().mul(-1)
        } {
            drop(current_approximation_borrow);
            self.get_appr(n - 1)?;

            if self
                .current_approximation
                .read()
                .unwrap()
                .as_ref()
                .unwrap()
                .max_appr
                .abs()
                <= BigInt::one()
            {
                return Ok(i32::MIN);
            }
        }

        Ok(self.known_msd())
    }

    /// Functionally equivalent, but iteratively evaluates to higher
    /// precision.
    fn iter_msd(&self, n: i32) -> NumResult<i32> {
        let mut prec = 0;
        while prec > n + 30 {
            let msd = self.msd_n(prec)?;
            if msd != i32::MIN {
                return Ok(msd);
            }
            check_prec(prec)?;
            self.cancellation_token.stop_if_cancelled()?;
            prec = (prec * 3) / 2 - 16;
        }
        self.msd_n(n)
    }

    /// This version returns a correct answer eventually, except
    /// that it loops forever (or returns an error) if this
    /// constructive real is zero.
    fn msd(&mut self) -> NumResult<i32> {
        self.iter_msd(i32::MIN)
    }

    /// Return 0 if x = y to within the indicated tolerance,
    /// -1 if x < y, and +1 if x > y.  If x and y are indeed
    /// equal, it is guaranteed that 0 will be returned.  If
    /// they differ by less than the tolerance, anything
    /// may happen.  The tolerance allowed is
    /// the maximum of (abs(this)+abs(x))*(2\**r) and 2\**a
    ///
    /// Parameters:
    /// - x: The other constructive real
    /// - r: Relative tolerance in bits
    /// - a: Absolute tolerance in bits
    pub fn compare_to_relative(&self, x: &Self, r: i32, a: i32) -> NumResult<Ordering> {
        let this_msd = self.iter_msd(a)?;
        let x_msd = x.iter_msd(if this_msd > a { this_msd } else { a })?;
        let max_msd = if x_msd > this_msd { x_msd } else { this_msd };
        let rel = max_msd + r;

        // This can't approach overflow, since r and a are
        // effectively divided by 2, and msds are checked.
        let abs_prec = if rel > a { rel } else { a };
        self.compare_to_absolute(x, abs_prec)
    }

    /// Approximate comparison with only an absolute tolerance.
    /// Identical to the three argument version, but without a relative
    /// tolerance.
    /// Result is 0 if both constructive reals are equal, indeterminate
    /// if they differ by less than 2**a.
    ///
    /// Parameters:
    /// - x: The other constructive real
    /// - a: Absolute tolerance in bits
    pub fn compare_to_absolute(&self, x: &Self, a: i32) -> NumResult<Ordering> {
        let needed_prec = a - 1;
        let this_appr = self.get_appr(needed_prec)?;
        let x_appr = x.get_appr(needed_prec)?;
        let comp1 = this_appr.cmp(&x_appr.clone().add(&BigInt::one()));
        if comp1 == Ordering::Less {
            return Ok(Ordering::Less);
        }
        let comp2 = this_appr.cmp(&x_appr.clone().sub(&BigInt::one()));
        if comp2 == Ordering::Greater {
            return Ok(Ordering::Greater);
        }
        
        Ok(Ordering::Equal)
    }

    /// Return -1 if self < x, or +1 if self > x.
    /// Should be called only if self != x.
    /// If self == x, this will not terminate correctly; typically it
    /// will run until it exhausts memory.
    /// If the two constructive reals may be equal, the two or 3 argument
    /// version of compare_to should be used.
    pub fn compare_to(&self, x: &Self) -> NumResult<Ordering> {
        let mut a = -20;
        loop {
            check_prec(a)?;
            let result = self.compare_to_absolute(x, a)?;
            if result != Ordering::Equal {
                return Ok(result);
            }
            self.cancellation_token.stop_if_cancelled()?;
            a *= 2;
        }
    }

    /// Equivalent to <TT>compareTo(CR.valueOf(0), a)</tt>
    pub fn sign_precision(&self, a: i32) -> NumResult<Sign> {
        if let Some(current_approximation) = self.current_approximation.read().unwrap().as_ref() {
            let quick_try = current_approximation.max_appr.sign();
            if quick_try != Sign::NoSign {
                return Ok(quick_try);
            }
        }
        let needed_prec = a - 1;
        let this_appr = self.get_appr(needed_prec)?;
        let sign = this_appr.sign();
        Ok(sign)
    }

    /// Return -1 if negative, +1 if positive.
    /// Should be called only if self != 0.
    /// In the 0 case, this will not terminate correctly; typically it
    /// will run until it exhausts memory.
    /// If the two constructive reals may be equal, the one or two argument
    /// version of sign should be used.
    pub fn sign(&self) -> NumResult<Sign> {
        let mut a = -20;
        loop {
            check_prec(a)?;
            let sign = self.sign_precision(a)?;
            if sign != Sign::NoSign {
                return Ok(sign);
            }
            a *= 2;
        }
    }

    /// Return a textual representation accurate to <TT>n</tt> places
    /// to the right of the decimal point.  <TT>n</tt> must be nonnegative.
    ///
    /// Parameters:
    /// - n: Number of digits (>= 0) included to the right of decimal point
    /// - radix: Base ( >= 2, <= 16) for the resulting representation
    pub fn to_string(&self, n: u32, radix: u32) -> NumResult<String> {
        let scaled_cr: ConstructiveReal = if radix == 16 {
            (self.clone() << (4 * n) as i32)?
        } else {
            let scale_factor = BigInt::from(radix).pow(n);
            self.clone() * ConstructiveReal::from(scale_factor)
        };

        let scaled_int = scaled_cr.get_appr(0)?;
        let mut scaled_string = scaled_int.abs().to_str_radix(radix);
        let result = if n == 0 {
            scaled_string
        } else {
            let mut len = scaled_string.len() as u32;
            if len <= n {
                scaled_string =
                    str::repeat("0", (n + 1 - len) as usize).to_string() + &scaled_string;
                len = n + 1
            }
            let whole = &scaled_string[0..(len - n) as usize];
            let fraction = &scaled_string[(len - n) as usize..];
            format!("{whole}.{fraction}")
        };
        Ok(if scaled_int.sign() == Sign::Minus {
            format!("-{result}")
        } else {
            result
        })
    }

    /// Return a BigInteger which differs by less than one from the
    /// constructive real.
    pub fn to_bigint(&self) -> NumResult<BigInt> {
        self.clone().get_appr(0)
    }

    /// Produce a constructive real equivalent to the original, assuming
    /// the original was an integer.  Undefined results if the original
    /// was not an integer.  Prevents evaluation of digits to the right
    /// of the decimal point, and may thus improve performance.
    pub fn assume_int(self) -> ConstructiveReal {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Arc::new(AssumedIntConstructive(self)),
            ..ConstructiveReal::default()
        }
    }

    /// The multiplicative inverse of a constructive real.
    /// x.inverse() is equivalent to ConstructiveReal::from(1).divide(x).
    pub fn inverse(self) -> ConstructiveReal {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Arc::new(InvertedConstructive(self)),
            ..ConstructiveReal::default()
        }
    }

    /// The real number x if self < 0, or y< otherwise.
    /// Requires x = y if self = 0.
    /// Since comparisons may diverge, this is often
    /// a useful alternative to conditionals.
    pub fn select(self, x: Self, y: Self) -> ConstructiveReal {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Arc::new(SelectConstructive {
                selector: self,
                op1: x,
                op2: y,
            }),
            ..ConstructiveReal::default()
        }
    }

    pub fn max(self, other: Self) -> ConstructiveReal {
        (self.clone() - other.clone()).select(other, self)
    }

    pub fn min(self, other: Self) -> ConstructiveReal {
        (self.clone() - other.clone()).select(self, other)
    }

    pub fn abs(self) -> ConstructiveReal {
        self.clone().select(-self.clone(), self)
    }

    /// The exponential function, that is e**self
    pub fn exp(self) -> NumResult<ConstructiveReal> {
        let low_prec = -10;
        let rough_appr = self.get_appr(low_prec)?;
        // Handle negative arguments directly; negating and computing inverse
        // can be very expensive.
        if rough_appr > BigInt::from(2) || rough_appr < BigInt::from(-2) {
            let square_root = (self >> 1)?.exp()?;
            Ok(square_root.clone() * square_root)
        } else {
            Ok(ConstructiveReal {
                cancellation_token: self.cancellation_token.clone(),
                t: Arc::new(PrescaledExpConstructive(self)),
                ..ConstructiveReal::default()
            })
        }
    }

    /// The natural (base e) logarithm
    pub fn ln(self) -> NumResult<ConstructiveReal> {
        let low_ln_limit: BigInt = BigInt::from(8);
        let high_ln_limit: BigInt = BigInt::from(16 + 8 /* 1.5 */);
        let scaled_4: BigInt = BigInt::from(4 * 16);
        let ten_ninths: ConstructiveReal = ConstructiveReal::from(10) / ConstructiveReal::from(9);
        let twentyfive_twentyfourths: ConstructiveReal =
            ConstructiveReal::from(25) / ConstructiveReal::from(24);
        let eightyone_eightyeths: ConstructiveReal =
            ConstructiveReal::from(81) / ConstructiveReal::from(80);
        let ln2_1: ConstructiveReal = ConstructiveReal::from(7) * ten_ninths.clone().simple_ln();
        let ln2_2: ConstructiveReal =
            ConstructiveReal::from(2) * twentyfive_twentyfourths.clone().simple_ln();
        let ln2_3: ConstructiveReal =
            ConstructiveReal::from(3) * eightyone_eightyeths.clone().simple_ln();
        let ln2: ConstructiveReal = ln2_1.clone() - ln2_2.clone() + ln2_3.clone();

        let low_prec = -4;
        let rough_appr = self.get_appr(low_prec)?; /* In sixteenths */
        if rough_appr < BigInt::zero() {
            return Err(DomainViolation(LogarithmDomainViolation(LogOfNegative)));
        };
        if rough_appr <= low_ln_limit {
            return Ok(-self.inverse().ln()?);
        }

        if rough_appr >= high_ln_limit {
            return if rough_appr <= scaled_4 {
                let quarter = self.sqrt().sqrt().ln()?;
                quarter << 2
            } else {
                let extra_bits = rough_appr.bits() - 3;
                let scaled_result = (self >> extra_bits as i32)?.ln()?;
                Ok(scaled_result + (ConstructiveReal::from(extra_bits) * ln2))
            };
        }
        Ok(self.simple_ln())
    }

    pub fn simple_ln(self) -> ConstructiveReal {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Arc::new(PrescaledLnConstructive(self - ConstructiveReal::from(1))),
            ..ConstructiveReal::default()
        }
    }

    pub fn sqrt(self) -> ConstructiveReal {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Arc::new(SquareRootConstructive(self)),
            ..ConstructiveReal::default()
        }
    }

    pub fn atan_reciporical(n: i32) -> ConstructiveReal {
        ConstructiveReal {
            t: Arc::new(InverseTanReciprocalConstructive(n)),
            ..ConstructiveReal::default()
        }
    }

    pub fn pi() -> ConstructiveReal {
        let four = Self::from(4);
        (four.clone() * (four.clone() * Self::atan_reciporical(5) - Self::atan_reciporical(239)))
            .with_known_value(ConstructiveRealKnownValue::Pi)
    }

    pub fn sin(self) -> NumResult<ConstructiveReal> {
        (Self::pi() / ConstructiveReal::from(2) - self).cos()
    }

    pub fn cos(self) -> NumResult<ConstructiveReal> {
        let rough_appr = self.get_appr(-1)?;
        let abs_rough_appr = rough_appr.abs();
        if abs_rough_appr >= BigInt::from(6) {
            // Subtract multiples of PI
            let multiplier = rough_appr / BigInt::from(6);
            let adjustment = Self::pi() * ConstructiveReal::from(multiplier.clone());
            if (multiplier & BigInt::one()).sign() == Sign::NoSign {
                Ok(-(self - adjustment).cos()?)
            } else {
                (self - adjustment).cos()
            }
        } else if abs_rough_appr >= BigInt::from(2) {
            // Scale further with double angle formula
            let cos_half = (self >> 1)?.cos()?;
            Ok(((cos_half.clone() * cos_half) << 1)? - ConstructiveReal::from(1))
        } else {
            Ok(ConstructiveReal {
                cancellation_token: self.cancellation_token.clone(),
                t: Arc::new(PrescaledCosConstructive(self)),
                ..ConstructiveReal::default()
            })
        }
    }

    pub fn with_cancellation_token(mut self, cancellation_token: CancellationToken) -> Self {
        self.cancellation_token = cancellation_token;
        self
    }

    pub fn with_known_value(mut self, known_value: ConstructiveRealKnownValue) -> Self {
        self.known_value = Some(known_value);
        self
    }
}

impl Default for ConstructiveReal {
    fn default() -> Self {
        ConstructiveReal {
            t: Arc::new(InvalidConstructive()),
            current_approximation: Arc::new(RwLock::new(None)),
            cancellation_token: CancellationToken::new(false),
            known_value: None,
        }
    }
}

impl From<BigInt> for ConstructiveReal {
    fn from(n: BigInt) -> Self {
        ConstructiveReal {
            t: Arc::new(BigIntegerConstructive(n)),
            ..ConstructiveReal::default()
        }
    }
}

impl From<i32> for ConstructiveReal {
    fn from(value: i32) -> Self {
        ConstructiveReal::from(BigInt::from(value))
    }
}

impl From<i64> for ConstructiveReal {
    fn from(value: i64) -> Self {
        ConstructiveReal::from(BigInt::from(value))
    }
}

impl From<u32> for ConstructiveReal {
    fn from(value: u32) -> Self {
        ConstructiveReal::from(BigInt::from(value))
    }
}

impl From<u64> for ConstructiveReal {
    fn from(value: u64) -> Self {
        ConstructiveReal::from(BigInt::from(value))
    }
}

impl From<BigRational> for ConstructiveReal {
    fn from(value: BigRational) -> Self {
        ConstructiveReal::from(value.numer().clone())
            / ConstructiveReal::from(value.denom().clone())
    }
}

impl TryFrom<f64> for ConstructiveReal {
    type Error = NumError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_nan() {
            return Err(InternalError(ConstructiveRealFromNan));
        }
        if value.is_infinite() {
            return Err(InternalError(ConstructiveRealFromInf));
        }
        let negative = value < 0.0;
        let bits = value.abs().to_bits();
        let mut mantissa = bits & 0xfffffffffffff;
        let biased_exp = (bits >> 52) as u32;
        let exp = biased_exp - 1075;
        if biased_exp != 0 {
            mantissa += 1_u64 << 52;
        } else {
            mantissa <<= 1;
        }

        let result = (ConstructiveReal::from(mantissa) << exp as i32)?;
        Ok(if negative { -result } else { result })
    }
}

impl TryFrom<f32> for ConstructiveReal {
    type Error = NumError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        ConstructiveReal::try_from(value as f64)
    }
}

impl Display for ConstructiveReal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.to_string(10, 10).unwrap_or("Error".to_string())
        )
    }
}

impl Add for ConstructiveReal {
    type Output = ConstructiveReal;

    fn add(self, rhs: Self) -> Self::Output {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Arc::new(AddConstructive {
                op1: self,
                op2: rhs,
            }),
            ..ConstructiveReal::default()
        }
    }
}

impl Shl<i32> for ConstructiveReal {
    type Output = NumResult<ConstructiveReal>;

    fn shl(self, rhs: i32) -> Self::Output {
        check_prec(rhs)?;
        Ok(ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Arc::new(ShiftConstructive {
                op: self,
                count: rhs,
            }),
            ..ConstructiveReal::default()
        })
    }
}

impl Shr<i32> for ConstructiveReal {
    type Output = NumResult<ConstructiveReal>;

    fn shr(self, rhs: i32) -> Self::Output {
        check_prec(rhs)?;
        Ok(ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Arc::new(ShiftConstructive {
                op: self,
                count: -rhs,
            }),
            ..ConstructiveReal::default()
        })
    }
}

impl Sub for ConstructiveReal {
    type Output = ConstructiveReal;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul for ConstructiveReal {
    type Output = ConstructiveReal;

    fn mul(self, rhs: Self) -> Self::Output {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Arc::new(MultiplyConstructive {
                op1: self,
                op2: rhs,
            }),
            ..ConstructiveReal::default()
        }
    }
}

impl Div for ConstructiveReal {
    type Output = ConstructiveReal;

    #[allow(
        clippy::suspicious_arithmetic_impl,
        reason = "This is the intended implementation."
    )]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl Neg for ConstructiveReal {
    type Output = ConstructiveReal;

    fn neg(self) -> Self::Output {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Arc::new(NegatedConstructive(self)),
            ..ConstructiveReal::default()
        }
    }
}

impl From<ConstructiveReal> for NumResult<BigInt> {
    fn from(value: ConstructiveReal) -> Self {
        value.clone().get_appr(0)
    }
}

impl From<ConstructiveReal> for NumResult<i32> {
    fn from(value: ConstructiveReal) -> Self {
        value.clone().get_appr(0).map(|n| n.to_i32().unwrap())
    }
}

impl From<ConstructiveReal> for NumResult<f64> {
    fn from(value: ConstructiveReal) -> Self {
        let value = value.clone();
        let my_msd = value.iter_msd(-1080 /* slightly > exp. range */)?;
        if my_msd == i32::MIN {
            return Ok(0.0);
        };
        let needed_prec = my_msd - 60;
        let scaled_int = value.get_appr(needed_prec)?.to_i64().unwrap() as f64;
        let may_underflow = needed_prec < -1000;
        let mut scaled_int_rep = scaled_int.to_bits();
        let exp_adj = if may_underflow {
            needed_prec + 96
        } else {
            needed_prec
        } as u64;
        let orig_exp = (scaled_int_rep >> 52) & 0x7ff;
        if ((orig_exp.overflowing_add(exp_adj).0) & !0x7ff) != 0 {
            // overflow
            if scaled_int < 0.0 {
                return Ok(f64::NEG_INFINITY);
            } else {
                return Ok(f64::INFINITY);
            }
        }

        scaled_int_rep = scaled_int_rep.overflowing_add(exp_adj << 52).0;
        let result = f64::from_bits(scaled_int_rep);
        if may_underflow {
            let two48 = (1_u64 << 48) as f64;
            Ok(result / two48 / two48)
        } else {
            Ok(result)
        }
    }
}

impl PartialEq for ConstructiveReal {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.current_approximation.data_ptr(), other.current_approximation.data_ptr())
    }
}

fn bound_log2(n: i32) -> i32 {
    let abs_n = n.abs();
    ((abs_n + 1) as f64).log(2.).ceil() as i32
}

/// Check that a precision is at least a factor of 8 away from
/// overflowng the integer used to hold a precision spec.
/// We generally perform this check early on, and then convince
/// ourselves that none of the operations performed on precisions
/// inside a function can generate an overflow.
fn check_prec(n: i32) -> NumResult<()> {
    let high = n >> 28;

    // if n is not in danger of overflowing, then the 4 high order
    // bits should be identical. Thus high is either 0 or -1.
    // The rest of this is to test for either of those in a way
    // that should be as cheap as possible.
    let high_shifted = n >> 29;
    if (high ^ high_shifted) != 0 {
        Err(PrecisionOverflow)
    } else {
        Ok(())
    }
}

fn shift(k: BigInt, n: i32) -> BigInt {
    if n == 0 {
        k
    } else if n < 0 {
        k.div_floor(&BigInt::from(2).pow(n.unsigned_abs()))
    } else {
        k.mul(&BigInt::from(2).pow(n as u32))
    }
}

fn scale(k: BigInt, n: i32) -> BigInt {
    if n >= 0 {
        shift(k, n)
    } else {
        let adj_k = shift(k, n + 1).add(&BigInt::from(1));
        shift(adj_k, -1)
    }
}

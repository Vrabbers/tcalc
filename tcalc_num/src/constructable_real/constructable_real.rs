use crate::error::InternalError::{ConstructableRealFromInf, ConstructableRealFromNan};
use crate::error::NumError::{InternalError, PrecisionOverflow};
use crate::error::{NumError, NumResult};
use num::bigint::Sign;
use num::traits::real::Real;
use num::{BigInt, Integer, One, Signed, ToPrimitive, Zero};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::mem::swap;
use std::ops::{Add, Div, Mul, Neg, Shl, Shr, Sub};
// https://android.googlesource.com/platform/external/crcalc/+/6db978c639e9bd5ac63fd88cbf3765d8c0fb3271/src/com/hp/creals/CR.java

#[derive(Clone, Debug)]
enum ConstructableRealType {
    Invalid,
    BigInteger(BigInt),
    Add(ConstructableReal, ConstructableReal),
    Shift(ConstructableReal, i32),
    AssumedInt(ConstructableReal),
    Negated(ConstructableReal),
    Multiply(ConstructableReal, ConstructableReal),
    Inverted(ConstructableReal),
    Select(ConstructableReal, ConstructableReal, ConstructableReal),
}

#[derive(Clone, Debug)]
pub struct ConstructableReal {
    t: Box<ConstructableRealType>,
    pub min_prec: i32,
    pub max_appr: BigInt,
    pub appr_valid: bool,
}

impl ConstructableReal {
    ///  Must be defined in implementors of ConstructableReal
    ///  Most users can ignore the existence of this method, and will
    ///  not ever need to implement ConstructableReal.
    ///  Returns value / 2 ** precision rounded to an integer.
    ///  The error in the result is strictly < 1.
    ///  Informally, approximate(n) gives a scaled approximation
    ///  accurate to 2**n.
    ///  Implementations may safely assume that precision is
    ///  at least a factor of 8 away from overflow.
    fn approximate(&self, precision: i32) -> NumResult<BigInt> {
        match self.t.as_ref() {
            ConstructableRealType::Invalid => {
                panic!("Tried to approximate an invalid constructable real.")
            }
            ConstructableRealType::BigInteger(value) => Ok(scale(value.clone(), -precision)),
            ConstructableRealType::Add(op1, op2) => {
                // Args need to be evaluated so that each error is < 1/4 ulp.
                // Rounding error from the cale call is <= 1/2 ulp, so that
                // final error is < 1 ulp.
                Ok(scale(
                    op1.clone().get_appr(precision - 2)? + op2.clone().get_appr(precision - 2)?,
                    -2,
                ))
            }
            ConstructableRealType::Shift(op, count) => op.clone().get_appr(precision - count),
            ConstructableRealType::AssumedInt(cr) => {
                if precision >= 0 {
                    cr.clone().get_appr(precision)
                } else {
                    Ok(scale(cr.clone().get_appr(0)?, -precision))
                }
            }
            ConstructableRealType::Negated(op) => Ok(op.clone().get_appr(precision)?.neg()),
            ConstructableRealType::Multiply(op1, op2) => {
                let mut op1 = op1.clone();
                let mut op2 = op2.clone();
                let half_prec = (precision >> 1) - 1;
                let mut msd_op1 = op1.msd_n(half_prec)?;
                let mut msd_op2;
                if msd_op1 == i32::MIN {
                    msd_op2 = op2.msd_n(half_prec)?;
                    if msd_op2 == i32::MIN {
                        // Product is small enough that zero will do as an
                        // approximation.
                        return Ok(BigInt::zero());
                    } else {
                        // Swap them, so the larger operand (in absolute value)
                        // is first.
                        swap(&mut op1, &mut op2);
                        msd_op1 = msd_op2;
                    }
                }

                // msd_op1 is valid at this point.
                let prec2 = precision - msd_op1 - 3; // Precision needed for op2.
                // The appr. error is multiplied by at most
                // 2 ** (msd_op1 + 1)
                // Thus each approximation contributes 1/4 ulp
                // to the rounding error, and the final rounding adds
                // another 1/2 ulp.
                let appr2 = op2.get_appr(prec2)?;
                if appr2.sign() == Sign::NoSign {
                    return Ok(BigInt::zero());
                }

                msd_op2 = op2.known_msd();
                let prec1 = precision - msd_op2 - 3; // Precision needed for op1.
                let appr1 = op1.get_appr(prec1)?;
                let scale_digits = prec1 + prec2 - precision;
                Ok(scale(appr1 * appr2, scale_digits))
            }
            ConstructableRealType::Inverted(op) => {
                let mut op = op.clone();
                let msd = op.msd()?;
                let inv_msd = 1 - msd;
                let digits_needed = inv_msd - precision + 3;
                // Number of SIGNIFICANT digits needed for
                // argument, excl. msd position, which may
                // be fictitious, since msd routine can be
                // off by 1.  Roughly 1 extra digit is
                // needed since the relative error is the
                // same in the argument and result, but
                // this isn't quite the same as the number
                // of significant digits.  Another digit
                // is needed to compensate for slop in the
                // calculation.
                // One further bit is required, since the
                // final rounding introduces a 0.5 ulp
                // error.
                let prec_needed = msd - digits_needed;
                let log_scale_factor = -precision - prec_needed;
                if log_scale_factor < 0 {
                    return Ok(BigInt::zero());
                }
                let dividend = BigInt::one() << log_scale_factor;
                let scaled_divisor = op.get_appr(prec_needed)?;
                let abs_scaled_divisor = scaled_divisor.abs();
                let adj_dividend = dividend + (abs_scaled_divisor.clone() >> 1);
                // Adjustment so that final result is rounded.
                let result: BigInt = adj_dividend / abs_scaled_divisor;
                if scaled_divisor.sign() == Sign::Minus {
                    Ok(-result)
                } else {
                    Ok(result)
                }
            }
            ConstructableRealType::Select(selector, op1, op2) => {
                let selector_sign = selector.clone().get_appr(-20)?.sign();
                match selector_sign {
                    Sign::Minus => op1.clone().get_appr(precision),
                    Sign::Plus => op2.clone().get_appr(precision),
                    Sign::NoSign => {
                        let op1_appr = op1.clone().get_appr(precision - 1)?;
                        let op2_appr = op2.clone().get_appr(precision - 1)?;
                        let diff = (op1_appr.clone() - op2_appr.clone()).abs();
                        if diff <= BigInt::one() {
                            // close enough; use either
                            return Ok(scale(op1_appr, -1));
                        }

                        if selector.clone().sign()? == Sign::Minus {
                            Ok(scale(op1_appr, -1))
                        } else {
                            Ok(scale(op2_appr, -1))
                        }
                    }
                }
            }
        }
    }

    fn get_appr(&mut self, precision: i32) -> NumResult<BigInt> {
        check_prec(precision)?;
        if self.appr_valid && precision >= self.min_prec {
            Ok(scale(self.max_appr.clone(), self.min_prec - precision))
        } else {
            let result = self.approximate(precision);
            self.min_prec = precision;
            self.max_appr = result.clone()?;
            self.appr_valid = true;
            result
        }
    }

    /// Return the position of the msd.
    /// If x.msd() == n then
    /// 2**(n-1) < abs(x) < 2**(n+1)
    /// This initial version assumes that max_appr is valid
    /// and sufficiently removed from zero
    /// that the msd is determined.
    fn known_msd(&self) -> i32 {
        let length = if self.max_appr.sign() != Sign::Minus {
            self.max_appr.bits()
        } else {
            self.max_appr.abs().bits()
        } as i32;
        let first_digit = self.min_prec + length - 1;
        first_digit
    }

    /// This version may return i32::MIN if the correct
    /// answer is < n.
    fn msd_n(&mut self, n: i32) -> NumResult<i32> {
        if !self.appr_valid
            || self.max_appr <= BigInt::one() && self.max_appr >= BigInt::one().mul(-1)
        {
            self.get_appr(n - 1)?;
            if self.max_appr.abs() <= BigInt::one() {
                return Ok(i32::MIN);
            }
        }

        Ok(self.known_msd())
    }

    /// Functionally equivalent, but iteratively evaluates to higher
    /// precision.
    fn iter_msd(&mut self, n: i32) -> NumResult<i32> {
        let mut prec = 0;
        while prec > n + 30 {
            let msd = self.msd_n(prec)?;
            if msd != i32::MIN {
                return Ok(msd);
            }
            check_prec(prec)?;
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
    pub fn compare_to_relative(&mut self, x: &mut Self, r: i32, a: i32) -> NumResult<Ordering> {
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
    /// - r: Relative tolerance in bits
    /// - a: Absolute tolerance in bits
    pub fn compare_to_absolute(&mut self, x: &mut Self, a: i32) -> NumResult<Ordering> {
        let needed_prec = a - 1;
        let this_appr = self.get_appr(needed_prec)?;
        let x_appr = x.get_appr(needed_prec)?;
        let comp1 = this_appr.cmp(&x_appr.clone().add(&BigInt::one()));
        if comp1 == Ordering::Greater {
            return Ok(Ordering::Greater);
        }
        let comp2 = this_appr.cmp(&x_appr.sub(&BigInt::one()));
        if comp2 == Ordering::Less {
            return Ok(Ordering::Less);
        }
        Ok(Ordering::Equal)
    }

    /// Return -1 if self < x, or +1 if self > x.
    /// Should be called only if self != x.
    /// If self == x, this will not terminate correctly; typically it
    /// will run until it exhausts memory.
    /// If the two constructive reals may be equal, the two or 3 argument
    /// version of compare_to should be used.
    pub fn compare_to(&mut self, x: &mut Self) -> NumResult<Ordering> {
        let mut a = -20;
        loop {
            check_prec(a)?;
            let result = self.compare_to_absolute(x, a)?;
            if result != Ordering::Equal {
                return Ok(result);
            }
            a *= 2;
        }
    }

    /// Equivalent to <TT>compareTo(CR.valueOf(0), a)</tt>
    pub fn sign_precision(&mut self, a: i32) -> NumResult<Sign> {
        if self.appr_valid {
            let quick_try = self.max_appr.sign();
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
    pub fn sign(&mut self) -> NumResult<Sign> {
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
        let mut scaled_cr: ConstructableReal = if radix == 16 {
            (self.clone() << (4 * n) as i32)?
        } else {
            let scale_factor = BigInt::from(radix).pow(n);
            self.clone() * ConstructableReal::from(scale_factor)
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
    pub fn assume_int(self) -> ConstructableReal {
        ConstructableReal {
            t: Box::new(ConstructableRealType::AssumedInt(self)),
            ..ConstructableReal::default()
        }
    }

    /// The multiplicative inverse of a constructive real.
    /// x.inverse() is equivalent to ConstructableReal::from(1).divide(x).
    pub fn inverse(self) -> ConstructableReal {
        ConstructableReal {
            t: Box::new(ConstructableRealType::Inverted(self)),
            ..ConstructableReal::default()
        }
    }

    /// The real number x if self < 0, or y< otherwise.
    /// Requires x = y if self = 0.
    /// Since comparisons may diverge, this is often
    /// a useful alternative to conditionals.
    pub fn select(self, x: Self, y: Self) -> ConstructableReal {
        ConstructableReal {
            t: Box::new(ConstructableRealType::Select(self, x, y)),
            ..ConstructableReal::default()
        }
    }

    pub fn max(self, other: Self) -> ConstructableReal {
        (self.clone() - other.clone()).select(other, self)
    }

    pub fn min(self, other: Self) -> ConstructableReal {
        (self.clone() - other.clone()).select(self, other)
    }

    pub fn abs(self) -> ConstructableReal {
        self.clone().select(-self.clone(), self)
    }
}

impl Default for ConstructableReal {
    fn default() -> Self {
        ConstructableReal {
            t: Box::new(ConstructableRealType::Invalid),
            min_prec: 0,
            max_appr: BigInt::zero(),
            appr_valid: false,
        }
    }
}

impl From<BigInt> for ConstructableReal {
    fn from(n: BigInt) -> Self {
        ConstructableReal {
            t: Box::new(ConstructableRealType::BigInteger(n)),
            ..ConstructableReal::default()
        }
    }
}

impl From<i32> for ConstructableReal {
    fn from(value: i32) -> Self {
        ConstructableReal::from(BigInt::from(value))
    }
}

impl From<i64> for ConstructableReal {
    fn from(value: i64) -> Self {
        ConstructableReal::from(BigInt::from(value))
    }
}

impl From<u32> for ConstructableReal {
    fn from(value: u32) -> Self {
        ConstructableReal::from(BigInt::from(value))
    }
}

impl From<u64> for ConstructableReal {
    fn from(value: u64) -> Self {
        ConstructableReal::from(BigInt::from(value))
    }
}

impl TryFrom<f64> for ConstructableReal {
    type Error = NumError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_nan() {
            return Err(InternalError(ConstructableRealFromNan));
        }
        if value.is_infinite() {
            return Err(InternalError(ConstructableRealFromInf));
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

        let result = (ConstructableReal::from(mantissa) << exp as i32)?;
        Ok(if negative { -result } else { result })
    }
}

impl TryFrom<f32> for ConstructableReal {
    type Error = NumError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        ConstructableReal::try_from(value as f64)
    }
}

impl Display for ConstructableReal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.to_string(10, 10).unwrap_or("Error".to_string())
        )
    }
}

impl Add for ConstructableReal {
    type Output = ConstructableReal;

    fn add(self, rhs: Self) -> Self::Output {
        ConstructableReal {
            t: Box::new(ConstructableRealType::Add(self, rhs)),
            ..ConstructableReal::default()
        }
    }
}

impl Shl<i32> for ConstructableReal {
    type Output = NumResult<ConstructableReal>;

    fn shl(self, rhs: i32) -> Self::Output {
        check_prec(rhs)?;
        Ok(ConstructableReal {
            t: Box::new(ConstructableRealType::Shift(self, rhs)),
            ..ConstructableReal::default()
        })
    }
}

impl Shr<i32> for ConstructableReal {
    type Output = NumResult<ConstructableReal>;

    fn shr(self, rhs: i32) -> Self::Output {
        check_prec(rhs)?;
        Ok(ConstructableReal {
            t: Box::new(ConstructableRealType::Shift(self, -rhs)),
            ..ConstructableReal::default()
        })
    }
}

impl Sub for ConstructableReal {
    type Output = ConstructableReal;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul for ConstructableReal {
    type Output = ConstructableReal;

    fn mul(self, rhs: Self) -> Self::Output {
        ConstructableReal {
            t: Box::new(ConstructableRealType::Multiply(self, rhs)),
            ..ConstructableReal::default()
        }
    }
}

impl Div for ConstructableReal {
    type Output = ConstructableReal;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl Neg for ConstructableReal {
    type Output = ConstructableReal;

    fn neg(self) -> Self::Output {
        ConstructableReal {
            t: Box::new(ConstructableRealType::Negated(self)),
            ..ConstructableReal::default()
        }
    }
}

impl From<ConstructableReal> for NumResult<BigInt> {
    fn from(value: ConstructableReal) -> Self {
        value.clone().get_appr(0)
    }
}

impl From<ConstructableReal> for NumResult<i32> {
    fn from(value: ConstructableReal) -> Self {
        value.clone().get_appr(0).map(|n| n.to_i32().unwrap())
    }
}

impl From<ConstructableReal> for NumResult<f64> {
    fn from(value: ConstructableReal) -> Self {
        let mut value = value.clone();
        let my_msd = value.iter_msd(-1080 /* slightly > exp. range */)?;
        if my_msd == i32::MIN {return Ok(0.0)};
        let needed_prec = my_msd - 60;
        let scaled_int = value.get_appr(needed_prec)?.to_i64().unwrap() as f64;
        let may_underflow = needed_prec < -1000;
        let mut scaled_int_rep = scaled_int.to_bits();
        let exp_adj = if may_underflow { needed_prec + 96 } else{ needed_prec} as u64;
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

pub fn bound_log2(n: i32) -> i32 {
    let abs_n = n.abs();
    ((abs_n + 1) as f64).log(2.).ceil() as i32
}

/// Check that a precision is at least a factor of 8 away from
/// overflowng the integer used to hold a precision spec.
/// We generally perform this check early on, and then convince
/// ourselves that none of the operations performed on precisions
/// inside a function can generate an overflow.
pub fn check_prec(n: i32) -> NumResult<()> {
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

pub fn shift(k: BigInt, n: i32) -> BigInt {
    if n == 0 {
        k
    } else if n < 0 {
        k.div_floor(&BigInt::from(2).pow(n.abs() as u32))
    } else {
        k.mul(&BigInt::from(2).pow(n as u32))
    }
}

pub fn scale(k: BigInt, n: i32) -> BigInt {
    if n >= 0 {
        shift(k, n)
    } else {
        let adj_k = shift(k, n + 1).add(&BigInt::from(1));
        shift(adj_k, -1)
    }
}

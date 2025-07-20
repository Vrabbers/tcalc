use crate::angle_unit::AngleUnit;
use crate::maths_symbols::MathsSymbols;
use num::{BigInt, BigRational, BigUint, FromPrimitive, One, Signed, Zero};
use num::bigint::Sign;
use num::integer::Roots;
use num::traits::Inv;
use crate::bigint_extensions::BigIntExtensions;
use crate::error::NumResult;

/// Max integer for which extractSquare is guaranteed to be optimal.
/// We currently fail to so for 44 = 11*4, but succeed for all perfect squares*n, with n <= 10
/// and numerator and denominator size < EXTRACT_SQUARE_MAX_LEN.
pub static EXTRACT_SQUARE_MAX_OPT: i32 = 43;

pub trait RationalExtensions {
    fn whole_number_bits(&self) -> i32;
    fn bit_length(&self) -> i32;
    /// Return a string describing r*pi radians. If degrees is true describe the
    /// given number of radians in units of degrees.
    fn symbolic_pi_multiple(&self, ang: AngleUnit, subsuperscript: bool) -> String;
    /// Convert to readable String. Intended for output to user. More expensive, less useful for
    /// debugging than toString(). If mixed is true, convert improper fractions to mixed fractions. If
    /// subsuperscript is true, use subscript and superscript characters to represent the fraction.
    /// Subsuperscript relies on unicode characters intended for this purpose. Not internationalized.
    fn to_nice_string(&self, subsuperscript: bool) -> String;
    /// Is the supplied IS_SQRT property argument known to be minimal?
    fn irreducible_sqrt(&self) -> bool;
    /// Is the argument such that trig_func(pi*arg) can be simplified, or which should have
    /// been reduced to the (0, 1/2) interval.
    fn can_trig_be_reduced(&self) -> bool;
    /// Reduce a SIN_PI or TAN_PI argument to the interval [-1/2, 1.5).
    fn reduced_arg(&self) -> BigRational;
    /// Returns a truncated (rounded towards 0) representation of the result. Includes n digits to the
    /// right of the decimal point.
    ///
    /// Parameters:
    /// n: result precision, >= 0
    fn to_string_truncated(&self, n: u32) -> String;
    fn sign(&self) -> Sign;
    fn try_as_integer(&self) -> Option<BigInt>;
    /// Compute r^(1/n) exactly. Return None if it's irrational. n != 0. Note that this is arguably
    /// well-defined when r is negative and n is odd. We produce a meaningful answer in such
    /// cases. TODO: Try harder to produce a result.
    fn nth_root(&self, n: i32) -> NumResult<BigRational>;
    /// Return an equivalent fraction with a positive denominator
    fn denom_positive(self) -> (BigInt, BigInt);
    /// Return a pair p, such that p[0]^2 * p[1] = this.
    /// We try to maximize p[0]s numerator and denominator, but not very hard.
    /// This rational is assumed to be in reduced form.
    fn extract_square_reduced(&self) -> (BigRational, BigRational);
}

impl RationalExtensions for BigRational {
    fn whole_number_bits(&self) -> i32 {
        if self.is_zero() {
            i32::MIN
        } else {
            (self.numer().bits() - self.denom().bits()) as i32
        }
    }

    fn bit_length(&self) -> i32 {
        (self.numer().bits() + self.denom().bits()) as i32
    }

    fn symbolic_pi_multiple(&self, ang: AngleUnit, subsuperscript: bool) -> String {
        match ang {
            AngleUnit::Degrees => {
                let r_in_degrees = self * BigRational::new(1.into(), 180.into());
                r_in_degrees.to_nice_string(subsuperscript)
            }
            AngleUnit::Radians => {
                if self.denom().is_one() {
                    format!("{}{}", self.numer(), MathsSymbols::Pi)
                } else if subsuperscript && !self.numer().is_one() {
                    format!(
                        "{}{}",
                        self.to_nice_string(subsuperscript),
                        MathsSymbols::Pi
                    )
                } else {
                    format!(
                        "{}/{}{}",
                        if self.numer().is_one() {
                            "".to_string()
                        } else {
                            self.numer().to_string()
                        },
                        MathsSymbols::Pi,
                        self.denom()
                    )
                }
            }
            AngleUnit::Gradians => {
                todo!()
            }
        }
    }

    fn to_nice_string(&self, subsuperscript: bool) -> String {
        "to nice string".into()
    }

    fn irreducible_sqrt(&self) -> bool {
        self.numer().bits() <= 30
            && self.numer().abs() <= BigInt::from_i32(EXTRACT_SQUARE_MAX_OPT).unwrap()
            && self.denom().bits() <= 30
            && self.denom().abs() <= BigInt::from_i32(EXTRACT_SQUARE_MAX_OPT).unwrap()
    }

    fn can_trig_be_reduced(&self) -> bool {
        !self.is_positive()
            || self >= &BigRational::new(1.into(), 2.into())
            || self == &BigRational::new(1.into(), 3.into())
            || self == &BigRational::new(1.into(), 4.into())
            || self == &BigRational::new(1.into(), 6.into())
    }

    fn reduced_arg(&self) -> BigRational {
        if self >= &BigRational::new((-1).into(), 2.into())
            && self < &BigRational::new(3.into(), 2.into())
        {
            return self.clone();
        }

        let arg_plus_half = self.clone() + BigRational::new(1.into(), 2.into());
        let arg_ph_floor = arg_plus_half.floor().to_integer();
        let result_offset = arg_ph_floor & !BigInt::one();
        self - result_offset
    }

    fn to_string_truncated(&self, n: u32) -> String {
        let mut digits = (self.numer().abs() * BigInt::from_i32(10).unwrap().pow(n)
            / self.denom().abs())
        .to_string();

        let mut len = digits.len();
        if len < (n as usize) + 1 {
            digits = format!("{}{}", "0".repeat((n as usize) + 1 - len), digits);
            len = (n as usize) + 1;
        }

        format!(
            "{}{}.{}",
            if self.is_negative() { "-" } else { "" },
            digits[0..len - (n as usize)].to_string(),
            digits[len - (n as usize)..].to_string()
        )
    }

    fn sign(&self) -> Sign {
        if self.is_zero() {
            Sign::NoSign
        } else if self.is_positive() {
            Sign::Plus
        } else {
            Sign::Minus
        }
    }

    fn try_as_integer(&self) -> Option<BigInt> {
        if self.is_integer() {
            Some(self.to_integer())
        } else {
            None
        }
    }

    /// Return an equivalent fraction with a positive denominator
    fn denom_positive(self) -> (BigInt, BigInt) {
        if self.denom().is_positive() {
            (self.numer().clone(), self.denom().clone())
        } else {
            (-self.numer().clone(), -self.denom().clone())
        }
    }

    fn nth_root(&self, n: i32) -> NumResult<BigRational> {
        // Return non-null if numerator and denominator are small perfect squares.
        if n < 0 {
            return Ok(self.nth_root(-n)?.inv());
        }

        let r = self.clone().denom_positive();
        let numer = r.0.nth_root_safe(n as u32)?;
        let denom = r.1.nth_root_safe(n as u32)?;
        Ok(BigRational::new(numer, denom))
    }

    fn extract_square_reduced(&self) -> (BigRational, BigRational) {
        if self.is_zero() {
            (BigRational::zero(), BigRational::one())
        } else {
            let mut num_result = self.numer().clone().extract_square();
            let den_result = self.denom().clone().extract_square();
            if self.sign() == Sign::Minus {
                num_result.1 = -num_result.1;
            }

            (BigRational::new(num_result.0, den_result.0), BigRational::new(num_result.1, den_result.1))
        }
    }
}

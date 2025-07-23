use crate::angle_unit::AngleUnit;
use crate::bigint_extensions::BigIntExtensions;
use crate::error::NumResult;
use crate::maths_symbols::MathsSymbols;
use num::bigint::Sign;
use num::traits::Inv;
use num::{BigInt, BigRational, FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::str::FromStr;

/// Max integer for which extractSquare is guaranteed to be optimal.
/// We currently fail to so for 44 = 11*4, but succeed for all perfect squares*n, with n <= 10
/// and numerator and denominator size < EXTRACT_SQUARE_MAX_LEN.
pub static EXTRACT_SQUARE_MAX_OPT: i32 = 43;

/// Max bit length for attempting to extract square, so as to not take too much time.
/// Large enough so that computations on floating point numbers cannot easily overflow this.
pub static EXTRACT_SQUARE_MAX_LEN: u64 = 5000;

static MAX_SIZE: u64 = 10000; // total, in bits

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
    fn reduced_arg(&self) -> Option<BigRational>;
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
    /// Is this number too big for us to continue with rational arithmetic? We return false for
    /// integers on the assumption that we have no better fallback.
    fn too_big(&self) -> bool;
    /// Will extractSquareReduced guarantee that p[1] is not a perfect square?
    /// This rational is assumed to be in reduced form.
    fn extract_square_will_succeed(&self) -> bool;
    /// Return an approximation of the base 2 log of the absolute value.
    /// We assume this is nonzero.
    /// We try to be reasonably accurate around 1.  When in doubt we return 0.
    /// The result is either 0 or within 20% of the truth.
    fn appr_log_2_abs(&self) -> f64;
    /// Return the number of decimal digits to the right of the decimal point required to represent the
    /// argument exactly. Return usize::MAX if that's not possible. Never returns a value less
    /// than zero, even if r is a power of ten.
    fn digits_required(&self) -> usize;
    /// Create a BigRational from a string in decimal point form.
    /// This function is not l10n friendly. The string must be converted to a string with
    /// numerals 0-9 and possibly one decimal point.
    fn from_decimal_string(s: &str) -> Option<BigRational>;
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
        let nicer = self.clone().denom_positive();
        let mut num = nicer.0.abs();
        let den = nicer.1;
        let negative = num.is_negative();
        let mut whole = None;
        if den == BigInt::one() {
            whole = Some(num);
            num = BigInt::zero();
        }

        let mut result = if negative {
            if whole.is_none() && subsuperscript {
                MathsSymbols::SuperscriptMinus.to_string()
            } else {
                MathsSymbols::Minus.to_string()
            }
        } else {
            "".to_string()
        };

        if let Some(whole) = &whole {
            result = format!("{result}{whole}");
        }
        // num == 0 ==> whole non-null.
        if num.is_zero() {
            return result;
        }

        let num_string = num.to_string();
        let den_string = den.to_string();
        if whole.is_some() {
            // Need a separator.
            result = format!("{result} ");
        }

        format!("{result}{num_string}/{den_string}")
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

    fn reduced_arg(&self) -> Option<BigRational> {
        if self >= &BigRational::new((-1).into(), 2.into())
            && self < &BigRational::new(3.into(), 2.into())
        {
            return Some(self.clone());
        }

        let arg_plus_half = self.clone() + BigRational::new(1.into(), 2.into());
        if arg_plus_half.too_big() {
            return None;
        }
        let arg_ph_floor = arg_plus_half.floor().to_integer();
        let result_offset = arg_ph_floor & !BigInt::one();
        Some(self - result_offset)
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

        // Elide the decimal point if not required
        if n == 0 {
            format!(
                "{}{}",
                if self.is_negative() { "-" } else { "" },
                &digits[0..len - (n as usize)]
            )
        } else {
            format!(
                "{}{}.{}",
                if self.is_negative() { "-" } else { "" },
                &digits[0..len - (n as usize)],
                &digits[len - (n as usize)..]
            )
        }
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
            let mut num_result = self.numer().abs().clone().extract_square();
            let den_result = self.denom().abs().clone().extract_square();
            if self.sign() == Sign::Minus {
                num_result.1 = -num_result.1;
            }

            (
                BigRational::new(num_result.0, den_result.0),
                BigRational::new(num_result.1, den_result.1),
            )
        }
    }

    fn too_big(&self) -> bool {
        !self.denom() == BigInt::one() && (self.numer().bits() + self.denom().bits() > MAX_SIZE)
    }

    fn extract_square_will_succeed(&self) -> bool {
        // We take the absolute value before extracting the square. That may increase the length by 1.
        // Hence <, not <= .
        self.numer().bits() < EXTRACT_SQUARE_MAX_LEN && self.denom().bits() < EXTRACT_SQUARE_MAX_LEN
    }

    fn appr_log_2_abs(&self) -> f64 {
        let whole_bits = self.whole_number_bits();
        if whole_bits > 10 || whole_bits < -10 {
            // Bit lengths suffice for our purposes.
            whole_bits as f64
        } else {
            // Argument is in the vicinity of one. numerator and denominator are nonzero, but may be
            // individually huge.
            let quotient = (self.numer().to_f64().unwrap() / self.denom().to_f64().unwrap()).abs();
            if quotient.is_infinite() || quotient.is_nan() || quotient == 0.0 {
                // Zero quotient means denominator overflowed and is meaningless. Ignore.
                0.0
            } else {
                quotient.log(2.)
            }
        }
    }

    fn digits_required(&self) -> usize {
        let mut powers_of_two = 0; // Max power of 2 that divides denominator
        let mut powers_of_five = 0; // Max power of 5 that divides denominator

        // Try the easy case first to speed things up.
        if self.denom().is_one() {
            return 0;
        }

        let big_five = BigInt::from_i32(5).unwrap();

        let mut den = self.denom().clone();
        if den.bits() > MAX_SIZE {
            return usize::MAX;
        }
        while !den.bit(0) {
            powers_of_two += 1;
            den = den >> 1;
        }
        while (&den % &big_five).is_zero() {
            powers_of_five += 1;
            den = den.clone() / big_five.clone();
        }

        // If the denominator has a factor of other than 2 or 5 (the divisors of 10), the decimal
        // expansion does not terminate.  Multiplying the fraction by any number of powers of 10
        // will not cancel the denominator.  (Recall the fraction was in lowest terms to start
        // with.) Otherwise the powers of 10 we need to cancel the denominator is the larger of
        // powers_of_two and powers_of_five.
        if !den.is_one() && den != BigInt::from_i32(-1).unwrap() {
            return usize::MAX;
        }
        powers_of_two.max(powers_of_five)
    }

    fn from_decimal_string(s: &str) -> Option<BigRational> {
        // Ensure that only digits and one decimal point are included
        if s.is_empty() {
            return None;
        }

        let mut have_decimal = false;
        let mut is_first_char = true;
        for c in s.chars() {
            if !c.is_ascii_digit() && c != '.' && c != '-' && c != '+' {
                return None;
            } else if !is_first_char && (c == '-' || c == '+') {
                return None;
            } else if c == '.' {
                if have_decimal {
                    // Second decimal point
                    return None;
                }
                have_decimal = true;
            }
            is_first_char = false;
        }

        let mut parts = s.split('.');
        let before = parts.next().unwrap();
        let after = parts.next().unwrap_or("0");
        Some(BigRational::new(
            BigInt::from_str(format!("{before}{after}").as_str()).ok()?,
            BigInt::from_i32(10).unwrap().pow(after.len() as u32),
        ))
    }
}

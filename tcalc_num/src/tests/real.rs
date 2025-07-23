#![allow(dead_code, unused_macros, unused_imports)]

use std::str::FromStr;
use num::{BigRational, FromPrimitive};
use crate::angle_unit::AngleUnit;
use crate::constructive_real::ConstructiveReal;
use crate::error::DomainViolation::{LogarithmDomainViolation, NthRoot};
use crate::error::LogarithmDomainViolation::{LogOfNegative, LogOfZero};
use crate::error::NumError::DomainViolation;
use crate::maths_symbols::MathsSymbols::Sqrt;
use crate::real::constants::{ONE, PI};
use crate::real::Real;

#[test]
fn test_addition() {
    let one = ONE.clone();
    let two = (one.clone() + one.clone()).unwrap();
    let three = (two.clone() + one.clone()).unwrap();
    let five = (two.clone() + three.clone()).unwrap();

    assert_eq!("2", two.to_string_truncated_or_less(10).unwrap());
    assert_eq!("3", three.to_string_truncated_or_less(10).unwrap());
    assert_eq!("5", five.to_string_truncated_or_less(10).unwrap());

    // Test addition with negative numbers
    let negative_one = -one.clone();
    let zero = (one.clone() + negative_one.clone()).unwrap();
    assert_eq!("0", zero.to_string_truncated_or_less(10).unwrap());

    // Test addition with decimal numbers
    let half = Real::from_str("0.5").unwrap();
    let one_and_half = (one.clone() + half.clone()).unwrap();
    assert_eq!("1.5", one_and_half.to_string_truncated_or_less(10).unwrap());
}

#[test]
fn test_subtraction() {
    let one = ONE.clone();
    let two = (one.clone() + one.clone()).unwrap();
    let three = (two.clone() + one.clone()).unwrap();

    // Basic subtraction
    let result = (three.clone() - one.clone()).unwrap();
    assert_eq!("2", result.to_string_truncated_or_less(10).unwrap());

    let result = (three.clone() - two.clone()).unwrap();
    assert_eq!("1", result.to_string_truncated_or_less(10).unwrap());

    // Subtraction resulting in zero
    let zero = (two.clone() - two.clone()).unwrap();
    assert_eq!("0", zero.to_string_truncated_or_less(10).unwrap());

    // Subtraction resulting in negative
    let negative_one = (one.clone() - two.clone()).unwrap();
    assert_eq!("-1", negative_one.to_string_truncated_or_less(10).unwrap());

    // Test with decimal numbers
    let half = Real::from_str("0.5").unwrap();
    let result = (one.clone() - half.clone()).unwrap();
    assert_eq!("0.5", result.to_string_truncated_or_less(10).unwrap());
}

#[test]
fn test_multiplication() {
    let one = ONE.clone();
    let two = (one.clone() + one.clone()).unwrap();
    let three = (two.clone() + one.clone()).unwrap();

    // Basic multiplication
    let four = two.clone() * two.clone();
    assert_eq!("4", four.to_string_truncated_or_less(10).unwrap());

    let six = two.clone() * three.clone();
    assert_eq!("6", six.to_string_truncated_or_less(10).unwrap());

    // Multiplication by zero
    let zero = Real::from_str("0").unwrap();
    let result = two.clone() * zero.clone();
    assert_eq!("0", result.to_string_truncated_or_less(10).unwrap());

    // Multiplication by one
    let result = three.clone() * one.clone();
    assert_eq!("3", result.to_string_truncated_or_less(10).unwrap());

    // Multiplication with negative numbers
    let negative_two = -two.clone();
    let negative_six = three.clone() * negative_two.clone();
    assert_eq!("-6", negative_six.to_string_truncated_or_less(10).unwrap());

    // Multiplication with decimals
    let half = Real::from_str("0.5").unwrap();
    let result = two.clone() * half.clone();
    assert_eq!("1", result.to_string_truncated_or_less(10).unwrap());
}

#[test]
fn test_division() {
    let one = ONE.clone();
    let two = (one.clone() + one.clone()).unwrap();
    let four = two.clone() * two.clone();
    let eight = four.clone() * two.clone();

    // Basic division
    let result = (eight.clone() / four.clone()).unwrap();
    assert_eq!("2", result.to_string_truncated_or_less(10).unwrap());

    let result = (four.clone() / two.clone()).unwrap();
    assert_eq!("2", result.to_string_truncated_or_less(10).unwrap());

    // Division resulting in one
    let result = (two.clone() / two.clone()).unwrap();
    assert_eq!("1", result.to_string_truncated_or_less(10).unwrap());

    // Division resulting in fraction
    let result = (one.clone() / two.clone()).unwrap();
    assert_eq!("0.5", result.to_string_truncated_or_less(10).unwrap());

    // Division with negative numbers
    let negative_two = -two.clone();
    let result = (four.clone() / negative_two.clone()).unwrap();
    assert_eq!("-2", result.to_string_truncated_or_less(10).unwrap());

    let result = (negative_two.clone() / two.clone()).unwrap();
    assert_eq!("-1", result.to_string_truncated_or_less(10).unwrap());

    // Division of negative by negative (should be positive)
    let negative_four = -four.clone();
    let result = (negative_four.clone() / negative_two.clone()).unwrap();
    assert_eq!("2", result.to_string_truncated_or_less(10).unwrap());
}

#[test]
fn test_division_by_zero() {
    use crate::error::DomainViolation::DivisionByZero;
    use crate::error::NumError::DomainViolation;

    let one = ONE.clone();
    let two = (one.clone() + one.clone()).unwrap();
    let zero = Real::from_str("0").unwrap();

    // Test division by zero with positive number
    let result = one.clone() / zero.clone();
    assert_eq!(result.unwrap_err(), DomainViolation(DivisionByZero));

    // Test division by zero with negative number
    let negative_two = -two.clone();
    let result = negative_two / zero.clone();
    assert_eq!(result.unwrap_err(), DomainViolation(DivisionByZero));

    // Test zero divided by zero
    let result = zero.clone() / zero.clone();
    assert_eq!(result.unwrap_err(), DomainViolation(DivisionByZero));

    // Test that zero divided by non-zero works fine
    let result = (zero.clone() / one.clone()).unwrap();
    assert_eq!("0", result.to_string_truncated_or_less(10).unwrap());
}

#[test]
fn test_real_pi() {
    let pi = Real::new_from_cr(ConstructiveReal::pi());
    let tau = pi.clone() * Real::new_from_rational(BigRational::from_i32(2).unwrap());
    assert_eq!("3.1415926535897932384626433832795028841971", pi.to_string_truncated_or_less(40).unwrap());
    assert_eq!("6.2831853071795864769252867665590057683943", tau.to_string_truncated_or_less(40).unwrap());
    assert_eq!("2π", tau.to_nice_string(AngleUnit::Degrees, false).unwrap());
}

#[test]
fn test_sin() {
    // Test sin(90 deg)
    let ninety = Real::from_str("90").unwrap();
    let sin_ninety = ninety.degrees_to_radians().sin();
    assert_eq!("1", sin_ninety.unwrap().to_string_truncated_or_less(10).unwrap());

    // Test sin(180 deg)
    let one_eighty = Real::from_str("180").unwrap();
    let sin_one_eighty = one_eighty.degrees_to_radians().sin();
    assert_eq!("0", sin_one_eighty.unwrap().to_string_truncated_or_less(10).unwrap());

    // Test sin(-90 deg)
    let neg_ninety = Real::from_str("-90").unwrap();
    let sin_neg_ninety = neg_ninety.degrees_to_radians().sin();
    assert_eq!("-1", sin_neg_ninety.unwrap().to_string_truncated_or_less(10).unwrap());
}

#[test]
fn test_sqrt() {
    // Test sqrt(4)
    let four = Real::from_str("4").unwrap();
    let sqrt_four = four.sqrt().unwrap();
    assert_eq!("2", sqrt_four.to_string_truncated_or_less(10).unwrap());

    // Test sqrt(9)
    let nine = Real::from_str("9").unwrap();
    let sqrt_nine = nine.sqrt().unwrap();
    assert_eq!("3", sqrt_nine.to_string_truncated_or_less(10).unwrap());

    // Test sqrt(2)
    let sqrt_two = sqrt_four.sqrt().unwrap();
    assert_eq!("1.4142135623", sqrt_two.to_string_truncated_or_less(10).unwrap());

    // Test sqrt(-2)
    let neg_two = Real::from_str("-2").unwrap();
    let sqrt_neg_two = neg_two.sqrt();
    assert_eq!(sqrt_neg_two.unwrap_err(), DomainViolation(NthRoot(2.)))
}

#[test]
fn test_ln() {
    let one = ONE.clone();
    let two = (one.clone() + one.clone()).unwrap();

    // Test ln(1) = 0
    let ln_one = one.clone().ln().unwrap();
    assert_eq!("0", ln_one.to_string_truncated_or_less(10).unwrap());

    // Test ln(e) = 1 (where e is Euler's number)
    let e = one.clone().exp().unwrap();
    let ln_e = e.ln().unwrap();
    assert_eq!("1", ln_e.to_string_truncated_or_less(10).unwrap());

    // Test ln(e^2) = 2
    let e_squared = e.clone() * e.clone();
    let ln_e_squared = e_squared.ln().unwrap();
    assert_eq!("2", ln_e_squared.to_string_truncated_or_less(10).unwrap());

    // Test ln(2) ≈ 0.6931471805
    let ln_two = two.clone().ln().unwrap();
    assert_eq!("0.6931471805", ln_two.to_string_truncated_or_less(10).unwrap());

    // Test ln(10) ≈ 2.3025850929
    let ten = Real::from_str("10").unwrap();
    let ln_ten = ten.ln().unwrap();
    assert_eq!("2.3025850929", ln_ten.to_string_truncated_or_less(10).unwrap());

    // Test ln(0.5) = -ln(2)
    let half = Real::from_str("0.5").unwrap();
    let ln_half = half.ln().unwrap();
    let negative_ln_two = -ln_two;
    let ln_half_str = ln_half.to_string_truncated_or_less(10).unwrap();
    let neg_ln_two_str = negative_ln_two.to_string_truncated_or_less(10).unwrap();
    assert_eq!(ln_half_str, neg_ln_two_str, "ln(0.5) should equal -ln(2)");

    // Test ln of negative number should fail
    let negative_one = -one.clone();
    let ln_negative = negative_one.cr_value().ln();
    assert_eq!(ln_negative.unwrap_err(), DomainViolation(LogarithmDomainViolation(LogOfNegative)));

    // Test ln(0) should fail (approaches negative infinity)
    let zero = Real::from_str("0").unwrap();
    let ln_zero = zero.ln();
    assert_eq!(ln_zero.unwrap_err(), DomainViolation(LogarithmDomainViolation(LogOfZero)));
}

#[test]
fn test_log() {
    let one = ONE.clone();
    let two = (one.clone() + one.clone()).unwrap();
    let ten = Real::from_str("10").unwrap();

    // Test log_10(1) = 0
    // log_10(x) = ln(x) / ln(10)
    let ln_one = one.clone().ln().unwrap();
    let ln_ten = ten.clone().ln().unwrap();
    let log10_one = (ln_one / ln_ten.clone()).unwrap();
    assert_eq!("0", log10_one.to_string_truncated_or_less(10).unwrap());

    // Test log_10(10) = 1
    let ln_ten_again = ten.clone().ln().unwrap();
    let log10_ten = (ln_ten_again / ln_ten.clone()).unwrap();
    assert_eq!("1", log10_ten.to_string_truncated_or_less(10).unwrap());

    // Test log_10(100) = 2
    let hundred = Real::from_str("100").unwrap();
    let ln_hundred = hundred.ln().unwrap();
    let log10_hundred = (ln_hundred / ln_ten.clone()).unwrap();
    assert_eq!("2", log10_hundred.to_string_truncated_or_less(10).unwrap());

    // Test log_10(0.1) = -1
    let point_one = Real::from_str("0.1").unwrap();
    let ln_point_one = point_one.ln().unwrap();
    let log10_point_one = (ln_point_one / ln_ten.clone()).unwrap();
    assert_eq!("-1", log10_point_one.to_string_truncated_or_less(10).unwrap());

    // Test log_2(8) = 3 (since 2^3 = 8)
    let eight = Real::from_str("8").unwrap();
    let ln_eight = eight.ln().unwrap();
    let ln_two = two.clone().ln().unwrap();
    let log2_eight = (ln_eight / ln_two.clone()).unwrap();
    assert_eq!("3", log2_eight.to_string_truncated_or_less(10).unwrap());

    // Test log_2(0.5) = -1 (since 2^(-1) = 0.5)
    let half = Real::from_str("0.5").unwrap();
    let ln_half = half.ln().unwrap();
    let log2_half = (ln_half / ln_two.clone()).unwrap();
    assert_eq!("-1", log2_half.to_string_truncated_or_less(10).unwrap());

    // Test log_e(e) = 1 (natural logarithm)
    let e = one.clone().exp().unwrap();
    let ln_e = e.clone().ln().unwrap();
    let ln_e_base = e.ln().unwrap();
    let log_e_e = (ln_e / ln_e_base).unwrap();
    assert_eq!("1", log_e_e.to_string_truncated_or_less(10).unwrap());

    // Test logarithm of negative number should fail
    let negative_two = -two.clone();
    let ln_negative = negative_two.cr_value().ln();
    assert_eq!(ln_negative.unwrap_err(), DomainViolation(LogarithmDomainViolation(LogOfNegative)));

    // Test logarithm of zero should fail
    let zero = Real::from_str("0").unwrap();
    let ln_zero = zero.ln();
    assert_eq!(ln_zero.unwrap_err(), DomainViolation(LogarithmDomainViolation(LogOfZero)));
}
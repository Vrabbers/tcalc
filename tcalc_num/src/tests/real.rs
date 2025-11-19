#![allow(dead_code, unused_macros, unused_imports)]

use crate::angle_unit::AngleUnit;
use crate::constructive_real::ConstructiveReal;
use crate::error::DomainViolation::{LogarithmDomainViolation, NthRoot, TanDomainViolation};
use crate::error::LogarithmDomainViolation::{LogOfNegative, LogOfZero};
use crate::error::NumError::DomainViolation;
use crate::maths_symbols::MathsSymbols::Sqrt;
use crate::number::Number;
use crate::real::Real;
use crate::real::constants::{ONE, PI, ZERO};
use num::{BigRational, FromPrimitive};
use std::str::FromStr;

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
    let four = (two.clone() * two.clone()).unwrap();
    assert_eq!("4", four.to_string_truncated_or_less(10).unwrap());

    let six = (two.clone() * three.clone()).unwrap();
    assert_eq!("6", six.to_string_truncated_or_less(10).unwrap());

    // Multiplication by zero
    let zero = Real::from_str("0").unwrap();
    let result = (two.clone() * zero.clone()).unwrap();
    assert_eq!("0", result.to_string_truncated_or_less(10).unwrap());

    // Multiplication by one
    let result = (three.clone() * one.clone()).unwrap();
    assert_eq!("3", result.to_string_truncated_or_less(10).unwrap());

    // Multiplication with negative numbers
    let negative_two = -two.clone();
    let negative_six = (three.clone() * negative_two.clone()).unwrap();
    assert_eq!("-6", negative_six.to_string_truncated_or_less(10).unwrap());

    // Multiplication with decimals
    let half = Real::from_str("0.5").unwrap();
    let result = (two.clone() * half.clone()).unwrap();
    assert_eq!("1", result.to_string_truncated_or_less(10).unwrap());
}

#[test]
fn test_division() {
    let one = ONE.clone();
    let two = (one.clone() + one.clone()).unwrap();
    let four = (two.clone() * two.clone()).unwrap();
    let eight = (four.clone() * two.clone()).unwrap();

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
    let tau = (pi.clone() * Real::new_from_rational(BigRational::from_i32(2).unwrap())).unwrap();
    assert_eq!(
        "3.1415926535897932384626433832795028841971",
        pi.to_string_truncated_or_less(40).unwrap()
    );
    assert_eq!(
        "6.2831853071795864769252867665590057683943",
        tau.to_string_truncated_or_less(40).unwrap()
    );
    assert_eq!("2π", tau.to_nice_string(AngleUnit::Degrees, false).unwrap());
}

#[test]
fn test_big_pi() {
    let pi = Real::new_from_cr(ConstructiveReal::pi());
    assert_eq!(
        include_str!("pi-million.txt"),
        pi.to_string_truncated_or_less(51_197).unwrap()
    );
}

#[test]
fn test_sin() {
    // Test sin(0 deg)
    let sin_zero = ZERO.clone().sin();
    assert_eq!(
        "0",
        sin_zero.unwrap().to_string_truncated_or_less(10).unwrap()
    );

    // Test sin(90 deg)
    let ninety = Real::from_str("90").unwrap();
    let sin_ninety = (ninety.degrees_to_radians()).unwrap().sin();
    assert_eq!(
        "1",
        sin_ninety.unwrap().to_string_truncated_or_less(10).unwrap()
    );

    // Test sin(180 deg)
    let one_eighty = Real::from_str("180").unwrap();
    let sin_one_eighty = one_eighty.degrees_to_radians().unwrap().sin();
    assert_eq!(
        "0",
        sin_one_eighty
            .unwrap()
            .to_string_truncated_or_less(10)
            .unwrap()
    );

    // Test sin(-90 deg)
    let neg_ninety = Real::from_str("-90").unwrap();
    let sin_neg_ninety = neg_ninety.degrees_to_radians().unwrap().sin().unwrap();
    assert_eq!(
        "-1",
        sin_neg_ninety.to_string_truncated_or_less(10).unwrap()
    );

    // sin(270) = sin(-90)
    let two_seventy = Real::from_str("270").unwrap();
    let sin_two_seventy = two_seventy.degrees_to_radians().unwrap().sin().unwrap();
    assert!(sin_neg_ninety.definitely_equals(&sin_two_seventy).unwrap());

    // sin(30) = 0.5
    let thirty = Real::from_str("30").unwrap();
    let sin_thirty = thirty.degrees_to_radians().unwrap().sin().unwrap();
    assert_eq!("0.5", sin_thirty.to_string_truncated_or_less(10).unwrap());

    // sin(45) = sqrt(2)/2
    let forty_five = Real::from_str("45").unwrap();
    let sin_forty_five = forty_five.degrees_to_radians().unwrap().sin().unwrap();
    let two = Real::from_str("2").unwrap();
    let sqrt_two_on_two = (two.clone().sqrt().unwrap() / two.clone()).unwrap();
    assert!(sin_forty_five.definitely_equals(&sqrt_two_on_two).unwrap());

    // sin(60) = sqrt(3)/2
    let sixty = Real::from_str("60").unwrap();
    let sin_sixty = sixty.degrees_to_radians().unwrap().sin().unwrap();
    let three = Real::from_str("3").unwrap();
    let sqrt_three_on_two = (three.clone().sqrt().unwrap() / two.clone()).unwrap();
    assert!(sin_sixty.definitely_equals(&sqrt_three_on_two).unwrap());

    // sin(360 + k) = sin(k)
    for k in 0..359 {
        let number_1 = Real::new_from_rational(BigRational::from_i32(k).unwrap())
            .degrees_to_radians()
            .unwrap()
            .sin()
            .unwrap();
        let number_2 = Real::new_from_rational(BigRational::from_i32(k + 360).unwrap())
            .degrees_to_radians()
            .unwrap()
            .sin()
            .unwrap();
        assert!(number_1.definitely_equals(&number_2).unwrap());
    }

    // sin(1) ≈ 0.01745241
    let sin_one = Real::from_str("1").unwrap().sin().unwrap();
    assert_eq!(sin_one.to_string_truncated_or_less(8).unwrap(), "0.01745241");
}

#[test]
fn test_cos() {
    // Test cos(0 deg)
    let cos_zero = ZERO.clone().cos();
    assert_eq!(
        "1",
        cos_zero.unwrap().to_string_truncated_or_less(10).unwrap()
    );

    // Test cos(90 deg)
    let ninety = Real::from_str("90").unwrap();
    let cos_ninety = ninety.degrees_to_radians().unwrap().cos();
    assert_eq!(
        "0",
        cos_ninety.unwrap().to_string_truncated_or_less(10).unwrap()
    );

    // Test cos(180 deg)
    let one_eighty = Real::from_str("180").unwrap();
    let cos_one_eighty = one_eighty.degrees_to_radians().unwrap().cos();
    assert_eq!(
        "-1",
        cos_one_eighty
            .unwrap()
            .to_string_truncated_or_less(10)
            .unwrap()
    );

    // Test cos(-90 deg)
    let neg_ninety = Real::from_str("-90").unwrap();
    let cos_neg_ninety = neg_ninety.degrees_to_radians().unwrap().cos().unwrap();
    assert_eq!("0", cos_neg_ninety.to_string_truncated_or_less(10).unwrap());

    // cos(270) = cos(-90)
    let two_seventy = Real::from_str("270").unwrap();
    let cos_two_seventy = two_seventy.degrees_to_radians().unwrap().cos().unwrap();
    assert!(cos_neg_ninety.definitely_equals(&cos_two_seventy).unwrap());

    // cos(60) = 0.5
    let sixty = Real::from_str("60").unwrap();
    let cos_sixty = sixty.degrees_to_radians().unwrap().cos().unwrap();
    assert_eq!("0.5", cos_sixty.to_string_truncated_or_less(10).unwrap());

    // cos(45) = sqrt(2)/2
    let forty_five = Real::from_str("45").unwrap();
    let cos_forty_five = forty_five.degrees_to_radians().unwrap().cos().unwrap();
    let two = Real::from_str("2").unwrap();
    let sqrt_two_on_two = (two.clone().sqrt().unwrap() / two.clone()).unwrap();
    assert!(cos_forty_five.definitely_equals(&sqrt_two_on_two).unwrap());

    // cos(30) = sqrt(3)/2
    let thirty = Real::from_str("30").unwrap();
    let cos_thirty = thirty.degrees_to_radians().unwrap().cos().unwrap();
    let three = Real::from_str("3").unwrap();
    let sqrt_three_on_two = (three.clone().sqrt().unwrap() / two.clone()).unwrap();
    assert!(cos_thirty.definitely_equals(&sqrt_three_on_two).unwrap());

    // cos(360 + k) = cos(k)
    for k in 0..359 {
        let number_1 = Real::new_from_rational(BigRational::from_i32(k).unwrap())
            .degrees_to_radians()
            .unwrap()
            .cos()
            .unwrap();
        let number_2 = Real::new_from_rational(BigRational::from_i32(k + 360).unwrap())
            .degrees_to_radians()
            .unwrap()
            .cos()
            .unwrap();
        assert!(number_1.definitely_equals(&number_2).unwrap());
    }
}

#[test]
fn test_tan() {
    // Test tan(0 deg)
    let tan_zero = ZERO.clone().tan();
    assert_eq!(
        "0",
        tan_zero.unwrap().to_string_truncated_or_less(10).unwrap()
    );

    // Test tan(90 deg)
    let ninety = Real::from_str("90").unwrap();
    let tan_ninety = ninety.degrees_to_radians().unwrap().tan();
    assert_eq!(tan_ninety.unwrap_err(), DomainViolation(TanDomainViolation));

    // tan(30) = sqrt(3)/3
    let thirty = Real::from_str("30").unwrap();
    let tan_thirty = thirty.degrees_to_radians().unwrap().tan().unwrap();
    let three = Real::from_str("3").unwrap();
    let sqrt_three_on_three = (three.clone().sqrt().unwrap() / three.clone()).unwrap();
    assert!(tan_thirty.definitely_equals(&sqrt_three_on_three).unwrap());

    // tan(45) = 1
    let forty_five = Real::from_str("45").unwrap();
    let tan_forty_five = forty_five.degrees_to_radians().unwrap().tan().unwrap();
    assert_eq!("1", tan_forty_five.to_string_truncated_or_less(10).unwrap());

    // tan(60) = sqrt(3)
    let sixty = Real::from_str("60").unwrap();
    let tan_sixty = sixty.degrees_to_radians().unwrap().tan().unwrap();
    let sqrt_three = three.clone().sqrt().unwrap();
    assert!(tan_sixty.definitely_equals(&sqrt_three).unwrap());

    // tan(360 + k) = tan(k), k % 360 != 90, 270
    for k in 0..359 {
        if k % 360 == 90 || k % 360 == 270 {
            // Tan is undefined for these inputs
            continue;
        }
        let number_1 = Real::new_from_rational(BigRational::from_i32(k).unwrap())
            .degrees_to_radians()
            .unwrap()
            .tan()
            .unwrap();
        let number_2 = Real::new_from_rational(BigRational::from_i32(k + 360).unwrap())
            .degrees_to_radians()
            .unwrap()
            .tan()
            .unwrap();
        assert!(number_1.definitely_equals(&number_2).unwrap());
    }
}

#[test]
fn test_asin() {
    // asin(sin(k)) = k, -90 <= k <= 90
    for k in -90..90 {
        let number_1 = Real::new_from_rational(BigRational::from_i32(k).unwrap())
            .degrees_to_radians()
            .unwrap()
            .sin()
            .unwrap()
            .asin()
            .unwrap();
        let number_2 = Real::new_from_rational(BigRational::from_i32(k % 360).unwrap())
            .degrees_to_radians()
            .unwrap();
        assert!(
            number_1.definitely_equals(&number_2).unwrap(),
            "asin(sin({k} deg)) = {}, {} deg = {}",
            number_1.to_string_truncated_or_less(10).unwrap(),
            k % 360,
            number_2.to_string_truncated_or_less(10).unwrap()
        );
    }
}

#[test]
fn test_acos() {
    // acos(cos(k)) = k, 0 <= k <= 180
    for k in 0..180 {
        let number_1 = Real::new_from_rational(BigRational::from_i32(k).unwrap())
            .degrees_to_radians()
            .unwrap()
            .cos()
            .unwrap()
            .acos()
            .unwrap();
        let number_2 = Real::new_from_rational(BigRational::from_i32(k % 360).unwrap())
            .degrees_to_radians()
            .unwrap();
        assert!(
            number_1.definitely_equals(&number_2).unwrap(),
            "acos(cos({k} deg)) = {}, {} deg = {}",
            number_1.to_string_truncated_or_less(10).unwrap(),
            k % 360,
            number_2.to_string_truncated_or_less(10).unwrap()
        );
    }
}

#[test]
fn test_atan() {
    // atan(tan(k)) = k, -90 < k < 90
    for k in -89..89 {
        let number_1 = Real::new_from_rational(BigRational::from_i32(k).unwrap())
            .degrees_to_radians()
            .unwrap()
            .tan()
            .unwrap()
            .atan()
            .unwrap();
        let number_2 = Real::new_from_rational(BigRational::from_i32(k % 360).unwrap())
            .degrees_to_radians()
            .unwrap();
        assert!(
            number_1.definitely_equals(&number_2).unwrap(),
            "atan(tan({k} deg)) = {}, {} deg = {}",
            number_1.to_string_truncated_or_less(10).unwrap(),
            k % 360,
            number_2.to_string_truncated_or_less(10).unwrap()
        );
    }
}

#[test]
fn test_sqrt() {
    // Test sqrt(4)
    let four = Real::from_str("4").unwrap();
    let sqrt_four = four.clone().sqrt().unwrap();
    assert_eq!("2", sqrt_four.to_string_truncated_or_less(10).unwrap());

    // Test sqrt(9)
    let nine = Real::from_str("9").unwrap();
    let sqrt_nine = nine.sqrt().unwrap();
    assert_eq!("3", sqrt_nine.to_string_truncated_or_less(10).unwrap());

    // Test sqrt(2)
    let sqrt_two = sqrt_four.clone().sqrt().unwrap();
    assert_eq!(
        "1.4142135623",
        sqrt_two.to_string_truncated_or_less(10).unwrap()
    );

    // Test sqrt(-2)
    let neg_two = Real::from_str("-2").unwrap();
    let sqrt_neg_two = neg_two.sqrt();
    assert_eq!(sqrt_neg_two.unwrap_err(), DomainViolation(NthRoot(2.)));

    // Test sqrt(e^4) = e^2
    let sqrt_exp_four = four.clone().exp().unwrap().sqrt().unwrap();
    let exp_two = sqrt_four.clone().exp().unwrap();
    assert!(sqrt_exp_four.definitely_equals(&exp_two).unwrap());

    // sqrt(k^2) = abs(k)
    for k in -1000..1000 {
        let number = Real::new_from_rational(BigRational::from_i32(k).unwrap());
        let sqrt_number_squared = number
            .clone()
            .pow(Real::new_from_rational(BigRational::from_i32(2).unwrap()))
            .unwrap()
            .sqrt()
            .unwrap();
        let abs_number = number.abs().unwrap();
        assert!(sqrt_number_squared.definitely_equals(&abs_number).unwrap());
    }
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
    let e_squared = (e.clone() * e.clone()).unwrap();
    let ln_e_squared = e_squared.ln().unwrap();
    assert_eq!("2", ln_e_squared.to_string_truncated_or_less(10).unwrap());

    // Test ln(2) ≈ 0.6931471805
    let ln_two = two.clone().ln().unwrap();
    assert_eq!(
        "0.6931471805",
        ln_two.to_string_truncated_or_less(10).unwrap()
    );

    // Test ln(10) ≈ 2.3025850929
    let ten = Real::from_str("10").unwrap();
    let ln_ten = ten.ln().unwrap();
    assert_eq!(
        "2.3025850929",
        ln_ten.to_string_truncated_or_less(10).unwrap()
    );

    // Test ln(0.5) = -ln(2)
    let half = Real::from_str("0.5").unwrap();
    let ln_half = half.ln().unwrap();
    let negative_ln_two = -ln_two;
    let ln_half_str = ln_half.to_string_truncated_or_less(10).unwrap();
    let neg_ln_two_str = negative_ln_two.to_string_truncated_or_less(10).unwrap();
    assert_eq!(ln_half_str, neg_ln_two_str, "ln(0.5) should equal -ln(2)");

    // Test ln of negative number should fail
    let negative_one = -one.clone();
    let ln_negative = negative_one.ln();
    assert_eq!(
        ln_negative.unwrap_err(),
        DomainViolation(LogarithmDomainViolation(LogOfNegative))
    );

    // Test ln(0) should fail (approaches negative infinity)
    let zero = Real::from_str("0").unwrap();
    let ln_zero = zero.ln();
    assert_eq!(
        ln_zero.unwrap_err(),
        DomainViolation(LogarithmDomainViolation(LogOfZero))
    );
}

#[test]
fn test_log() {
    let one = ONE.clone();
    let two = (one.clone() + one.clone()).unwrap();
    let ten = Real::from_str("10").unwrap();

    // Test log_10(1) = 0
    // log_10(x) = ln(x) / ln(10)
    let log10_one = one.clone().log().unwrap();
    assert_eq!("0", log10_one.to_string_truncated_or_less(10).unwrap());

    // Test log_10(10) = 1
    let log10_ten = ten.clone().log().unwrap();
    assert_eq!("1", log10_ten.to_string_truncated_or_less(10).unwrap());

    // Test log_10(100) = 2
    let hundred = Real::from_str("100").unwrap();
    let log10_hundred = hundred.log().unwrap();
    assert_eq!("2", log10_hundred.to_string_truncated_or_less(10).unwrap());

    // Test log_10(0.1) = -1
    let point_one = Real::from_str("0.1").unwrap();
    let log10_point_one = point_one.log().unwrap();
    assert_eq!(
        "-1",
        log10_point_one.to_string_truncated_or_less(10).unwrap()
    );

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

    // Test logarithm of negative number should fail
    let negative_two = -two.clone();
    let log_negative = negative_two.log();
    assert_eq!(
        log_negative.unwrap_err(),
        DomainViolation(LogarithmDomainViolation(LogOfNegative))
    );

    // Test logarithm of zero should fail
    let zero = Real::from_str("0").unwrap();
    let log_zero = zero.log();
    assert_eq!(
        log_zero.unwrap_err(),
        DomainViolation(LogarithmDomainViolation(LogOfZero))
    );
}

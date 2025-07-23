#![allow(dead_code, unused_macros, unused_imports)]

use std::str::FromStr;
use num::{BigRational, FromPrimitive};
use crate::angle_unit::AngleUnit;
use crate::constructive_real::ConstructiveReal;
use crate::error::DomainViolation::NthRoot;
use crate::error::NumError::DomainViolation;
use crate::maths_symbols::MathsSymbols::Sqrt;
use crate::real::constants::{ONE, PI};
use crate::real::Real;

#[test]
fn test_basic() {
    let one = ONE.clone();
    let two = (one.clone() + one.clone()).unwrap();
    assert_eq!("2", two.to_string_truncated_or_less(10).unwrap());
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
    let ninety = Real::from_str("90").unwrap();
    let sin_ninety = ninety.degrees_to_radians().sin();
    assert_eq!("1", sin_ninety.unwrap().to_string_truncated_or_less(10).unwrap());

    let one_eighty = Real::from_str("180").unwrap();
    let sin_one_eighty = one_eighty.degrees_to_radians().sin();
    assert_eq!("0", sin_one_eighty.unwrap().to_string_truncated_or_less(10).unwrap());

    let neg_ninety = Real::from_str("-90").unwrap();
    let sin_neg_ninety = neg_ninety.degrees_to_radians().sin();
    assert_eq!("-1", sin_neg_ninety.unwrap().to_string_truncated_or_less(10).unwrap());
}

#[test]
fn test_sqrt() {
    let four = Real::from_str("4").unwrap();
    let sqrt_four = four.sqrt().unwrap();
    assert_eq!("2", sqrt_four.to_string_truncated_or_less(10).unwrap());

    let nine = Real::from_str("9").unwrap();
    let sqrt_nine = nine.sqrt().unwrap();
    assert_eq!("3", sqrt_nine.to_string_truncated_or_less(10).unwrap());

    let sqrt_two = sqrt_four.sqrt().unwrap();
    assert_eq!("1.4142135623", sqrt_two.to_string_truncated_or_less(10).unwrap());
    
    let neg_two = Real::from_str("-2").unwrap();
    let sqrt_neg_two = neg_two.sqrt();
    assert_eq!(sqrt_neg_two.unwrap_err(), DomainViolation(NthRoot(2.)))
}
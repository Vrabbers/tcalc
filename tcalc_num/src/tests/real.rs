use num::{BigRational, FromPrimitive};
use crate::angle_unit::AngleUnit;
use crate::constructive_real::ConstructiveReal;
use crate::real::constants::ONE;
use crate::real::Real;

#[test]
fn test_basic() {
    let one = ONE.clone();
    let two = one.clone() + one.clone();
    assert_eq!("2", two.unwrap().to_string());
}

#[test]
fn test_real_pi() {
    let pi = Real::new_from_cr(ConstructiveReal::pi());
    let tau = pi.clone() * Real::new_from_rational(BigRational::from_i32(2).unwrap());
    assert_eq!("3.1415926535897932384626433832795028841971", pi.to_string_truncated(40).unwrap());
    assert_eq!("6.2831853071795864769252867665590057683943", tau.to_string_truncated(40).unwrap());
    assert_eq!("2π", tau.to_nice_string(AngleUnit::Degrees, false).unwrap());
}
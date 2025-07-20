use crate::constructive_real;
use crate::real::Real;
use num::{BigRational, FromPrimitive, One, Zero};
use std::clone::Clone;
use std::convert::Into;
use std::ops::Mul;
use std::sync::LazyLock;

pub static PI: LazyLock<Real> =
    LazyLock::new(|| Real::new_from_cr(constructive_real::constants::PI.clone()));
pub static E: LazyLock<Real> =
    LazyLock::new(|| Real::new_from_cr(constructive_real::constants::E.clone()));
pub static ZERO: LazyLock<Real> = LazyLock::new(|| Real::new_from_rational(BigRational::zero()));
pub static ONE: LazyLock<Real> = LazyLock::new(|| Real::new_from_rational(BigRational::one()));
pub static M_ONE: LazyLock<Real> =
    LazyLock::new(|| Real::new_from_rational(BigRational::from_i32(-1).unwrap()));
pub static TWO: LazyLock<Real> =
    LazyLock::new(|| Real::new_from_rational(BigRational::from_i32(2).unwrap()));
pub static HALF: LazyLock<Real> =
    LazyLock::new(|| Real::new_from_rational(BigRational::new(1.into(), 2.into())));
pub static M_HALF: LazyLock<Real> =
    LazyLock::new(|| Real::new_from_rational(BigRational::new((-1).into(), 2.into())));
pub static TEN: LazyLock<Real> =
    LazyLock::new(|| Real::new_from_rational(BigRational::from_i32(10).unwrap()));
pub static RADIANS_PER_DEGREE: LazyLock<Real> = LazyLock::new(|| {
    Real::new_from_rat_cr(
        BigRational::new(1.into(), 180.into()),
        constructive_real::constants::PI.clone(),
    )
});
pub static LN_10: LazyLock<Real> =
    LazyLock::new(|| Real::new_from_cr(constructive_real::constants::LN_10.clone()));
pub static HALF_SQRT_2: LazyLock<Real> = LazyLock::new(|| {
    Real::new_from_rat_cr(
        BigRational::new(1.into(), 2.into()),
        constructive_real::constants::SQRT_2.clone(),
    )
});
pub static SQRT_3: LazyLock<Real> = LazyLock::new(|| {
    Real::new_from_cr(
        constructive_real::constants::SQRT_3.clone(),
    )
});
pub static HALF_SQRT_3: LazyLock<Real> = LazyLock::new(|| {
    Real::new_from_rat_cr(
        BigRational::new(1.into(), 2.into()),
        constructive_real::constants::SQRT_3.clone(),
    )
});
pub static THIRD_SQRT_3: LazyLock<Real> = LazyLock::new(|| {
    Real::new_from_rat_cr(
        BigRational::new(1.into(), 3.into()),
        constructive_real::constants::SQRT_3.clone(),
    )
});
pub static PI_OVER_2: LazyLock<Real> = LazyLock::new(|| {
    Real::new_from_rat_cr(
        BigRational::new(1.into(), 2.into()),
        constructive_real::constants::PI.clone(),
    )
});

pub static PI_OVER_3: LazyLock<Real> = LazyLock::new(|| {
    Real::new_from_rat_cr(
        BigRational::new(1.into(), 3.into()),
        constructive_real::constants::PI.clone(),
    )
});
pub static PI_OVER_4: LazyLock<Real> = LazyLock::new(|| {
    Real::new_from_rat_cr(
        BigRational::new(1.into(), 4.into()),
        constructive_real::constants::PI.clone(),
    )
});
pub static PI_OVER_6: LazyLock<Real> = LazyLock::new(|| {
    Real::new_from_rat_cr(
        BigRational::new(1.into(), 6.into()),
        constructive_real::constants::PI.clone(),
    )
});
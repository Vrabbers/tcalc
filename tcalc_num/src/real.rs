pub mod constants;
mod cr_property;

use crate::constructive_real::ConstructiveReal;
use num::bigint::Sign;
use num::{BigInt, BigRational, FromPrimitive, One, Signed};

#[derive(Debug, Clone)]
pub struct Real {
    rat: BigRational,
    cr: ConstructiveReal,
}

impl Real {
    pub fn new(rat: BigRational, cr: ConstructiveReal) -> Self {
        Self { rat, cr }
    }

    pub fn new_from_cr(cr: ConstructiveReal) -> Self {
        Self {
            rat: BigRational::one(),
            cr,
        }
    }

    pub fn new_from_rational(rat: BigRational) -> Self {
        Self {
            rat,
            cr: ConstructiveReal::from(1),
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

/// Is the argument such that trig_func(pi*arg) can be simplified, or which should have
/// been reduced to the (0, 1/2) interval.
fn can_trig_be_reduced(arg: BigRational) -> bool {
    !arg.is_positive()
        || arg >= BigRational::new(1.into(), 2.into())
        || arg == BigRational::new(1.into(), 3.into())
        || arg == BigRational::new(1.into(), 4.into())
        || arg == BigRational::new(1.into(), 6.into())
}

/// Reduce a SIN_PI or TAN_PI argument to the interval [-1/2, 1.5).
fn reduced_arg(arg: BigRational) -> BigRational {
    if arg >= BigRational::new((-1).into(), 2.into()) && arg < BigRational::new(3.into(), 2.into()) {
        return arg;
    }

    let arg_plus_half = arg.clone() + BigRational::new(1.into(), 2.into());
    let arg_ph_floor = arg_plus_half.floor().to_integer();
    let result_offset = arg_ph_floor & !BigInt::one();
    arg - result_offset
}
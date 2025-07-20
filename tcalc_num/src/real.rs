pub mod constants;
mod cr_property;
mod signed_property;

use crate::constructive_real::ConstructiveReal;
use crate::constructive_real::constants::ONE;
use crate::error::NumResult;
use crate::real::cr_property::{CRProperty, CRPropertyType};
use num::bigint::Sign;
use num::{BigInt, BigRational, FromPrimitive, One, Signed};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Real {
    rat: BigRational,
    cr: ConstructiveReal,
    cr_property: CRProperty,
}

impl Real {
    pub fn new(rat: BigRational, cr: ConstructiveReal, cr_property: CRProperty) -> Self {
        Self {
            rat,
            cr,
            cr_property,
        }
    }

    /// Shorthand constructor; computes non-null property only for a few special cases.
    pub fn new_from_rat_cr(rat: BigRational, cr: ConstructiveReal) -> Self {
        Self {
            rat,
            cr_property: Option::<CRProperty>::from(cr.clone()).unwrap(),
            cr,
        }
    }

    pub fn new_from_cr(cr: ConstructiveReal) -> Self {
        Self::new_from_rat_cr(BigRational::one(), cr)
    }

    pub fn new_from_cr_property(cr: ConstructiveReal, cr_property: CRProperty) -> Self {
        Self::new(BigRational::one(), cr, cr_property)
    }

    pub fn new_from_rat_property(rat: BigRational, cr_property: CRProperty) -> Self {
        Self::new(rat, cr_property.cr().unwrap().unwrap(), cr_property)
    }

    pub fn new_from_property(cr_property: CRProperty) -> Self {
        Self::new_from_rat_property(BigRational::one(), cr_property)
    }

    pub fn new_from_rational(rat: BigRational) -> Self {
        Self::new(rat, ONE.clone(), CRProperty::one())
    }

    /// Check that if crProperty uniquely defines a constructive real, then crProperty
    /// and crFactor both describe approximately the same number.
    pub fn property_correct(&self, prec: i32) -> NumResult<bool> {
        let property_cr = self.cr_property.cr()?;
        if let Some(property_cr) = property_cr {
            let bound = self.cr_property.msb_bound();
            if bound != i32::MIN
                && property_cr
                    .clone()
                    .abs()
                    .compare_to_absolute(&(ONE.clone() << bound)?, prec)?
                    == Ordering::Less
            {
                // msb_bound produced incorrect result.
                Ok(false)
            } else {
                Ok(self.cr.compare_to_absolute(&property_cr, prec)? == Ordering::Equal)
            }
        } else {
            Ok(true)
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
    if arg >= BigRational::new((-1).into(), 2.into()) && arg < BigRational::new(3.into(), 2.into())
    {
        return arg;
    }

    let arg_plus_half = arg.clone() + BigRational::new(1.into(), 2.into());
    let arg_ph_floor = arg_plus_half.floor().to_integer();
    let result_offset = arg_ph_floor & !BigInt::one();
    arg - result_offset
}

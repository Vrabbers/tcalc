use std::{
    fmt::{Debug, Display},
    marker::Sized,
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

use num::{bigint::Sign, traits::Inv};

use crate::error::NumResult;

pub trait Number:
    Add<Output = NumResult<Self>>
    + Neg<Output = Self>
    + Sub<Output = NumResult<Self>>
    + Mul<Output = NumResult<Self>>
    + Div<Output = NumResult<Self>>
    + Inv<Output = NumResult<Self>>
    + FromStr
    + Display
    + Debug
    + Sized
    + Clone
    + 'static
{
    fn abs(&self) -> NumResult<Self>;
    fn sign(&self) -> NumResult<Sign>;

    fn pow(&self, expon: Self) -> NumResult<Self>;
    fn ln(&self) -> NumResult<Self>;
    fn log(&self) -> NumResult<Self>;
    fn exp(&self) -> NumResult<Self>;
    fn sqrt(&self) -> NumResult<Self>;
    fn fact(&self) -> NumResult<Self>;

    fn degrees_to_radians(&self) -> NumResult<Self>;
    fn radians_to_degrees(&self) -> NumResult<Self>;

    fn sin(&self) -> NumResult<Self>;
    fn cos(&self) -> NumResult<Self>;
    fn tan(&self) -> NumResult<Self>;

    fn asin(&self) -> NumResult<Self>;
    fn acos(&self) -> NumResult<Self>;
    fn atan(&self) -> NumResult<Self>;

    fn sinh(&self) -> NumResult<Self>;
    fn cosh(&self) -> NumResult<Self>;
    fn tanh(&self) -> NumResult<Self>;
}

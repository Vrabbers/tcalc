use std::{
    marker::Sized,
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

use num::traits::Inv;

use crate::error::NumResult;

pub trait Num:
    Add<Output = NumResult<Self>>
    + Neg<Output = Self>
    + Sub<Output = NumResult<Self>>
    + Mul<Output = NumResult<Self>>
    + Div<Output = NumResult<Self>>
    + Inv<Output = NumResult<Self>>
    + FromStr
    + Sized
{
    fn log(&self) -> NumResult<Self>;
    fn exp(&self) -> NumResult<Self>;
}

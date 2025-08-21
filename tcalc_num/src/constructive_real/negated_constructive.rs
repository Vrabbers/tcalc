use crate::constructive_real::{ConstructiveReal, ConstructiveRealType};
use crate::error::NumResult;
use num::BigInt;
use std::ops::Neg;

#[derive(Debug)]
pub(crate) struct NegatedConstructive(pub ConstructiveReal);

impl ConstructiveRealType for NegatedConstructive {
    fn approximate(&self, precision: i32, _: &ConstructiveReal) -> NumResult<BigInt> {
        Ok(self.0.clone().get_appr(precision)?.neg())
    }
}

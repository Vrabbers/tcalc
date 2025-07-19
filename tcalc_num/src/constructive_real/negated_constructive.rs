use std::ops::Neg;
use cancellation_token::CancellationToken;
use num::BigInt;
use crate::constructive_real::{ConstructiveReal, ConstructiveRealType};
use crate::error::NumResult;

#[derive(Debug)]
pub(crate) struct NegatedConstructive(pub ConstructiveReal);

impl ConstructiveRealType for NegatedConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        Ok(self.0.clone().get_appr(precision)?.neg())
    }
}

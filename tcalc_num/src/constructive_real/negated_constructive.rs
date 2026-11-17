use crate::constructive_real::{Approximate, Cached, ConstructiveReal};
use crate::error::NumResult;
use cancellation_token::CancellationToken;
use num::BigInt;
use std::ops::Neg;

#[derive(Debug)]
pub(crate) struct NegatedConstructive(pub ConstructiveReal);

impl Approximate for Cached<NegatedConstructive> {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        Ok(self.operation.0.clone().get_appr(precision)?.neg())
    }
}

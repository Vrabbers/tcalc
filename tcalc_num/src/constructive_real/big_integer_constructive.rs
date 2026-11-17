use crate::constructive_real::{Approximate, Cached, scale};
use crate::error::NumResult;
use cancellation_token::CancellationToken;
use num::BigInt;

#[derive(Debug)]
pub(crate) struct BigIntegerConstructive(pub BigInt);

impl Approximate for Cached<BigIntegerConstructive> {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        Ok(scale(self.operation.0.clone(), -precision))
    }
}

use crate::constructive_real::{ConstructiveRealType, scale};
use crate::error::NumResult;
use cancellation_token::CancellationToken;
use num::BigInt;

#[derive(Debug)]
pub(crate) struct BigIntegerConstructive(pub BigInt);

impl ConstructiveRealType for BigIntegerConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        Ok(scale(self.0.clone(), -precision))
    }
}

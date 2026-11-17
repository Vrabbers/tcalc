use crate::constructive_real::{Approximate, Cached, ConstructiveReal};
use crate::error::NumResult;
use cancellation_token::CancellationToken;
use num::BigInt;

#[derive(Debug)]
pub(crate) struct ShiftConstructive {
    pub op: ConstructiveReal,
    pub count: i32,
}

impl Approximate for Cached<ShiftConstructive> {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        self.operation
            .op
            .clone()
            .get_appr(precision - self.operation.count)
    }
}

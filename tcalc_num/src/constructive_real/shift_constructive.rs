use crate::constructive_real::{ConstructiveReal, ConstructiveRealType};
use crate::error::NumResult;
use cancellation_token::CancellationToken;
use num::BigInt;

#[derive(Debug)]
pub(crate) struct ShiftConstructive {
    pub op: ConstructiveReal,
    pub count: i32,
}

impl ConstructiveRealType for ShiftConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        self.op.clone().get_appr(precision - self.count)
    }
}

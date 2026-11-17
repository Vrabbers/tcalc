use crate::constructive_real::{Approximate, Cached, ConstructiveReal, scale};
use crate::error::NumResult;
use cancellation_token::CancellationToken;
use num::BigInt;

#[derive(Debug)]
pub(crate) struct AssumedIntConstructive(pub ConstructiveReal);

impl Approximate for Cached<AssumedIntConstructive> {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        if precision >= 0 {
            self.operation.0.clone().get_appr(precision)
        } else {
            Ok(scale(self.operation.0.clone().get_appr(0)?, -precision))
        }
    }
}

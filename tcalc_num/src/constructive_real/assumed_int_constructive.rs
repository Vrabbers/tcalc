use cancellation_token::CancellationToken;
use num::BigInt;
use crate::constructive_real::{scale, ConstructiveReal, ConstructiveRealType};
use crate::error::NumResult;

#[derive(Debug)]
pub(crate) struct AssumedIntConstructive(pub ConstructiveReal);

impl ConstructiveRealType for AssumedIntConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        if precision >= 0 {
            self.0.clone().get_appr(precision)
        } else {
            Ok(scale(self.0.clone().get_appr(0)?, -precision))
        }
    }
}
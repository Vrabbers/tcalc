use cancellation_token::CancellationToken;
use num::BigInt;
use crate::constructive_real::{scale, ConstructiveReal, ConstructiveRealType};
use crate::error::NumResult;

#[derive(Debug)]
pub(crate) struct AddConstructive {
    pub op1: ConstructiveReal,
    pub op2: ConstructiveReal,
}

impl ConstructiveRealType for AddConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        // Args need to be evaluated so that each error is < 1/4 ulp.
        // Rounding error from the cale call is <= 1/2 ulp, so that
        // final error is < 1 ulp.
        Ok(scale(
            self.op1.clone().get_appr(precision - 2)? + self.op2.clone().get_appr(precision - 2)?,
            -2,
        ))
    }
}

use crate::constructive_real::{ConstructiveReal, ConstructiveRealType, scale};
use crate::error::NumResult;
use num::BigInt;

#[derive(Debug)]
pub(crate) struct AddConstructive {
    pub op1: ConstructiveReal,
    pub op2: ConstructiveReal,
}

impl ConstructiveRealType for AddConstructive {
    fn approximate(&self, precision: i32, _: &ConstructiveReal) -> NumResult<BigInt> {
        // Args need to be evaluated so that each error is < 1/4 ulp.
        // Rounding error from the cale call is <= 1/2 ulp, so that
        // final error is < 1 ulp.
        Ok(scale(
            self.op1.clone().get_appr(precision - 2)? + self.op2.clone().get_appr(precision - 2)?,
            -2,
        ))
    }
}

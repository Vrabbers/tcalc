use crate::constructive_real::{scale, ConstructiveReal, ConstructiveRealType};
use crate::error::NumResult;
use num::BigInt;

#[derive(Debug)]
pub(crate) struct BigIntegerConstructive(pub BigInt);

impl ConstructiveRealType for BigIntegerConstructive {
    fn approximate(&self, precision: i32, _: &ConstructiveReal) -> NumResult<BigInt> {
        Ok(scale(self.0.clone(), -precision))
    }
}

use crate::constructive_real::ConstructiveRealType;
use crate::error::NumResult;
use cancellation_token::CancellationToken;
use num::BigInt;

#[derive(Debug)]
pub(crate) struct InvalidConstructive();

impl ConstructiveRealType for InvalidConstructive {
    fn approximate(&self, _precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        panic!("Tried to approximate an invalid constructive real.")
    }
}

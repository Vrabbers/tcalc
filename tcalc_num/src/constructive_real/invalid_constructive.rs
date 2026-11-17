use crate::constructive_real::{Approximate, Cached};
use crate::error::NumResult;
use cancellation_token::CancellationToken;
use num::BigInt;

#[derive(Debug)]
pub(crate) struct InvalidConstructive();

impl Approximate for Cached<InvalidConstructive> {
    fn approximate(&self, _precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        panic!("Tried to approximate an invalid constructive real.")
    }
}

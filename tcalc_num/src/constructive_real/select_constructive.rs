use crate::constructive_real::{ConstructiveReal, ConstructiveRealType, scale};
use crate::error::NumResult;
use cancellation_token::CancellationToken;
use num::bigint::Sign;
use num::{BigInt, One, Signed};

#[derive(Debug)]
pub(crate) struct SelectConstructive {
    pub selector: ConstructiveReal,
    pub op1: ConstructiveReal,
    pub op2: ConstructiveReal,
}

impl ConstructiveRealType for SelectConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        let selector_sign = self.selector.clone().get_appr(-20)?.sign();
        match selector_sign {
            Sign::Minus => self.op1.clone().get_appr(precision),
            Sign::Plus => self.op2.clone().get_appr(precision),
            Sign::NoSign => {
                let op1_appr = self.op1.clone().get_appr(precision - 1)?;
                let op2_appr = self.op2.clone().get_appr(precision - 1)?;
                let diff = (op1_appr.clone() - op2_appr.clone()).abs();
                if diff <= BigInt::one() {
                    // close enough; use either
                    return Ok(scale(op1_appr, -1));
                }

                if self.selector.clone().sign()? == Sign::Minus {
                    Ok(scale(op1_appr, -1))
                } else {
                    Ok(scale(op2_appr, -1))
                }
            }
        }
    }
}

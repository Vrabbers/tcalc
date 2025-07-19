use std::mem::swap;
use cancellation_token::CancellationToken;
use num::{BigInt, Zero};
use num::bigint::Sign;
use crate::constructive_real::{scale, ConstructiveReal, ConstructiveRealType};
use crate::error::NumResult;

#[derive(Debug)]
pub(crate) struct MultiplyConstructive {
    pub op1: ConstructiveReal,
    pub op2: ConstructiveReal,
}

impl ConstructiveRealType for MultiplyConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        let mut op1 = self.op1.clone();
        let mut op2 = self.op2.clone();
        let half_prec = (precision >> 1) - 1;
        let mut msd_op1 = op1.msd_n(half_prec)?;
        let mut msd_op2;
        if msd_op1 == i32::MIN {
            msd_op2 = op2.msd_n(half_prec)?;
            if msd_op2 == i32::MIN {
                // Product is small enough that zero will do as an
                // approximation.
                return Ok(BigInt::zero());
            } else {
                // Swap them, so the larger operand (in absolute value)
                // is first.
                swap(&mut op1, &mut op2);
                msd_op1 = msd_op2;
            }
        }

        // msd_op1 is valid at this point.
        let prec2 = precision - msd_op1 - 3; // Precision needed for op2.
        // The appr. error is multiplied by at most
        // 2 ** (msd_op1 + 1)
        // Thus each approximation contributes 1/4 ulp
        // to the rounding error, and the final rounding adds
        // another 1/2 ulp.
        let appr2 = op2.get_appr(prec2)?;
        if appr2.sign() == Sign::NoSign {
            return Ok(BigInt::zero());
        }

        msd_op2 = op2.known_msd();
        let prec1 = precision - msd_op2 - 3; // Precision needed for op1.
        let appr1 = op1.get_appr(prec1)?;
        let scale_digits = prec1 + prec2 - precision;
        Ok(scale(appr1 * appr2, scale_digits))
    }
}

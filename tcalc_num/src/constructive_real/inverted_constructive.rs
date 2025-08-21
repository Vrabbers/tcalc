use crate::constructive_real::{ConstructiveReal, ConstructiveRealType};
use crate::error::NumResult;
use num::bigint::Sign;
use num::{BigInt, One, Signed, Zero};

#[derive(Debug)]
pub(crate) struct InvertedConstructive(pub ConstructiveReal);

impl ConstructiveRealType for InvertedConstructive {
    fn approximate(&self, precision: i32, _: &ConstructiveReal) -> NumResult<BigInt> {
        let mut op = self.0.clone();
        let msd = op.msd()?;
        let inv_msd = 1 - msd;
        let digits_needed = inv_msd - precision + 3;
        // Number of SIGNIFICANT digits needed for
        // argument, excl. msd position, which may
        // be fictitious, since msd routine can be
        // off by 1.  Roughly 1 extra digit is
        // needed since the relative error is the
        // same in the argument and result, but
        // this isn't quite the same as the number
        // of significant digits.  Another digit
        // is needed to compensate for slop in the
        // calculation.
        // One further bit is required, since the
        // final rounding introduces a 0.5 ulp
        // error.
        let prec_needed = msd - digits_needed;
        let log_scale_factor = -precision - prec_needed;
        if log_scale_factor < 0 {
            return Ok(BigInt::zero());
        }
        let dividend = BigInt::one() << log_scale_factor;
        let scaled_divisor = op.get_appr(prec_needed)?;
        let abs_scaled_divisor = scaled_divisor.abs();
        let adj_dividend = dividend + (abs_scaled_divisor.clone() >> 1);
        // Adjustment so that final result is rounded.
        let result: BigInt = adj_dividend / abs_scaled_divisor;
        if scaled_divisor.sign() == Sign::Minus {
            Ok(-result)
        } else {
            Ok(result)
        }
    }
}

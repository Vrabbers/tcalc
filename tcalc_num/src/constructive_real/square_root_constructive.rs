use crate::constructive_real::{ConstructiveReal, ConstructiveRealType, scale, shift};
use crate::error::DomainViolation::NthRoot;
use crate::error::NumError::DomainViolation;
use crate::error::{CancelCheckable, NumResult};
use cancellation_token::CancellationToken;
use num::{BigInt, One, ToPrimitive, Zero};

#[derive(Debug)]
pub(crate) struct SquareRootConstructive(pub ConstructiveReal);

impl ConstructiveRealType for SquareRootConstructive {
    fn approximate(&self, precision: i32, ct: CancellationToken) -> NumResult<BigInt> {
        let op = self.0.clone();
        // Conservative estimate of number of
        // significant bits in double precision
        // computation.
        const FP_PREC: i32 = 50;
        const FP_OP_PREC: i32 = 60;

        let max_prec_needed = 2 * precision - 1;
        let msd = op.msd_n(max_prec_needed)?;
        if msd <= max_prec_needed {
            return Ok(BigInt::zero());
        };
        let result_msd = msd / 2; // +- 1
        let result_digits = result_msd - precision; // +- 2
        if result_digits > FP_PREC {
            // Compute less precise approximation and use a Newton iter.
            let appr_digits = result_digits / 2 + 6;
            // This should be conservative.  Is fewer enough?
            let appr_prec = result_msd - appr_digits;
            ct.stop_if_cancelled()?;
            let last_appr = self.approximate(appr_prec, ct)?; // TODO: changed from get_appr.
            let prod_prec = 2 * appr_prec;
            let op_appr = op.get_appr(prod_prec)?;
            // Slightly fewer might be enough;
            // Compute (last_appr * last_appr + op_appr)/(last_appr/2)
            // while adjusting the scaling to make everything work
            let prod_prec_scaled_numerator = last_appr.clone() * last_appr.clone() + op_appr;
            let scaled_numerator = scale(prod_prec_scaled_numerator, appr_prec - precision);
            let shifted_result = scaled_numerator / last_appr;
            Ok((shifted_result + BigInt::one()) >> 1)
        } else {
            // Use a double precision floating point approximation.
            // Make sure all precisions are even
            let op_prec = (msd - FP_OP_PREC) & !1;
            let working_prec = op_prec - FP_OP_PREC;
            let scaled_bi_appr = op.get_appr(op_prec)? << FP_OP_PREC;
            let scaled_appr = scaled_bi_appr.to_f64().unwrap();
            if scaled_appr < 0. {
                return Err(DomainViolation(NthRoot(2.)));
            }
            let scaled_fp_sqrt = scaled_appr.sqrt();
            let scaled_sqrt = BigInt::from(scaled_fp_sqrt as i64);
            let shift_count = working_prec / 2 - precision;
            Ok(shift(scaled_sqrt, shift_count))
        }
    }
}

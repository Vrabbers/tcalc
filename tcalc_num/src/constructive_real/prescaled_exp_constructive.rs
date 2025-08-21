use crate::constructive_real::{ConstructiveReal, ConstructiveRealType, bound_log2, scale};
use crate::error::{CancelCheckable, NumResult};
use num::{BigInt, One, Signed, Zero};

#[derive(Debug)]
pub(crate) struct PrescaledExpConstructive(pub ConstructiveReal);

impl ConstructiveRealType for PrescaledExpConstructive {
    fn approximate(&self, precision: i32, cr: &ConstructiveReal) -> NumResult<BigInt> {
        if precision >= 1 {
            return Ok(BigInt::zero());
        }
        let iterations_needed = -precision / 2 + 2; // conservative estimate > 0.
        //  Claim: each intermediate term is accurate
        //  to 2*2^calc_precision.
        //  Total rounding error in series computation is
        //  2*iterations_needed*2^calc_precision,
        //  exclusive of error in op.
        let calc_precision = precision - bound_log2(2 * iterations_needed) - 4; // for error in op, truncation.
        let op_prec = precision - 3;
        let op_appr = self.0.clone().get_appr(op_prec)?;
        // Error in argument results in error of < 3/8 ulp.
        // Sum of term eval. rounding error is < 1/16 ulp.
        // Series truncation error < 1/16 ulp.
        // Final rounding error is <= 1/2 ulp.
        // Thus final error is < 1 ulp.
        let scaled_1 = BigInt::one() << -calc_precision;
        let mut current_term = scaled_1.clone();
        let mut current_sum = scaled_1.clone();
        let mut n = 0;
        let max_trunc_error = BigInt::one() << (precision - 4 - calc_precision);
        while current_term.clone().abs() >= max_trunc_error {
            cr.cancellation_token.stop_if_cancelled()?;
            n += 1;
            /* current_term = current_term * op / n */
            current_term = scale(current_term * op_appr.clone(), op_prec);
            current_term /= BigInt::from(n);
            current_sum += current_term.clone()
        }
        Ok(scale(current_sum, calc_precision - precision))
    }
}

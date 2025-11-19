use crate::constructive_real::{ConstructiveReal, ConstructiveRealType, bound_log2, scale};
use crate::error::{CancelCheckable, NumResult};
use cancellation_token::CancellationToken;
use num::{BigInt, One, Signed};
use std::ops::Add;

#[derive(Debug)]
pub(crate) struct PrescaledCosConstructive(pub ConstructiveReal);

impl ConstructiveRealType for PrescaledCosConstructive {
    fn is_slow(&self) -> bool {
        true
    }

    fn approximate(&self, precision: i32, ct: CancellationToken) -> NumResult<BigInt> {
        if precision >= 1 {
            return Ok(BigInt::from(0));
        }
        let iterations_needed = -precision / 2 + 4; // conservative estimate > 0.
        //  Claim: each intermediate term is accurate
        //  to 2*2^calc_precision.
        //  Total rounding error in series computation is
        //  2*iterations_needed*2^calc_precision,
        //  exclusive of error in op.
        let calc_precision = precision - bound_log2(2 * iterations_needed) - 4; // for error in op, truncation.
        let op_prec = precision - 2;
        let op_appr = self.0.get_appr(op_prec)?;
        // Error in argument results in error of < 1/4 ulp.
        // Cumulative arithmetic rounding error is < 1/16 ulp.
        // Series truncation error < 1/16 ulp.
        // Final rounding error is <= 1/2 ulp.
        // Thus final error is < 1 ulp.
        let mut current_term;
        let mut n;
        let max_trunc_error = BigInt::one() << (precision - 4 - calc_precision);
        n = 0;
        current_term = BigInt::one() << -calc_precision;
        let mut current_sum = current_term.clone();
        while current_term.abs() >= max_trunc_error {
            ct.stop_if_cancelled()?;
            n += 2;
            /* current_term = - current_term * op * op / n * (n - 1)   */
            current_term = scale(current_term * op_appr.clone(), op_prec);
            current_term = scale(current_term * op_appr.clone(), op_prec);
            let divisor = BigInt::from(-n) * BigInt::from(n - 1);
            current_term /= divisor;
            current_sum += current_term.clone();
        }
        Ok(scale(current_sum, calc_precision - precision))
    }
}

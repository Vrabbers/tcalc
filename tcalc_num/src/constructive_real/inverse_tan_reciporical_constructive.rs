use crate::constructive_real::{ConstructiveRealType, bound_log2, scale};
use crate::error::{CancelCheckable, NumResult};
use cancellation_token::CancellationToken;
use num::{BigInt, One, Signed, Zero};
use std::ops::Add;

#[derive(Debug)]
pub(crate) struct InverseTanReciprocalConstructive(pub i32);

impl ConstructiveRealType for InverseTanReciprocalConstructive {
    fn approximate(&self, precision: i32, ct: CancellationToken) -> NumResult<BigInt> {
        let op = self.0;
        if precision >= 1 {
            return Ok(BigInt::zero());
        };
        let iterations_needed = -precision / 2 + 2; // conservative estimate > 0.
        //  Claim: each intermediate term is accurate
        //  to 2*base^calc_precision.
        //  Total rounding error in series computation is
        //  2*iterations_needed*base^calc_precision,
        //  exclusive of error in op.
        let calc_precision = precision - bound_log2(2 * iterations_needed) - 2; // for error in op, truncation.
        // Error in argument results in error of < 3/8 ulp.
        // Cumulative arithmetic rounding error is < 1/4 ulp.
        // Series truncation error < 1/4 ulp.
        // Final rounding error is <= 1/2 ulp.
        // Thus final error is < 1 ulp.
        let scaled_1 = BigInt::one() << -calc_precision;
        let big_op = BigInt::from(op);
        let big_op_squared = BigInt::from(op * op);
        let op_inverse = scaled_1 / big_op;
        let mut current_power = op_inverse.clone();
        let mut current_term = op_inverse.clone();
        let mut current_sum = op_inverse.clone();
        let mut current_sign = 1;
        let mut n = 1;
        let max_trunc_error = BigInt::one() << (precision - 2 - calc_precision);
        while current_term.abs() >= max_trunc_error {
            ct.stop_if_cancelled()?;
            n += 2;
            current_power /= big_op_squared.clone();
            current_sign = -current_sign;
            current_term = current_power.clone() / BigInt::from(current_sign * n);
            current_sum = current_sum.add(current_term.clone());
        }
        Ok(scale(current_sum, calc_precision - precision))
    }
}

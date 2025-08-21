use crate::constructive_real::{ConstructiveReal, ConstructiveRealType, bound_log2, scale};
use crate::error::{CancelCheckable, NumResult};
use num::{BigInt, One, Signed, Zero};
use std::ops::Add;

#[derive(Debug)]
pub(crate) struct PrescaledLnConstructive(pub ConstructiveReal);

impl ConstructiveRealType for PrescaledLnConstructive {
    fn is_slow(&self) -> bool {
        true
    }

    fn approximate(&self, precision: i32, cr: &ConstructiveReal) -> NumResult<BigInt> {
        if precision >= 0 {
            return Ok(BigInt::zero());
        };
        let iterations_needed = -precision; // conservative estimate > 0.
        //  Claim: each intermediate term is accurate
        //  to 2*2^calc_precision.  Total error is
        //  2*iterations_needed*2^calc_precision
        //  exclusive of error in op.
        let calc_precision = precision - bound_log2(2 * iterations_needed) - 4; // for error in op, truncation.
        let op_prec = precision - 3;
        let op_appr = self.0.clone().get_appr(op_prec)?;
        // Error analysis as for exponential.
        let mut x_nth = scale(op_appr.clone(), op_prec - calc_precision);
        let mut current_term = x_nth.clone(); // x**n
        let mut current_sum = current_term.clone();
        let mut n = 1;
        let mut current_sign = 1; // (-1)^(n-1)
        let max_trunc_error = BigInt::one() << (precision - 4 - calc_precision);
        while current_term.abs() >= max_trunc_error {
            cr.cancellation_token.stop_if_cancelled()?;
            n += 1;
            current_sign = -current_sign;
            x_nth = scale(x_nth * op_appr.clone(), op_prec);
            current_term = x_nth.clone() / (BigInt::from(n * current_sign));
            // x**n / (n * (-1)**(n-1))
            current_sum = current_sum.add(current_term.clone());
        }
        Ok(scale(current_sum, calc_precision - precision))
    }
}

use crate::constructive_real::{ConstructiveReal, ConstructiveRealType, bound_log2, scale};
use crate::error::{CancelCheckable, NumResult};
use cancellation_token::CancellationToken;
use num::{BigInt, FromPrimitive, One, Signed, Zero};

#[derive(Debug)]
pub(crate) struct PrescaledAsinConstructive(pub ConstructiveReal);

impl ConstructiveRealType for PrescaledAsinConstructive {
    fn is_slow(&self) -> bool {
        true
    }

    fn approximate(&self, precision: i32, ct: CancellationToken) -> NumResult<BigInt> {
        // The Taylor series is the sum of x^(2n+1) * (2n)!/(4^n n!^2 (2n+1))
        // Note that (2n)!/(4^n n!^2) is always less than one.
        // (The denominator is effectively 2n*2n*(2n-2)*(2n-2)*...*2*2
        // which is clearly > (2n)!)
        // Thus all terms are bounded by x^(2n+1).
        // Unfortunately, there's no easy way to prescale the argument
        // to less than 1/sqrt(2), and we can only approximate that.
        // Thus the worst case iteration count is fairly high.
        // But it doesn't make much difference.
        if precision >= 2 {
            return Ok(BigInt::zero()); // Never bigger than 4.
        }
        let iterations_needed = -3 * precision / 2 + 4;
        // conservative estimate > 0.
        // Follows from assumed bound on x and
        // the fact that only every other Taylor
        // Series term is present.
        //  Claim: each intermediate term is accurate
        //  to 2*2^calc_precision.
        //  Total rounding error in series computation is
        //  2*iterations_needed*2^calc_precision,
        //  exclusive of error in op.
        let calc_precision = precision - bound_log2(2 * iterations_needed) - 4; // for error in op, truncation.
        let op_prec = precision - 3; // always <= -2
        let op_appr = self.0.get_appr(op_prec)?;
        // Error in argument results in error of < 1/4 ulp.
        // (Derivative is bounded by 2 in the specified range and we use
        // 3 extra digits.)
        // Ignoring the argument error, each term has an error of
        // < 3ulps relative to calc_precision, which is more precise than p.
        // Cumulative arithmetic rounding error is < 3/16 ulp (relative to p).
        // Series truncation error < 2/16 ulp.  (Each computed term
        // is at most 2/3 of last one, so some of remaining series <
        // 3/2 * current term.)
        // Final rounding error is <= 1/2 ulp.
        // Thus final error is < 1 ulp (relative to p).
        let max_last_term = BigInt::one() << precision - 4 - calc_precision;
        let mut exp = 1; // Current exponent, = 2n+1 in above expression
        let mut current_term = op_appr.clone() << op_prec - calc_precision;
        let mut current_sum = current_term.clone();
        let mut current_factor = current_term.clone();
        // Current scaled Taylor series term
        // before division by the exponent.
        // Accurate to 3 ulp at calc_precision.
        while current_term.clone().abs() >= (max_last_term) {
            ct.stop_if_cancelled()?;
            exp += 2;
            // current_factor = current_factor * op * op * (exp-1) * (exp-2) /
            // (exp-1) * (exp-1), with the two exp-1 factors cancelling,
            // giving
            // current_factor = current_factor * op * op * (exp-2) / (exp-1)
            // Thus the error any in the previous term is multiplied by
            // op^2, adding an error of < (1/2)^(2/3) < 2/3 the original
            // error.
            current_factor *= BigInt::from_i32(exp - 2).unwrap();
            current_factor = scale(current_factor * op_appr.clone(), op_prec + 2);
            // Carry 2 extra bits of precision forward; thus
            // this effectively introduces 1/8 ulp error.
            current_factor *= op_appr.clone();
            let divisor = BigInt::from_i32(exp - 1).unwrap();
            current_factor /= divisor;
            // Another 1/4 ulp error here.
            current_factor = scale(current_factor, op_prec - 2);
            // Remove extra 2 bits.  1/2 ulp rounding error.
            // Current_factor has original 3 ulp rounding error, which we
            // reduced by 1, plus < 1 ulp new rounding error.
            current_term = current_factor.clone() / BigInt::from_i32(exp).unwrap();
            // Contributes 1 ulp error to sum plus at most 3 ulp
            // from current_factor.
            current_sum += current_term.clone();
        }
        Ok(scale(current_sum, calc_precision - precision))
    }
}

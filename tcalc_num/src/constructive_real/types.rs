use crate::constructive_real::NumError::DomainViolation;
use crate::constructive_real::{ConstructiveReal, ConstructiveRealType, bound_log2, scale, shift};
use crate::error::{CancelCheckable, DomainViolation::SquareRootOfNegative, NumResult};
use cancellation_token::CancellationToken;
use num::{BigInt, One, Signed, ToPrimitive, Zero, bigint::Sign};
use std::mem::swap;
use std::ops::{Add, Neg};

#[derive(Debug)]
pub(crate) struct InvalidConstructive();

impl ConstructiveRealType for InvalidConstructive {
    fn approximate(&self, _precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        panic!("Tried to approximate an invalid constructive real.")
    }
}

#[derive(Debug)]
pub(crate) struct BigIntegerConstructive(pub BigInt);

impl ConstructiveRealType for BigIntegerConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        Ok(scale(self.0.clone(), -precision))
    }
}

#[derive(Debug)]
pub(crate) struct AddConstructive {
    pub op1: ConstructiveReal,
    pub op2: ConstructiveReal,
}

impl ConstructiveRealType for AddConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        // Args need to be evaluated so that each error is < 1/4 ulp.
        // Rounding error from the cale call is <= 1/2 ulp, so that
        // final error is < 1 ulp.
        Ok(scale(
            self.op1.clone().get_appr(precision - 2)? + self.op2.clone().get_appr(precision - 2)?,
            -2,
        ))
    }
}

#[derive(Debug)]
pub(crate) struct ShiftConstructive {
    pub op: ConstructiveReal,
    pub count: i32,
}

impl ConstructiveRealType for ShiftConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        self.op.clone().get_appr(precision - self.count)
    }
}

#[derive(Debug)]
pub(crate) struct AssumedIntConstructive(pub ConstructiveReal);

impl ConstructiveRealType for AssumedIntConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        if precision >= 0 {
            self.0.clone().get_appr(precision)
        } else {
            Ok(scale(self.0.clone().get_appr(0)?, -precision))
        }
    }
}

#[derive(Debug)]
pub(crate) struct NegatedConstructive(pub ConstructiveReal);

impl ConstructiveRealType for NegatedConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        Ok(self.0.clone().get_appr(precision)?.neg())
    }
}

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

#[derive(Debug)]
pub(crate) struct InvertedConstructive(pub ConstructiveReal);

impl ConstructiveRealType for InvertedConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
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

#[derive(Debug)]
pub(crate) struct PrescaledExpConstructive(pub ConstructiveReal);

impl ConstructiveRealType for PrescaledExpConstructive {
    fn approximate(&self, precision: i32, ct: CancellationToken) -> NumResult<BigInt> {
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
            ct.stop_if_cancelled()?;
            n += 1;
            /* current_term = current_term * op / n */
            current_term = scale(current_term * op_appr.clone(), op_prec);
            current_term /= BigInt::from(n);
            current_sum += current_term.clone()
        }
        Ok(scale(current_sum, calc_precision - precision))
    }
}

#[derive(Debug)]
pub(crate) struct PrescaledLnConstructive(pub ConstructiveReal);

impl ConstructiveRealType for PrescaledLnConstructive {
    fn is_slow(&self) -> bool {
        true
    }

    fn approximate(&self, precision: i32, ct: CancellationToken) -> NumResult<BigInt> {
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
            ct.stop_if_cancelled()?;
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
            current_sum = current_sum.add(current_term.clone());
        }
        Ok(scale(current_sum, calc_precision - precision))
    }
}

#[derive(Debug)]
pub(crate) struct SquareRootConstructive(pub ConstructiveReal);

impl ConstructiveRealType for SquareRootConstructive {
    fn approximate(&self, precision: i32, ct: CancellationToken) -> NumResult<BigInt> {
        let mut op = self.0.clone();
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
                return Err(DomainViolation(SquareRootOfNegative));
            }
            let scaled_fp_sqrt = scaled_appr.sqrt();
            let scaled_sqrt = BigInt::from(scaled_fp_sqrt as i64);
            let shift_count = working_prec / 2 - precision;
            Ok(shift(scaled_sqrt, shift_count))
        }
    }
}

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

use crate::error::DomainViolation::{LogarithmOfNegative, SquareRootOfNegative};
use crate::error::InternalError::{ConstructiveRealFromInf, ConstructiveRealFromNan};
use crate::error::NumError::{DomainViolation, InternalError, PrecisionOverflow};
use crate::error::{CancelCheckable, NumError, NumResult};
use cancellation_token::CancellationToken;
use num::bigint::Sign;
use num::{BigInt, Integer, One, Signed, ToPrimitive, Zero};
use std::cell::RefCell;
use std::clone::Clone;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::mem::swap;
use std::ops::{Add, Div, Mul, Neg, Shl, Shr, Sub};
use std::rc::Rc;

// https://android.googlesource.com/platform/external/crcalc/+/6db978c639e9bd5ac63fd88cbf3765d8c0fb3271/src/com/hp/creals/CR.java

trait ConstructiveRealType
where
    Self: Debug,
{
    ///  Must be defined in implementors of ConstructiveRealApproximation
    ///  Returns value / 2 ** precision rounded to an integer.
    ///  The error in the result is strictly < 1.
    ///  Informally, approximate(n) gives a scaled approximation
    ///  accurate to 2**n.
    ///  Implementations may safely assume that precision is
    ///  at least a factor of 8 away from overflow.
    fn approximate(&self, precision: i32, ct: CancellationToken) -> NumResult<BigInt>;

    // Should be true for implementations for which approximate calls are
    // somewhat expensive.
    // If we need to (re)evaluate, we speculatively evaluate to slightly
    // higher precision, miminimizing reevaluations.
    // Note that this requires any arguments to be evaluated to higher
    // precision than absolutely necessary.  It can thus potentially
    // result in lots of wasted effort, and should be used judiciously.
    // This assumes that the order of magnitude of the number is roughly one.
    fn is_slow(&self) -> bool {
        false
    }
}

#[derive(Debug)]
struct InvalidConstructive();

impl ConstructiveRealType for InvalidConstructive {
    fn approximate(&self, _precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        panic!("Tried to approximate an invalid constructive real.")
    }
}

#[derive(Debug)]
struct BigIntegerConstructive(BigInt);

impl ConstructiveRealType for BigIntegerConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        Ok(scale(self.0.clone(), -precision))
    }
}

#[derive(Debug)]
struct AddConstructive {
    op1: ConstructiveReal,
    op2: ConstructiveReal,
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
struct ShiftConstructive {
    op: ConstructiveReal,
    count: i32,
}

impl ConstructiveRealType for ShiftConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        self.op.clone().get_appr(precision - self.count)
    }
}

#[derive(Debug)]
struct AssumedIntConstructive(ConstructiveReal);

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
struct NegatedConstructive(ConstructiveReal);

impl ConstructiveRealType for NegatedConstructive {
    fn approximate(&self, precision: i32, _: CancellationToken) -> NumResult<BigInt> {
        Ok(self.0.clone().get_appr(precision)?.neg())
    }
}

#[derive(Debug)]
struct MultiplyConstructive {
    op1: ConstructiveReal,
    op2: ConstructiveReal,
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
struct InvertedConstructive(ConstructiveReal);

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
struct SelectConstructive {
    selector: ConstructiveReal,
    op1: ConstructiveReal,
    op2: ConstructiveReal,
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
struct PrescaledExpConstructive(ConstructiveReal);

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
struct PrescaledLnConstructive(ConstructiveReal);

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
struct PrescaledCosConstructive(ConstructiveReal);

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
struct SquareRootConstructive(ConstructiveReal);

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
struct InverseTanReciprocalConstructive(i32);

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

#[derive(Clone, Debug)]
pub struct ConstructiveReal {
    t: Rc<dyn ConstructiveRealType>,
    current_approximation: Rc<RefCell<Option<ConstructiveRealApproximation>>>,
    cancellation_token: CancellationToken,
}

#[derive(Debug)]
pub struct ConstructiveRealApproximation {
    pub min_prec: i32,
    pub max_appr: BigInt,
}

impl ConstructiveReal {
    fn get_appr(&self, precision: i32) -> NumResult<BigInt> {
        check_prec(precision)?;

        let current_approximation_borrow = self.current_approximation.borrow();
        let current_approximation = current_approximation_borrow.as_ref();

        if self.t.is_slow() {
            let max_prec = -64;
            let prec_incr = 32;

            if let Some(current_approximation) = current_approximation
                && precision >= current_approximation.min_prec
            {
                Ok(scale(
                    current_approximation.max_appr.clone(),
                    current_approximation.min_prec - precision,
                ))
            } else {
                drop(current_approximation_borrow);

                let eval_prec = if precision >= max_prec {
                    max_prec
                } else {
                    (precision - prec_incr + 1) & !(prec_incr - 1)
                };
                let result = self
                    .t
                    .approximate(eval_prec, self.cancellation_token.clone())?;
                if let Some(current_approximation) =
                    self.current_approximation.borrow_mut().as_mut()
                {
                    current_approximation.min_prec = precision;
                    current_approximation.max_appr = result.clone();
                } else {
                    self.current_approximation
                        .replace(Some(ConstructiveRealApproximation {
                            min_prec: precision,
                            max_appr: result.clone(),
                        }));
                }
                Ok(scale(result, eval_prec - precision))
            }
        } else if let Some(current_approximation) = current_approximation
            && precision >= current_approximation.min_prec
        {
            Ok(scale(
                current_approximation.max_appr.clone(),
                current_approximation.min_prec - precision,
            ))
        } else {
            drop(current_approximation_borrow);

            let result = self
                .t
                .approximate(precision, self.cancellation_token.clone())?;
            if let Some(current_approximation) = self.current_approximation.borrow_mut().as_mut() {
                current_approximation.min_prec = precision;
                current_approximation.max_appr = result.clone();
            } else {
                self.current_approximation
                    .replace(Some(ConstructiveRealApproximation {
                        min_prec: precision,
                        max_appr: result.clone(),
                    }));
            }
            Ok(result)
        }
    }

    /// Return the position of the msd.
    /// If x.msd() == n then
    /// 2**(n-1) < abs(x) < 2**(n+1)
    /// This initial version assumes that max_appr is valid
    /// and sufficiently removed from zero
    /// that the msd is determined.
    fn known_msd(&self) -> i32 {
        let current_approximation_borrow = self.current_approximation.borrow();
        let current_approximation = current_approximation_borrow.as_ref().unwrap();

        let length = if current_approximation.max_appr.sign() != Sign::Minus {
            current_approximation.max_appr.bits()
        } else {
            current_approximation.max_appr.abs().bits()
        } as i32;

        current_approximation.min_prec + length - 1
    }

    /// This version may return i32::MIN if the correct
    /// answer is < n.
    fn msd_n(&mut self, n: i32) -> NumResult<i32> {
        let current_approximation_borrow = self.current_approximation.borrow();
        let current_approximation = current_approximation_borrow.as_ref();

        if current_approximation.is_none() || {
            let current_approximation = current_approximation.unwrap();
            current_approximation.max_appr <= BigInt::one()
                && current_approximation.max_appr >= BigInt::one().mul(-1)
        } {
            drop(current_approximation_borrow);

            self.get_appr(n - 1)?;

            if self
                .current_approximation
                .borrow()
                .as_ref()
                .unwrap()
                .max_appr
                .abs()
                <= BigInt::one()
            {
                return Ok(i32::MIN);
            }
        }

        Ok(self.known_msd())
    }

    /// Functionally equivalent, but iteratively evaluates to higher
    /// precision.
    fn iter_msd(&mut self, n: i32) -> NumResult<i32> {
        let mut prec = 0;
        while prec > n + 30 {
            let msd = self.msd_n(prec)?;
            if msd != i32::MIN {
                return Ok(msd);
            }
            check_prec(prec)?;
            self.cancellation_token.stop_if_cancelled()?;
            prec = (prec * 3) / 2 - 16;
        }
        self.msd_n(n)
    }

    /// This version returns a correct answer eventually, except
    /// that it loops forever (or returns an error) if this
    /// constructive real is zero.
    fn msd(&mut self) -> NumResult<i32> {
        self.iter_msd(i32::MIN)
    }

    /// Return 0 if x = y to within the indicated tolerance,
    /// -1 if x < y, and +1 if x > y.  If x and y are indeed
    /// equal, it is guaranteed that 0 will be returned.  If
    /// they differ by less than the tolerance, anything
    /// may happen.  The tolerance allowed is
    /// the maximum of (abs(this)+abs(x))*(2\**r) and 2\**a
    ///
    /// Parameters:
    /// - x: The other constructive real
    /// - r: Relative tolerance in bits
    /// - a: Absolute tolerance in bits
    pub fn compare_to_relative(&mut self, x: &mut Self, r: i32, a: i32) -> NumResult<Ordering> {
        let this_msd = self.iter_msd(a)?;
        let x_msd = x.iter_msd(if this_msd > a { this_msd } else { a })?;
        let max_msd = if x_msd > this_msd { x_msd } else { this_msd };
        let rel = max_msd + r;

        // This can't approach overflow, since r and a are
        // effectively divided by 2, and msds are checked.
        let abs_prec = if rel > a { rel } else { a };
        self.compare_to_absolute(x, abs_prec)
    }

    /// Approximate comparison with only an absolute tolerance.
    /// Identical to the three argument version, but without a relative
    /// tolerance.
    /// Result is 0 if both constructive reals are equal, indeterminate
    /// if they differ by less than 2**a.
    ///
    /// Parameters:
    /// - x: The other constructive real
    /// - r: Relative tolerance in bits
    /// - a: Absolute tolerance in bits
    pub fn compare_to_absolute(&mut self, x: &mut Self, a: i32) -> NumResult<Ordering> {
        let needed_prec = a - 1;
        let this_appr = self.get_appr(needed_prec)?;
        let x_appr = x.get_appr(needed_prec)?;
        let comp1 = this_appr.cmp(&x_appr.clone().add(&BigInt::one()));
        if comp1 == Ordering::Greater {
            return Ok(Ordering::Greater);
        }
        let comp2 = this_appr.cmp(&x_appr.sub(&BigInt::one()));
        if comp2 == Ordering::Less {
            return Ok(Ordering::Less);
        }
        Ok(Ordering::Equal)
    }

    /// Return -1 if self < x, or +1 if self > x.
    /// Should be called only if self != x.
    /// If self == x, this will not terminate correctly; typically it
    /// will run until it exhausts memory.
    /// If the two constructive reals may be equal, the two or 3 argument
    /// version of compare_to should be used.
    pub fn compare_to(&mut self, x: &mut Self) -> NumResult<Ordering> {
        let mut a = -20;
        loop {
            check_prec(a)?;
            let result = self.compare_to_absolute(x, a)?;
            if result != Ordering::Equal {
                return Ok(result);
            }
            a *= 2;
        }
    }

    /// Equivalent to <TT>compareTo(CR.valueOf(0), a)</tt>
    pub fn sign_precision(&mut self, a: i32) -> NumResult<Sign> {
        if let Some(current_approximation) = self.current_approximation.borrow().as_ref() {
            let quick_try = current_approximation.max_appr.sign();
            if quick_try != Sign::NoSign {
                return Ok(quick_try);
            }
        }
        let needed_prec = a - 1;
        let this_appr = self.get_appr(needed_prec)?;
        let sign = this_appr.sign();
        Ok(sign)
    }

    /// Return -1 if negative, +1 if positive.
    /// Should be called only if self != 0.
    /// In the 0 case, this will not terminate correctly; typically it
    /// will run until it exhausts memory.
    /// If the two constructive reals may be equal, the one or two argument
    /// version of sign should be used.
    pub fn sign(&mut self) -> NumResult<Sign> {
        let mut a = -20;
        loop {
            check_prec(a)?;
            let sign = self.sign_precision(a)?;
            if sign != Sign::NoSign {
                return Ok(sign);
            }
            a *= 2;
        }
    }

    /// Return a textual representation accurate to <TT>n</tt> places
    /// to the right of the decimal point.  <TT>n</tt> must be nonnegative.
    ///
    /// Parameters:
    /// - n: Number of digits (>= 0) included to the right of decimal point
    /// - radix: Base ( >= 2, <= 16) for the resulting representation
    pub fn to_string(&self, n: u32, radix: u32) -> NumResult<String> {
        let scaled_cr: ConstructiveReal = if radix == 16 {
            (self.clone() << (4 * n) as i32)?
        } else {
            let scale_factor = BigInt::from(radix).pow(n);
            self.clone() * ConstructiveReal::from(scale_factor)
        };

        let scaled_int = scaled_cr.get_appr(0)?;
        let mut scaled_string = scaled_int.abs().to_str_radix(radix);
        let result = if n == 0 {
            scaled_string
        } else {
            let mut len = scaled_string.len() as u32;
            if len <= n {
                scaled_string =
                    str::repeat("0", (n + 1 - len) as usize).to_string() + &scaled_string;
                len = n + 1
            }
            let whole = &scaled_string[0..(len - n) as usize];
            let fraction = &scaled_string[(len - n) as usize..];
            format!("{whole}.{fraction}")
        };
        Ok(if scaled_int.sign() == Sign::Minus {
            format!("-{result}")
        } else {
            result
        })
    }

    /// Return a BigInteger which differs by less than one from the
    /// constructive real.
    pub fn to_bigint(&self) -> NumResult<BigInt> {
        self.clone().get_appr(0)
    }

    /// Produce a constructive real equivalent to the original, assuming
    /// the original was an integer.  Undefined results if the original
    /// was not an integer.  Prevents evaluation of digits to the right
    /// of the decimal point, and may thus improve performance.
    pub fn assume_int(self) -> ConstructiveReal {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Rc::new(AssumedIntConstructive(self)),
            ..ConstructiveReal::default()
        }
    }

    /// The multiplicative inverse of a constructive real.
    /// x.inverse() is equivalent to ConstructiveReal::from(1).divide(x).
    pub fn inverse(self) -> ConstructiveReal {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Rc::new(InvertedConstructive(self)),
            ..ConstructiveReal::default()
        }
    }

    /// The real number x if self < 0, or y< otherwise.
    /// Requires x = y if self = 0.
    /// Since comparisons may diverge, this is often
    /// a useful alternative to conditionals.
    pub fn select(self, x: Self, y: Self) -> ConstructiveReal {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Rc::new(SelectConstructive {
                selector: self,
                op1: x,
                op2: y,
            }),
            ..ConstructiveReal::default()
        }
    }

    pub fn max(self, other: Self) -> ConstructiveReal {
        (self.clone() - other.clone()).select(other, self)
    }

    pub fn min(self, other: Self) -> ConstructiveReal {
        (self.clone() - other.clone()).select(self, other)
    }

    pub fn abs(self) -> ConstructiveReal {
        self.clone().select(-self.clone(), self)
    }

    /// The exponential function, that is e**self
    pub fn exp(self) -> NumResult<ConstructiveReal> {
        let low_prec = -10;
        let rough_appr = self.get_appr(low_prec)?;
        // Handle negative arguments directly; negating and computing inverse
        // can be very expensive.
        if rough_appr > BigInt::from(2) || rough_appr < BigInt::from(-2) {
            let square_root = (self >> 1)?.exp()?;
            Ok(square_root.clone() * square_root)
        } else {
            Ok(ConstructiveReal {
                cancellation_token: self.cancellation_token.clone(),
                t: Rc::new(PrescaledExpConstructive(self)),
                ..ConstructiveReal::default()
            })
        }
    }

    /// The natural (base e) logarithm
    pub fn ln(self) -> NumResult<ConstructiveReal> {
        let LOW_LN_LIMIT: BigInt = BigInt::from(8);
        let HIGH_LN_LIMIT: BigInt = BigInt::from(16 + 8 /* 1.5 */);
        let SCALED_4: BigInt = BigInt::from(4 * 16);
        let TEN_NINTHS: ConstructiveReal = ConstructiveReal::from(10) / ConstructiveReal::from(9);
        let TWENTYFIVE_TWENTYFOURTHS: ConstructiveReal =
            ConstructiveReal::from(25) / ConstructiveReal::from(24);
        let EIGHTYONE_EIGHTYETHS: ConstructiveReal =
            ConstructiveReal::from(81) / ConstructiveReal::from(80);
        let LN2_1: ConstructiveReal = ConstructiveReal::from(7) * TEN_NINTHS.clone().simple_ln();
        let LN2_2: ConstructiveReal =
            ConstructiveReal::from(2) * TWENTYFIVE_TWENTYFOURTHS.clone().simple_ln();
        let LN2_3: ConstructiveReal =
            ConstructiveReal::from(3) * EIGHTYONE_EIGHTYETHS.clone().simple_ln();
        let LN2: ConstructiveReal = LN2_1.clone() - LN2_2.clone() + LN2_3.clone();

        let low_prec = -4;
        let rough_appr = self.get_appr(low_prec)?; /* In sixteenths */
        if rough_appr < BigInt::zero() {
            return Err(DomainViolation(LogarithmOfNegative));
        };
        if rough_appr <= LOW_LN_LIMIT {
            return Ok(-self.inverse().ln()?);
        }

        if rough_appr >= HIGH_LN_LIMIT {
            return if rough_appr <= SCALED_4 {
                let quarter = self.sqrt().sqrt().ln()?;
                quarter << 2
            } else {
                let extra_bits = rough_appr.bits() - 3;
                let scaled_result = (self >> extra_bits as i32)?.ln()?;
                Ok(scaled_result + (ConstructiveReal::from(extra_bits) * LN2))
            };
        }
        Ok(self.simple_ln())
    }

    pub fn simple_ln(self) -> ConstructiveReal {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Rc::new(PrescaledLnConstructive(self - ConstructiveReal::from(1))),
            ..ConstructiveReal::default()
        }
    }

    pub fn sqrt(self) -> ConstructiveReal {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Rc::new(SquareRootConstructive(self)),
            ..ConstructiveReal::default()
        }
    }

    pub fn atan_reciporical(n: i32) -> ConstructiveReal {
        ConstructiveReal {
            t: Rc::new(InverseTanReciprocalConstructive(n)),
            ..ConstructiveReal::default()
        }
    }

    pub fn pi() -> ConstructiveReal {
        let four = Self::from(4);
        four.clone() * (four.clone() * Self::atan_reciporical(5) - Self::atan_reciporical(239))
    }

    pub fn sin(self) -> NumResult<ConstructiveReal> {
        (Self::pi() / ConstructiveReal::from(2) - self).cos()
    }

    pub fn cos(self) -> NumResult<ConstructiveReal> {
        let rough_appr = self.get_appr(-1)?;
        let abs_rough_appr = rough_appr.abs();
        if abs_rough_appr >= BigInt::from(6) {
            // Subtract multiples of PI
            let multiplier = rough_appr / BigInt::from(6);
            let adjustment = Self::pi() * ConstructiveReal::from(multiplier.clone());
            if (multiplier & BigInt::one()).sign() == Sign::NoSign {
                Ok(-(self - adjustment).cos()?)
            } else {
                (self - adjustment).cos()
            }
        } else if abs_rough_appr >= BigInt::from(2) {
            // Scale further with double angle formula
            let cos_half = (self >> 1)?.cos()?;
            Ok(((cos_half.clone() * cos_half) << 1)? - ConstructiveReal::from(1))
        } else {
            Ok(ConstructiveReal {
                cancellation_token: self.cancellation_token.clone(),
                t: Rc::new(PrescaledCosConstructive(self)),
                ..ConstructiveReal::default()
            })
        }
    }

    pub fn with_cancellation_token(mut self, cancellation_token: CancellationToken) -> Self {
        self.cancellation_token = cancellation_token;
        self
    }
}

impl Default for ConstructiveReal {
    fn default() -> Self {
        ConstructiveReal {
            t: Rc::new(InvalidConstructive()),
            current_approximation: Rc::new(RefCell::new(None)),
            cancellation_token: CancellationToken::new(false),
        }
    }
}

impl From<BigInt> for ConstructiveReal {
    fn from(n: BigInt) -> Self {
        ConstructiveReal {
            t: Rc::new(BigIntegerConstructive(n)),
            ..ConstructiveReal::default()
        }
    }
}

impl From<i32> for ConstructiveReal {
    fn from(value: i32) -> Self {
        ConstructiveReal::from(BigInt::from(value))
    }
}

impl From<i64> for ConstructiveReal {
    fn from(value: i64) -> Self {
        ConstructiveReal::from(BigInt::from(value))
    }
}

impl From<u32> for ConstructiveReal {
    fn from(value: u32) -> Self {
        ConstructiveReal::from(BigInt::from(value))
    }
}

impl From<u64> for ConstructiveReal {
    fn from(value: u64) -> Self {
        ConstructiveReal::from(BigInt::from(value))
    }
}

impl TryFrom<f64> for ConstructiveReal {
    type Error = NumError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_nan() {
            return Err(InternalError(ConstructiveRealFromNan));
        }
        if value.is_infinite() {
            return Err(InternalError(ConstructiveRealFromInf));
        }
        let negative = value < 0.0;
        let bits = value.abs().to_bits();
        let mut mantissa = bits & 0xfffffffffffff;
        let biased_exp = (bits >> 52) as u32;
        let exp = biased_exp - 1075;
        if biased_exp != 0 {
            mantissa += 1_u64 << 52;
        } else {
            mantissa <<= 1;
        }

        let result = (ConstructiveReal::from(mantissa) << exp as i32)?;
        Ok(if negative { -result } else { result })
    }
}

impl TryFrom<f32> for ConstructiveReal {
    type Error = NumError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        ConstructiveReal::try_from(value as f64)
    }
}

impl Display for ConstructiveReal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.to_string(10, 10).unwrap_or("Error".to_string())
        )
    }
}

impl Add for ConstructiveReal {
    type Output = ConstructiveReal;

    fn add(self, rhs: Self) -> Self::Output {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Rc::new(AddConstructive {
                op1: self,
                op2: rhs,
            }),
            ..ConstructiveReal::default()
        }
    }
}

impl Shl<i32> for ConstructiveReal {
    type Output = NumResult<ConstructiveReal>;

    fn shl(self, rhs: i32) -> Self::Output {
        check_prec(rhs)?;
        Ok(ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Rc::new(ShiftConstructive {
                op: self,
                count: rhs,
            }),
            ..ConstructiveReal::default()
        })
    }
}

impl Shr<i32> for ConstructiveReal {
    type Output = NumResult<ConstructiveReal>;

    fn shr(self, rhs: i32) -> Self::Output {
        check_prec(rhs)?;
        Ok(ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Rc::new(ShiftConstructive {
                op: self,
                count: -rhs,
            }),
            ..ConstructiveReal::default()
        })
    }
}

impl Sub for ConstructiveReal {
    type Output = ConstructiveReal;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul for ConstructiveReal {
    type Output = ConstructiveReal;

    fn mul(self, rhs: Self) -> Self::Output {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Rc::new(MultiplyConstructive {
                op1: self,
                op2: rhs,
            }),
            ..ConstructiveReal::default()
        }
    }
}

impl Div for ConstructiveReal {
    type Output = ConstructiveReal;

    #[allow(clippy::suspicious_arithmetic_impl, reason = "This is the intended implementation.")]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl Neg for ConstructiveReal {
    type Output = ConstructiveReal;

    fn neg(self) -> Self::Output {
        ConstructiveReal {
            cancellation_token: self.cancellation_token.clone(),
            t: Rc::new(NegatedConstructive(self)),
            ..ConstructiveReal::default()
        }
    }
}

impl From<ConstructiveReal> for NumResult<BigInt> {
    fn from(value: ConstructiveReal) -> Self {
        value.clone().get_appr(0)
    }
}

impl From<ConstructiveReal> for NumResult<i32> {
    fn from(value: ConstructiveReal) -> Self {
        value.clone().get_appr(0).map(|n| n.to_i32().unwrap())
    }
}

impl From<ConstructiveReal> for NumResult<f64> {
    fn from(value: ConstructiveReal) -> Self {
        let mut value = value.clone();
        let my_msd = value.iter_msd(-1080 /* slightly > exp. range */)?;
        if my_msd == i32::MIN {
            return Ok(0.0);
        };
        let needed_prec = my_msd - 60;
        let scaled_int = value.get_appr(needed_prec)?.to_i64().unwrap() as f64;
        let may_underflow = needed_prec < -1000;
        let mut scaled_int_rep = scaled_int.to_bits();
        let exp_adj = if may_underflow {
            needed_prec + 96
        } else {
            needed_prec
        } as u64;
        let orig_exp = (scaled_int_rep >> 52) & 0x7ff;
        if ((orig_exp.overflowing_add(exp_adj).0) & !0x7ff) != 0 {
            // overflow
            if scaled_int < 0.0 {
                return Ok(f64::NEG_INFINITY);
            } else {
                return Ok(f64::INFINITY);
            }
        }

        scaled_int_rep = scaled_int_rep.overflowing_add(exp_adj << 52).0;
        let result = f64::from_bits(scaled_int_rep);
        if may_underflow {
            let two48 = (1_u64 << 48) as f64;
            Ok(result / two48 / two48)
        } else {
            Ok(result)
        }
    }
}

fn bound_log2(n: i32) -> i32 {
    let abs_n = n.abs();
    ((abs_n + 1) as f64).log(2.).ceil() as i32
}

/// Check that a precision is at least a factor of 8 away from
/// overflowng the integer used to hold a precision spec.
/// We generally perform this check early on, and then convince
/// ourselves that none of the operations performed on precisions
/// inside a function can generate an overflow.
fn check_prec(n: i32) -> NumResult<()> {
    let high = n >> 28;

    // if n is not in danger of overflowing, then the 4 high order
    // bits should be identical. Thus high is either 0 or -1.
    // The rest of this is to test for either of those in a way
    // that should be as cheap as possible.
    let high_shifted = n >> 29;
    if (high ^ high_shifted) != 0 {
        Err(PrecisionOverflow)
    } else {
        Ok(())
    }
}

fn shift(k: BigInt, n: i32) -> BigInt {
    if n == 0 {
        k
    } else if n < 0 {
        k.div_floor(&BigInt::from(2).pow(n.unsigned_abs()))
    } else {
        k.mul(&BigInt::from(2).pow(n as u32))
    }
}

fn scale(k: BigInt, n: i32) -> BigInt {
    if n >= 0 {
        shift(k, n)
    } else {
        let adj_k = shift(k, n + 1).add(&BigInt::from(1));
        shift(adj_k, -1)
    }
}

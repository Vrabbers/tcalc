use std::ops::Rem;
use num::{BigInt, FromPrimitive, Integer, One, Signed, Zero};
use crate::error::DomainViolation::{NthRoot};
use crate::error::NumError::DomainViolation;
use crate::error::NumResult;

static EXTRACT_SQUARE_MAX_LEN: u64 = 5000;

pub trait BigIntExtensions {
    /// Return nth root of x if it's an integer
    fn nth_root_safe(&self, n: u32) -> NumResult<BigInt>;
    /// Return a pair p, such that p[0]^2 * p[1] = x.
    /// x is assumed positive. We try to maximize p[0], but not very hard.
    fn extract_square(self) -> (BigInt, BigInt);
    /// Calculate the remainder when self is divided by other. Always returns a positive result.
    fn rem_wraparound(self, other: &BigInt) -> BigInt;
}

impl BigIntExtensions for BigInt {
    fn nth_root_safe(&self, n: u32) -> NumResult<BigInt> {
        if n == 0 {
            Err(DomainViolation(NthRoot(0.)))
        } else if self.is_negative() && n.is_even() {
            Err(DomainViolation(NthRoot(n as f32)))
        } else {
            Ok(self.nth_root(n))
        }
    }

    /// Return a pair p, such that p[0]^2 * p[1] = x.
    /// x is assumed positive. We try to maximize p[0], but not very hard.
    fn extract_square(self) -> (BigInt, BigInt) {
        let some_primes = [BigInt::from_i32(2).unwrap(), BigInt::from_i32(3).unwrap(),
            BigInt::from_i32(5).unwrap(), BigInt::from_i32(7).unwrap(), BigInt::from_i32(11).unwrap(),
            BigInt::from_i32(13).unwrap()];

        let mut square = BigInt::one();
            let mut rest = self;
            if rest.bits() > EXTRACT_SQUARE_MAX_LEN {
                return (square, rest);
            }

        for prime in some_primes {
            if rest == BigInt::one() {
                break;
            }

            loop {
                let this_prime_square = prime.clone().pow(2);
                let qr = rest.div_mod_floor(&this_prime_square);
                if qr.1.is_zero() {
                    rest = qr.0;  // Remaining quotient.
                    square *= prime.clone();
                } else {
                    break;
                }
            }
        }

        for i in 1..10 {
            let qr = rest.div_mod_floor(&BigInt::from_i32(i).unwrap());
            if qr.1.is_zero() {
                let root = qr.0.nth_root(2);
                rest = BigInt::from_i32(i).unwrap();
                square *= root;
                break;
            }
        }

        (square, rest)
    }

    fn rem_wraparound(self, other: &BigInt) -> BigInt {
        let rem = self % other;

        // Ensure rem wraps around if it is negative
        if rem.is_negative() {
            rem + other
        } else {
            rem
        }
    }
}
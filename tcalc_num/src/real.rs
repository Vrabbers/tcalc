use crate::error::NumError::{DivisionByZero, InternalError};
use crate::error::NumResult;
use crate::transcendentals::pi::pi_digits;
use num::{BigInt, BigRational, FromPrimitive, One, Signed, Zero};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Real {
    rat: BigRational,
    re: RealPart,
}

impl Real {
    pub fn from_u32(n: u32) -> Self {
        Self {
            rat: BigRational::from_u32(n).unwrap(),
            re: RealPart::One,
        }
    }

    pub fn from_i32(n: i32) -> Self {
        Self {
            rat: BigRational::from_i32(n).unwrap(),
            re: RealPart::One,
        }
    }

    pub fn from_rational(rat: BigRational) -> Self {
        Self {
            rat,
            re: RealPart::One,
        }
    }

    pub fn pi() -> Self {
        Self {
            rat: BigRational::one(),
            re: RealPart::Pi,
        }
    }

    pub fn evaluate_to_string(self) -> impl Iterator<Item = String> {
        gen move {
            match self.re {
                RealPart::One => yield format!("{}", self.rat),
                RealPart::Pi => {
                    let pi = fold_rational_iterator(self.re.as_rational(), 200);
                    let result = self.rat.mul(pi);

                    if result.is_negative() {
                        yield "-".into();
                    }

                    let trunc = result.trunc();
                    yield format!("{}", trunc);
                    let fract = result.fract();
                    if fract != BigRational::zero() {
                        yield format!(".{}", fract);
                    }
                }
                RealPart::Series => {
                    let series = fold_rational_iterator(self.re.as_rational(), 100);
                    yield format!("{}", self.rat.mul(series))
                }
            }
        }
    }

    pub fn evaluate_to_string_with_dp(self, precision: usize) -> String {
        self.evaluate_to_string()
            .take(precision)
            .collect::<Vec<String>>()
            .join("")
    }
}

impl Add for Real {
    type Output = NumResult<Self>;
    fn add(self, other: Self) -> Self::Output {
        if self.re == other.re {
            match self.re {
                RealPart::One => Ok(Self {
                    rat: self.rat + other.rat,
                    re: self.re,
                }),
                RealPart::Pi => Ok(Self {
                    rat: self.rat + other.rat,
                    re: self.re,
                }),
                RealPart::Series => Err(InternalError),
            }
        } else {
            Err(InternalError)
        }
    }
}

impl Sub for Real {
    type Output = NumResult<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs.mul(Real::from_i32(-1))?)
    }
}

impl Mul for Real {
    type Output = NumResult<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        // TODO
        Ok(Self {
            rat: self.rat * rhs.rat,
            re: self.re,
        })
    }
}

impl Div for Real {
    type Output = NumResult<Self>;
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.rat.is_zero() {
            return Err(DivisionByZero)
        }

        // TODO
        Ok(Self {
            rat: self.rat * rhs.rat,
            re: self.re,
        })
    }
}

fn fold_rational_iterator(
    iterator: impl Iterator<Item = BigRational>,
    steps: usize,
) -> BigRational {
    iterator
        .take(steps)
        .fold(BigRational::zero(), |acc, x| acc + x)
}

#[derive(Debug, PartialEq, Clone)]
enum RealPart {
    One,
    Pi,
    Series,
}

impl RealPart {
    fn as_rational(&self) -> impl Iterator<Item = BigRational> {
        gen move {
            match self {
                RealPart::One => yield BigRational::from_u32(1).unwrap(),
                RealPart::Pi => {
                    let pi_iterator = pi_digits();
                    let mut denom = BigInt::one();

                    for pi_digit in pi_iterator {
                        yield BigRational::new_raw(pi_digit, denom.clone());
                        denom = denom.mul(10);
                    }
                }
                RealPart::Series => yield BigRational::from_u32(1).unwrap(),
            }
        }
        .into_iter()
    }
}

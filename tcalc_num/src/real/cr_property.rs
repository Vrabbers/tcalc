use num::{BigRational, FromPrimitive, One, Signed, ToPrimitive, Zero};
use crate::constructive_real::constants::{LN_10, ONE, PI};
use crate::constructive_real::{ConstructiveReal, ConstructiveRealKnownValue};
use crate::error::NumResult;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum CRPropertyType {
    One,
    Pi,
    Sqrt,
    Exp,
    Ln,
    Log,
    SinPi,
    TanPi,
    Asin,
    Atan,
    Irrational,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct CRProperty {
    pub kind: CRPropertyType,
    pub arg: Option<BigRational>,
}

impl CRProperty {
    pub fn one() -> Self {
        Self {
            kind: CRPropertyType::One,
            arg: None,
        }
    }

    pub fn pi() -> Self {
        Self {
            kind: CRPropertyType::Pi,
            arg: None,
        }
    }

    pub fn irrational() -> Self {
        Self {
            kind: CRPropertyType::Irrational,
            arg: None,
        }
    }

    pub fn sqrt_2() -> Self {
        Self {
            kind: CRPropertyType::Sqrt,
            arg: Some(BigRational::from_i32(2).unwrap()),
        }
    }

    pub fn sqrt_3() -> Self {
        Self {
            kind: CRPropertyType::Sqrt,
            arg: Some(BigRational::from_i32(3).unwrap()),
        }
    }

    pub fn e() -> Self {
        Self {
            kind: CRPropertyType::Exp,
            arg: Some(BigRational::one()),
        }
    }

    pub fn ln_10() -> Self {
        Self {
            kind: CRPropertyType::Ln,
            arg: Some(BigRational::from_i32(10).unwrap()),
        }
    }

    pub fn determines_cr(&self) -> bool {
        self.kind != CRPropertyType::Irrational
    }

    pub fn new(kind: CRPropertyType, arg: BigRational) -> Self {
        // This enforces requirements on arg.

        match kind {
            CRPropertyType::One => Self::one(),
            CRPropertyType::Pi => Self::pi(),
            CRPropertyType::Irrational => Self::irrational(),
            _ => {
                if kind == CRPropertyType::Sqrt && arg.is_one() {
                    Self::one()
                } else if kind == CRPropertyType::Exp && arg.is_zero() {
                    Self::one()
                } else {
                    Self {
                        kind,
                        arg: Some(arg),
                    }
                }
            }
        }
    }

    pub fn is_one(&self) -> bool {
        self.kind == CRPropertyType::One
    }

    pub fn is_pi(&self) -> bool {
        self.kind == CRPropertyType::Pi
    }

    pub fn is_unknown_irrational(&self) -> bool {
        self.kind == CRPropertyType::Irrational
    }

    pub fn is_nonzero(&self) -> bool {
        match self.kind {
            CRPropertyType::One => {
                true
            }
            CRPropertyType::Pi => {
                true
            }
            CRPropertyType::Irrational => {
                true
            }
            CRPropertyType::Exp => {
                // It would be correct to always answer true. But we intentionally fail to provide the
                // guarantee for large negative arguments, since it would be expensive to actually
                // distinguish the value from zero, and answering true often results in such an attempt.
                self.arg >= BigRational::from_i32(-10000)
            }
            CRPropertyType::Ln => {
                // arg > 1
                true
            }
            CRPropertyType::Log => {
                // arg > 1
                true
            }
            CRPropertyType::Sqrt => {
                // arg > 0
                true
            }
            CRPropertyType::SinPi => {
                // arg != 0
                true
            }
            CRPropertyType::TanPi => {
                // arg != 0
                true
            }
            CRPropertyType::Asin => {
                // arg != 0
                true
            }
            CRPropertyType::Atan => {
                // arg != 0
                true
            }
        }
    }

    /// Return the constructive real described by the property.  We could instead omit storing the CR
    /// value when we have a sufficiently descriptive property.  But that might hurt performance
    /// slightly, since we would sometimes lose the benefit of prior argument evaluations.
    pub fn cr(&self) -> NumResult<Option<ConstructiveReal>> {
        match self.kind {
            CRPropertyType::One => {
                Ok(Some(ONE.clone()))
            }
            CRPropertyType::Pi => {
                Ok(Some(PI.clone()))
            }
            CRPropertyType::Exp => {
                if self.arg.is_none() {
                    Ok(None)
                } else {
                    Ok(Some(ConstructiveReal::from(self.arg.clone().unwrap()).exp()?))
                }
            }
            CRPropertyType::Ln => {
                if self.arg.is_none() {
                    Ok(None)
                } else {
                    Ok(Some(ConstructiveReal::from(self.arg.clone().unwrap()).ln()?))
                }
            }
            CRPropertyType::Log => {
                if self.arg.is_none() {
                    Ok(None)
                } else {

                    Ok(Some(ConstructiveReal::from(self.arg.clone().unwrap()).ln()? / LN_10.clone()))
                }
            }
            CRPropertyType::Sqrt => {
                if self.arg.is_none() {
                    Ok(None)
                } else {
                    Ok(Some(ConstructiveReal::from(self.arg.clone().unwrap()).sqrt()))
                }
            }
            CRPropertyType::SinPi => {
                if self.arg.is_none() {
                    Ok(None)
                } else {
                    Ok(Some((ConstructiveReal::from(self.arg.clone().unwrap()) * PI.clone()).sin()?))
                }
            }
            CRPropertyType::TanPi => {
                todo!()
            }
            CRPropertyType::Asin => {
                todo!()
            }
            CRPropertyType::Atan => {
                todo!()
            }
            CRPropertyType::Irrational => {
                Ok(None)
            }
        }
    }

    pub fn msb_bound(&self) -> i32 {
        match self.kind {
            CRPropertyType::One => {0}
            CRPropertyType::Pi => {1}
            CRPropertyType::Sqrt => {
                let wnb = self.arg.clone().unwrap().whole_number_bits();
                if wnb == i32::MIN {
                    wnb
                } else {
                    (wnb >> 1) - 2
                }
            }
            CRPropertyType::Ln | CRPropertyType::Log => {
                if self.arg >= BigRational::from_i32(2) {
                    // ln(2) > log(2) > 1/4
                    -2
                } else {
                    // Argument is in the vicinity of 1, result may be near zero.
                    i32::MIN
                }
            }
            CRPropertyType::Exp => {
                let result = self.arg.clone().unwrap().floor();
                if result.bit_length() <= 30 {
                    if !result.is_negative() {
                        // multiply by a bit less than 1/ln(2).
                        (result.to_i32().unwrap() / 5) * 7
                    } else {
                        // multiply by a bit more than 1/ln(2), making sure we err on correct side.
                        (result.to_i32().unwrap() / 2 - 1) * 3
                    }
                } else if result.is_positive() {
                    // Positive and takes > 30 bits to represent.
                    return 100_000_000
                } else {
                    i32::MIN
                }
            }
            CRPropertyType::SinPi | CRPropertyType::TanPi | CRPropertyType::Asin | CRPropertyType::Atan => {
                // These all behave like x or <pi>x near zero. Thus the following very rough estimate holds.
                if self.arg.clone().unwrap() > BigRational::new(1.into(), 1024.into()) {
                    -11
                } else {
                    i32::MIN
                }
            }
            CRPropertyType::Irrational => {
                i32::MIN
            }
        }
    }
}

impl From<ConstructiveReal> for Option<CRProperty> {
    /// Try to compute a property describing the constructive real.
    /// Recognizes only a few specific constants.
    fn from(value: ConstructiveReal) -> Self {
        match value.known_value {
            Some(ConstructiveRealKnownValue::One) => Some(CRProperty::one()),
            Some(ConstructiveRealKnownValue::Pi) => Some(CRProperty::pi()),
            Some(ConstructiveRealKnownValue::Sqrt2) => Some(CRProperty::sqrt_2()),
            Some(ConstructiveRealKnownValue::Sqrt3) => Some(CRProperty::sqrt_3()),
            Some(ConstructiveRealKnownValue::E) => Some(CRProperty::e()),
            Some(ConstructiveRealKnownValue::Ln10) => Some(CRProperty::ln_10()),
            _ => None,
        }
    }
}

trait RationalExtensions {
    fn whole_number_bits(&self) -> i32;
    fn bit_length(&self) -> i32;
}

impl RationalExtensions for BigRational {
    fn whole_number_bits(&self) -> i32 {
        if self.is_zero() {
            i32::MIN
        } else {
            (self.numer().bits() - self.denom().bits()) as i32
        }
    }

    fn bit_length(&self) -> i32 {
        (self.numer().bits() + self.denom().bits()) as i32
    }
}
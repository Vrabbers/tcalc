use crate::angle_unit::AngleUnit;
use crate::constructive_real::constants::{LN_10, ONE, PI};
use crate::constructive_real::{ConstructiveReal, ConstructiveRealKnownValue};
use crate::error::NumResult;
use crate::maths_symbols::MathsSymbols;
use crate::rational_extensions::RationalExtensions;
use num::bigint::Sign;
use num::{BigRational, FromPrimitive, One, Signed, ToPrimitive, Zero};

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone)]
#[repr(u8)]
pub enum CRProperty {
    One = 1,
    Pi = 2,
    Sqrt(BigRational) = 3,
    Exp(BigRational) = 4,
    Ln(BigRational) = 5,
    Log(BigRational) = 6,
    SinPi(BigRational, bool) = 7,
    TanPi(BigRational, bool) = 8,
    Asin(BigRational) = 9,
    Atan(BigRational) = 10,
    Irrational = 11,
}

impl CRProperty {
    pub fn one() -> Self {
        CRProperty::One
    }

    pub fn pi() -> Self {
        CRProperty::Pi
    }

    pub fn irrational() -> Self {
        CRProperty::Irrational
    }

    pub fn sqrt_2() -> Self {
        CRProperty::Sqrt(BigRational::from_i32(2).unwrap())
    }

    pub fn sqrt_3() -> Self {
        CRProperty::Sqrt(BigRational::from_i32(3).unwrap())
    }

    pub fn e() -> Self {
        CRProperty::Exp(BigRational::one())
    }

    pub fn ln_10() -> Self {
        CRProperty::Ln(BigRational::from_i32(10).unwrap())
    }

    pub fn determines_cr(&self) -> bool {
        self != &CRProperty::Irrational
    }

    pub fn new(kind: CRProperty) -> Self {
        // This enforces requirements on arg.

        match kind {
            CRProperty::One => Self::one(),
            CRProperty::Pi => Self::pi(),
            CRProperty::Irrational => Self::irrational(),
            CRProperty::Sqrt(arg) if arg.is_one() => Self::one(),
            CRProperty::Exp(arg) if arg.is_zero() => Self::one(),
            _ => kind,
        }
    }

    /// Return a property corresponding to sin(pi*arg), normalizing arg to the correct range. Caller is
    /// responsible for ensuring that the argument is not one that leads to a rational result.
    /// The negate field of the result is set if the property corresponds to the negated argument,
    /// rather than the argument itself.
    /// Returns None if we can't normalize the argument.
    pub fn new_sin_pi(arg: BigRational) -> Option<Self> {
        let mut n_arg = arg.reduced_arg()?;
        if n_arg.too_big() {
            return None;
        }

        let mut neg = false;
        if n_arg >= BigRational::new(1.into(), 2.into()) {
            // sin(x) = sin(pi - x)
            n_arg = BigRational::one() - n_arg;
        }
        if !n_arg.too_big() && n_arg.sign() == Sign::Minus {
            n_arg = -n_arg;
            neg = true;
        }
        if n_arg.too_big() {
            None
        } else {
            Some(Self::new(CRProperty::SinPi(n_arg, neg)))
        }
    }

    /// Return a property corresponding to tan(pi*arg), normalizing arg to the correct range. Caller is
    /// responsible for ensuring that the argument is not one that leads to a rational result.
    /// The negate field of the result is set if the property corresponds to the negated argument,
    /// rather than the argument itself.
    /// Returns None if we can't normalize the argument.
    pub fn new_tan_pi(arg: BigRational) -> Option<Self> {
        let mut n_arg = arg.reduced_arg()?;
        if n_arg.too_big() {
            return None;
        }

        let mut neg = false;
        if n_arg >= BigRational::new(1.into(), 2.into()) {
            // tan(x) = tan(x - pi)
            n_arg -= BigRational::one();
        }
        if !n_arg.too_big() && n_arg.sign() == Sign::Minus {
            n_arg = -n_arg;
            neg = true;
        }
        if n_arg.too_big() {
            None
        } else {
            Some(Self::new(CRProperty::TanPi(n_arg, neg)))
        }
    }

    pub fn is_one(&self) -> bool {
        self == &CRProperty::One
    }

    pub fn is_pi(&self) -> bool {
        self == &CRProperty::Pi
    }

    pub fn is_unknown_irrational(&self) -> bool {
        self == &CRProperty::Irrational
    }

    pub fn is_nonzero(&self) -> bool {
        match self {
            CRProperty::One => true,
            CRProperty::Pi => true,
            CRProperty::Irrational => true,
            CRProperty::Exp(arg) => {
                // It would be correct to always answer true. But we intentionally fail to provide the
                // guarantee for large negative arguments, since it would be expensive to actually
                // distinguish the value from zero, and answering true often results in such an attempt.
                arg >= &BigRational::from_i32(-10000).unwrap()
            }
            CRProperty::Ln(_) => {
                // arg > 1
                true
            }
            CRProperty::Log(_) => {
                // arg > 1
                true
            }
            CRProperty::Sqrt(_) => {
                // arg > 0
                true
            }
            CRProperty::SinPi(_, _) => {
                // arg != 0
                true
            }
            CRProperty::TanPi(_, _) => {
                // arg != 0
                true
            }
            CRProperty::Asin(_) => {
                // arg != 0
                true
            }
            CRProperty::Atan(_) => {
                // arg != 0
                true
            }
        }
    }

    /// Return the constructive real described by the property.  We could instead omit storing the CR
    /// value when we have a sufficiently descriptive property.  But that might hurt performance
    /// slightly, since we would sometimes lose the benefit of prior argument evaluations.
    pub fn cr(&self) -> NumResult<Option<ConstructiveReal>> {
        match self {
            CRProperty::One => Ok(Some(ONE.clone())),
            CRProperty::Pi => Ok(Some(PI.clone())),
            CRProperty::Exp(arg) => Ok(Some(ConstructiveReal::from(arg.clone()).exp()?)),
            CRProperty::Ln(arg) => Ok(Some(ConstructiveReal::from(arg.clone()).ln()?)),
            CRProperty::Log(arg) => Ok(Some(
                ConstructiveReal::from(arg.clone()).ln()? / LN_10.clone(),
            )),
            CRProperty::Sqrt(arg) => Ok(Some(ConstructiveReal::from(arg.clone()).sqrt())),
            CRProperty::SinPi(arg, _) => Ok(Some(
                (ConstructiveReal::from(arg.clone()) * PI.clone()).sin()?,
            )),
            CRProperty::TanPi(arg, _) => Ok(Some(
                (ConstructiveReal::from(arg.clone()) * PI.clone()).tan()?,
            )),
            CRProperty::Asin(arg) => Ok(Some(ConstructiveReal::from(arg.clone()).asin()?)),
            CRProperty::Atan(arg) => Ok(Some(ConstructiveReal::from(arg.clone()).atan()?)),
            CRProperty::Irrational => Ok(None),
        }
    }

    pub fn msb_bound(&self) -> i32 {
        match self {
            CRProperty::One => 0,
            CRProperty::Pi => 1,
            CRProperty::Sqrt(arg) => {
                let wnb = arg.whole_number_bits();
                if wnb == i32::MIN { wnb } else { (wnb >> 1) - 2 }
            }
            CRProperty::Ln(arg) | CRProperty::Log(arg) => {
                if arg >= &BigRational::from_i32(2).unwrap() {
                    // ln(2) > log(2) > 1/4
                    -2
                } else {
                    // Argument is in the vicinity of 1, result may be near zero.
                    i32::MIN
                }
            }
            CRProperty::Exp(arg) => {
                let result = arg.floor();
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
                    100_000_000
                } else {
                    i32::MIN
                }
            }
            CRProperty::SinPi(arg, _)
            | CRProperty::TanPi(arg, _)
            | CRProperty::Asin(arg)
            | CRProperty::Atan(arg) => {
                // These all behave like x or <pi>x near zero. Thus the following very rough estimate holds.
                if arg > &BigRational::new(1.into(), 1024.into()) {
                    -11
                } else {
                    i32::MIN
                }
            }
            CRProperty::Irrational => i32::MIN,
        }
    }

    /// If the property indicates the constructive real has a simple symbolic representation, return
    /// it. The name is intended to be appended to the rational multiplier, so the name of one is the
    /// empty string. If subsuperscript is true, use subscripts and superscripts to render the
    /// embedded rational.
    pub fn cr_symbolic(&self, ang: AngleUnit, subsuperscript: bool) -> Option<String> {
        // TODO: This currently ignores translation issues.
        if self.is_unknown_irrational() {
            return None;
        }

        if self.is_one() {
            return Some("".to_string());
        }

        if self.is_pi() {
            return Some(MathsSymbols::Pi.to_string());
        }

        if let CRProperty::Exp(exp_arg) = self {
            if exp_arg.is_one() {
                return Some("e".to_string());
            } else {
                return Some(format!("exp({})", exp_arg.to_nice_string(subsuperscript)));
            }
        }

        if let CRProperty::Sqrt(sqrt_arg) = self {
            if sqrt_arg.is_integer() {
                return Some(format!("{}{}", MathsSymbols::Sqrt, sqrt_arg.to_integer()));
            } else {
                return Some(format!(
                    "{}({})",
                    MathsSymbols::Sqrt,
                    sqrt_arg.to_nice_string(subsuperscript)
                ));
            }
        }

        if let CRProperty::Ln(ln_arg) = self {
            return Some(format!("ln({})", ln_arg.to_nice_string(subsuperscript)));
        }

        if let CRProperty::Log(log_arg) = self {
            return Some(format!("log({})", log_arg.to_nice_string(subsuperscript)));
        }

        if let CRProperty::SinPi(sin_arg, _) = self {
            return Some(format!(
                "sin({})",
                sin_arg.symbolic_pi_multiple(ang, subsuperscript)
            ));
        }

        if let CRProperty::TanPi(tan_arg, _) = self {
            return Some(format!(
                "tan({})",
                tan_arg.symbolic_pi_multiple(ang, subsuperscript)
            ));
        }

        if let CRProperty::Asin(asin_arg) = self {
            return Some(format!(
                "asin({}){}",
                asin_arg.to_nice_string(subsuperscript),
                ang
            ));
        }

        if let CRProperty::Atan(atan_arg) = self {
            return Some(format!(
                "atan({}){}",
                atan_arg.to_nice_string(subsuperscript),
                ang
            ));
        }

        None
    }

    /// Is self known to be algebraic (as opposed to transcendental)? Currently only produces meaningful
    /// results for the above known special constructive reals.
    pub fn definitely_algebraic(&self) -> bool {
        matches!(
            self,
            CRProperty::One
                | CRProperty::Sqrt(_)
                | CRProperty::SinPi(_, _)
                | CRProperty::TanPi(_, _)
        )
    }

    pub fn get_arg(&self) -> Option<&BigRational> {
        match self {
            CRProperty::One => None,
            CRProperty::Pi => None,
            CRProperty::Sqrt(arg) => Some(arg),
            CRProperty::Exp(arg) => Some(arg),
            CRProperty::Ln(arg) => Some(arg),
            CRProperty::Log(arg) => Some(arg),
            CRProperty::SinPi(arg, _) => Some(arg),
            CRProperty::TanPi(arg, _) => Some(arg),
            CRProperty::Asin(arg) => Some(arg),
            CRProperty::Atan(arg) => Some(arg),
            CRProperty::Irrational => None,
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

pub trait OptionalCRProperty {
    fn definitely_algebraic(&self) -> bool;
    fn is_one(&self) -> bool;
    fn is_pi(&self) -> bool;
    fn is_nonzero(&self) -> bool;
    fn cr_symbolic(&self, ang: AngleUnit, subsuperscript: bool) -> Option<String>;
    fn is_unknown_irrational(&self) -> bool;
}

impl OptionalCRProperty for Option<CRProperty> {
    fn definitely_algebraic(&self) -> bool {
        if let Some(p) = self {
            p.definitely_algebraic()
        } else {
            false
        }
    }

    fn is_one(&self) -> bool {
        if let Some(p) = self {
            p.is_one()
        } else {
            false
        }
    }

    fn is_pi(&self) -> bool {
        if let Some(p) = self { p.is_pi() } else { false }
    }

    fn is_nonzero(&self) -> bool {
        if let Some(p) = self {
            p.is_nonzero()
        } else {
            false
        }
    }

    fn cr_symbolic(&self, ang: AngleUnit, subsuperscript: bool) -> Option<String> {
        if let Some(p) = self {
            p.cr_symbolic(ang, subsuperscript)
        } else {
            None
        }
    }

    fn is_unknown_irrational(&self) -> bool {
        if let Some(p) = self {
            p.is_unknown_irrational()
        } else {
            false
        }
    }
}

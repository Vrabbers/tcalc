use num::{BigRational, FromPrimitive, One};

#[derive(Eq, PartialEq)]
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
    Irrational
}

#[derive(Eq, PartialEq)]
pub struct CRProperty {
    kind: CRPropertyType,
    arg: Option<BigRational>
}

impl CRProperty {
    pub fn one() -> Self {
        Self {
            kind: CRPropertyType::One,
            arg: None
        }
    }

    pub fn pi() -> Self {
        Self {
            kind: CRPropertyType::Pi,
            arg: None
        }
    }

    pub fn irrational(arg: BigRational) -> Self {
        Self {
            kind: CRPropertyType::Irrational,
            arg: Some(arg)
        }
    }

    pub fn sqrt_2() -> Self {
        Self {
            kind: CRPropertyType::Sqrt,
            arg: Some(BigRational::from_i32(2).unwrap())
        }
    }

    pub fn sqrt_3() -> Self {
        Self {
            kind: CRPropertyType::Sqrt,
            arg: Some(BigRational::from_i32(3).unwrap())
        }
    }

    pub fn e() -> Self {
        Self {
            kind: CRPropertyType::Exp,
            arg: Some(BigRational::one())
        }
    }

    pub fn ln_10() -> Self {
        Self {
            kind: CRPropertyType::Ln,
            arg: Some(BigRational::from_i32(10).unwrap())
        }
    }

    pub fn determines_cr(&self) -> bool {
        self.kind != CRPropertyType::Irrational
    }
}


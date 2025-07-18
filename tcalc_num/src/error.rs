#[derive(Debug, PartialEq, Clone)]
pub enum NumError {
    InternalError(InternalError),
    PrecisionOverflow,
    DivisionByZero,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InternalError {
    ConstructableRealFromNan,
    ConstructableRealFromInf,
}

pub type NumResult<T> = Result<T, NumError>;
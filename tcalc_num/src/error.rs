#[derive(Debug, PartialEq, Clone)]
pub enum NumError {
    InternalError(InternalError),
    PrecisionOverflow,
    DivisionByZero,
    DomainViolation(DomainViolation)
}

#[derive(Debug, PartialEq, Clone)]
pub enum InternalError {
    ConstructableRealFromNan,
    ConstructableRealFromInf,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DomainViolation {
    LogarithmOfNegative,
    SquareRootOfNegative,
}

pub type NumResult<T> = Result<T, NumError>;
use cancellation_token::CancellationToken;

#[derive(Debug, PartialEq, Clone)]
pub enum NumError {
    InternalError(InternalError),
    OperationCancelledError,
    PrecisionOverflow,
    DomainViolation(DomainViolation),
    Overflow,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InternalError {
    ConstructiveRealFromNan,
    ConstructiveRealFromInf,
    UnconstructableFloat,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DomainViolation {
    DivisionByZero,
    LogarithmDomainViolation(LogarithmDomainViolation),
    NthRoot(f32),
    TanDomainViolation,
    AsinDomainViolation,
    OrdinalDomainViolation(OrdinalDomainViolation),
    FactorialDomainViolation(FactorialDomainViolation),
}

#[derive(Debug, PartialEq, Clone)]
pub enum OrdinalDomainViolation {
    ZeroBaseZeroOrder,

    /// Technically a division by zero but let's split this so we get more descriptive errors
    ZeroBaseNegativeOrder,

    NegativeBaseNonIntegerOrder,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LogarithmDomainViolation {
    LogOfNegative,
    LogOfZero,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FactorialDomainViolation {
    NonIntegerBase,
    NegativeBase,
}

pub type NumResult<T> = Result<T, NumError>;

pub trait CancelCheckable {
    fn stop_if_cancelled(&self) -> NumResult<()>;
}

impl CancelCheckable for CancellationToken {
    fn stop_if_cancelled(&self) -> NumResult<()> {
        if self.is_canceled() {
            Err(NumError::OperationCancelledError)
        } else {
            Ok(())
        }
    }
}

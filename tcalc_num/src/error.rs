use cancellation_token::CancellationToken;

#[derive(Debug, PartialEq, Clone)]
pub enum NumError {
    InternalError(InternalError),
    OperationCancelledError,
    PrecisionOverflow,
    DomainViolation(DomainViolation),
}

#[derive(Debug, PartialEq, Clone)]
pub enum InternalError {
    ConstructiveRealFromNan,
    ConstructiveRealFromInf,
    UnconstructableFloat
}

#[derive(Debug, PartialEq, Clone)]
pub enum DomainViolation {
    DivisionByZero,
    LogarithmOfNegative,
    NthRoot(f32),
    TanDomainViolation,
    AsinDomainViolation,
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
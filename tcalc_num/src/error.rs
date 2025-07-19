use cancellation_token::CancellationToken;

#[derive(Debug, PartialEq, Clone)]
pub enum NumError {
    InternalError(InternalError),
    OperationCancelledError,
    PrecisionOverflow,
    DivisionByZero,
    DomainViolation(DomainViolation)
}

#[derive(Debug, PartialEq, Clone)]
pub enum InternalError {
    ConstructiveRealFromNan,
    ConstructiveRealFromInf,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DomainViolation {
    LogarithmOfNegative,
    SquareRootOfNegative,
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
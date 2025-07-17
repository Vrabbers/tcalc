#[derive(Debug, PartialEq)]
pub enum NumError {
    InternalError,
    DivisionByZero,
}

pub type NumResult<T> = Result<T, NumError>;
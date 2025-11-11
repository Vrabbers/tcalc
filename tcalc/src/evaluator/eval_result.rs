use tcalc_num::number::Number;
pub use tcalc_num::error::*;

#[derive(Clone, Debug)]
pub enum EvalValue<Num: Number> {
    Numeric(Num),
    AssignedVariable { variable_name: String, value: Num },
    Comparison(bool),
}

#[derive(Clone, Debug)]
pub enum EvalError {
    NumberError(NumError),
    AssignToConstant,
    UndefinedVariable,
    UndefinedFunction,
    InvalidArgumentCount,
    ComplexInequality,
    InvalidProgram,
}

impl From<NumError> for EvalError {
    fn from(value: NumError) -> Self {
        Self::NumberError(value)
    }
}

pub type EvalResult<Num> = Result<EvalValue<Num>, EvalError>;

use crate::evaluator::{
    builtins::basic_builtins,
    eval_result::EvalError,
};
use std::collections::HashMap;

use tcalc_num::{angle_unit::AngleUnit, number::Number};

mod builtins;

pub mod eval_result;
pub struct Evaluator<Num: Number> {
    constants: HashMap<String, Num>,
    variables: HashMap<String, Num>,
    functions: HashMap<String, Vec<EvalFunction<Num>>>,
    angle_unit: AngleUnit,
}

pub trait EvalFn<Num: Number> {
    fn call(&self, stack: &mut Vec<Num>, evaluator: &Evaluator<Num>) -> Result<(), EvalError>;
}

impl<F, Num: Number> EvalFn<Num> for F
where
    F: Fn(&mut Vec<Num>, &Evaluator<Num>) -> Result<(), EvalError>,
{
    fn call(&self, stack: &mut Vec<Num>, evaluator: &Evaluator<Num>) -> Result<(), EvalError> {
        self(stack, evaluator)
    }
}

pub struct EvalFunction<Num: Number>(pub i32, pub Box<dyn EvalFn<Num>>);

impl<Num: Number> Evaluator<Num> {
    pub fn new() -> Self {
        Self {
            constants: HashMap::new(),
            variables: HashMap::new(),
            functions: basic_builtins(),
            angle_unit: AngleUnit::Degrees,
        }
    }
}

impl<Num: Number> Default for Evaluator<Num> {
    fn default() -> Self {
        Self::new()
    }
}

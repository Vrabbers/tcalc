use std::collections::HashMap;

use tcalc_num::{error::NumResult, number::Number};

use crate::evaluator::{EvalFn, EvalFunction, Evaluator, eval_result::EvalError};

fn pop<T>(stack: &mut Vec<T>) -> Result<T, EvalError> {
    match stack.pop() {
        Some(x) => Ok(x),
        None => Err(EvalError::InvalidProgram),
    }
}

struct SimpleBuiltin<F>(F);

impl<Num, F> EvalFn<Num> for SimpleBuiltin<F>
where
    Num: Number,
    F: Fn(&Num) -> NumResult<Num>,
{
    fn call(&self, stack: &mut Vec<Num>, _evaluator: &Evaluator<Num>) -> Result<(), EvalError> {
        let value = pop(stack)?;
        let result = self.0(&value)?;
        stack.push(result);

        Ok(())
    }
}

fn builtin1<Num: Number>(f: &'static impl Fn(&Num) -> NumResult<Num>) -> EvalFunction<Num> {
    let fun = Box::new(SimpleBuiltin(f));
    EvalFunction(1, fun)
}

fn builtin_pow<Num: Number>() -> EvalFunction<Num> {
    EvalFunction(
        2,
        Box::new(|stack: &mut Vec<Num>, _evaluator: &Evaluator<Num>| {
            let exponent = pop(stack)?;
            let base = pop(stack)?;
            let result = base.pow(exponent)?;
            stack.push(result);
            Ok(())
        }),
    )
}

pub fn basic_builtins<Num: Number>() -> HashMap<String, Vec<EvalFunction<Num>>> {
    HashMap::from(
        [
            ("sqrt", vec![builtin1(&Num::sqrt)]),
            ("exp", vec![builtin1(&Num::exp)]),
            ("log", vec![builtin1(&Num::log)]),
            ("ln", vec![builtin1(&Num::ln)]),
            ("abs", vec![builtin1(&Num::abs)]),
            ("fact", vec![builtin1(&Num::fact)]),
            ("deg", vec![builtin1(&Num::degrees_to_radians)]),
            ("rad", vec![builtin1(&Num::radians_to_degrees)]),
            ("sin", vec![builtin1(&Num::sin)]),
            ("cos", vec![builtin1(&Num::cos)]),
            ("tan", vec![builtin1(&Num::tan)]),
            ("asin", vec![builtin1(&Num::asin)]),
            ("acos", vec![builtin1(&Num::acos)]),
            ("atan", vec![builtin1(&Num::atan)]),
            ("sinh", vec![builtin1(&Num::sinh)]),
            ("cosh", vec![builtin1(&Num::cosh)]),
            ("tanh", vec![builtin1(&Num::tanh)]),
            ("pow", vec![builtin_pow()]),
        ]
        .map(|(n, fs)| (n.to_string(), fs)),
    )
}

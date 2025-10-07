use std::collections::HashMap;

use tcalc_num::{error::NumResult, number::Number};

use crate::evaluator::{
    EvalFn, EvalFunction, Evaluator, VecEvalErrorExtensions, angle_unit_to_radians,
    eval_result::EvalError, radians_to_angle_unit,
};

struct SimpleBuiltin<F>(F);

impl<Num, F> EvalFn<Num> for SimpleBuiltin<F>
where
    Num: Number,
    F: Fn(&Num) -> NumResult<Num>,
{
    fn call(&self, stack: &mut Vec<Num>, _evaluator: &Evaluator<Num>) -> Result<(), EvalError> {
        let value = stack.pop_result()?;
        let result = self.0(&value)?;
        stack.push(result);

        Ok(())
    }
}

struct TrigBuiltin<F>(F);

impl<Num, F> EvalFn<Num> for TrigBuiltin<F>
where
    Num: Number,
    F: Fn(&Num) -> NumResult<Num>,
{
    fn call(&self, stack: &mut Vec<Num>, evaluator: &Evaluator<Num>) -> Result<(), EvalError> {
        let value = angle_unit_to_radians(stack.pop_result()?, evaluator.angle_unit)?;
        let result = self.0(&value)?;
        stack.push(result);

        Ok(())
    }
}

struct InvTrigBuiltin<F>(F);

impl<Num, F> EvalFn<Num> for InvTrigBuiltin<F>
where
    Num: Number,
    F: Fn(&Num) -> NumResult<Num>,
{
    fn call(&self, stack: &mut Vec<Num>, evaluator: &Evaluator<Num>) -> Result<(), EvalError> {
        let value = stack.pop_result()?;
        let result = self.0(&value)?;
        stack.push(radians_to_angle_unit(result, evaluator.angle_unit)?);

        Ok(())
    }
}

fn builtin1<Num, F>(f: &'static F) -> EvalFunction<Num>
where
    Num: Number,
    F: Fn(&Num) -> NumResult<Num>,
{
    EvalFunction(1, Box::new(SimpleBuiltin(f)))
}

fn builtin_trig<Num, F>(f: &'static F) -> EvalFunction<Num>
where
    Num: Number,
    F: Fn(&Num) -> NumResult<Num>,
{
    EvalFunction(1, Box::new(TrigBuiltin(f)))
}

fn builtin_invtrig<Num, F>(f: &'static F) -> EvalFunction<Num>
where
    Num: Number,
    F: Fn(&Num) -> NumResult<Num>,
{
    EvalFunction(1, Box::new(InvTrigBuiltin(f)))
}

fn builtin_pow<Num: Number>() -> EvalFunction<Num> {
    EvalFunction(
        2,
        Box::new(|stack: &mut Vec<Num>, _evaluator: &Evaluator<Num>| {
            let exponent = stack.pop_result()?;
            let base = stack.pop_result()?;
            let result = base.pow(exponent)?;
            stack.push(result);
            Ok(())
        }),
    )
}

fn builtin_log2<Num: Number>() -> EvalFunction<Num> {
    EvalFunction(
        2,
        Box::new(|stack: &mut Vec<Num>, _evaluator: &Evaluator<Num>| {
            let second = stack.pop_result()?;
            let first = stack.pop_result()?;
            stack.push((first.log()? / second.log()?)?);
            Ok(())
        }),
    )
}

fn builtin_cbrt<Num: Number>() -> EvalFunction<Num> {
    EvalFunction(
        1,
        Box::new(|stack: &mut Vec<Num>, _evaluator: &Evaluator<Num>| {
            let val = stack.pop_result()?;
            stack.push(val.pow(Num::from(3).inv()?)?);
            Ok(())
        }),
    )
}

pub fn basic_builtins<Num: Number>() -> HashMap<String, Vec<EvalFunction<Num>>> {
    HashMap::from(
        [
            ("sqrt", vec![builtin1(&Num::sqrt)]),
            ("cbrt", vec![builtin_cbrt()]),
            ("exp", vec![builtin1(&Num::exp)]),
            ("log", vec![builtin1(&Num::log), builtin_log2()]),
            ("ln", vec![builtin1(&Num::ln)]),
            ("abs", vec![builtin1(&Num::abs)]),
            ("fact", vec![builtin1(&Num::fact)]),
            ("sin", vec![builtin_trig(&Num::sin)]),
            ("cos", vec![builtin_trig(&Num::cos)]),
            ("tan", vec![builtin_trig(&Num::tan)]),
            ("asin", vec![builtin_invtrig(&Num::asin)]),
            ("acos", vec![builtin_invtrig(&Num::acos)]),
            ("atan", vec![builtin_invtrig(&Num::atan)]),
            ("sinh", vec![builtin1(&Num::sinh)]),
            ("cosh", vec![builtin1(&Num::cosh)]),
            ("tanh", vec![builtin1(&Num::tanh)]),
            ("pow", vec![builtin_pow()]),
        ]
        .map(|(n, fs)| (n.to_string(), fs)),
    )
}

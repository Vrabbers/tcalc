use std::collections::HashMap;
use std::rc::Rc;
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

pub fn basic_builtins<Num: Number>() -> HashMap<String, Vec<Rc<EvalFunction<Num>>>> {
    HashMap::from(
        [
            ("sqrt", vec![Rc::new(builtin1(&Num::sqrt))]),
            ("cbrt", vec![Rc::new(builtin_cbrt())]),
            ("exp", vec![Rc::new(builtin1(&Num::exp))]),
            ("log", vec![Rc::new(builtin1(&Num::log)), Rc::new(builtin_log2())]),
            ("ln", vec![Rc::new(builtin1(&Num::ln))]),
            ("abs", vec![Rc::new(builtin1(&Num::abs))]),
            ("fact", vec![Rc::new(builtin1(&Num::fact))]),
            ("sin", vec![Rc::new(builtin_trig(&Num::sin))]),
            ("cos", vec![Rc::new(builtin_trig(&Num::cos))]),
            ("tan", vec![Rc::new(builtin_trig(&Num::tan))]),
            ("asin", vec![Rc::new(builtin_invtrig(&Num::asin))]),
            ("acos", vec![Rc::new(builtin_invtrig(&Num::acos))]),
            ("atan", vec![Rc::new(builtin_invtrig(&Num::atan))]),
            ("sinh", vec![Rc::new(builtin1(&Num::sinh))]),
            ("cosh", vec![Rc::new(builtin1(&Num::cosh))]),
            ("tanh", vec![Rc::new(builtin1(&Num::tanh))]),
            ("pow", vec![Rc::new(builtin_pow())]),
        ]
        .map(|(n, fs)| (n.to_string(), fs)),
    )
}

pub fn basic_builtin_consts<Num: Number>() -> HashMap<String, Num> {
    HashMap::from(
        [
            ("pi", Num::pi()),
            ("π", Num::pi()),
            ("tau", Num::tau()),
            ("τ", Num::tau()),
            ("e", Num::e()),
        ]
        .map(|(s, n)| (s.to_string(), n)),
    )
}

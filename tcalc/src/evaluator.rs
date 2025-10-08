use crate::evaluator::builtins::{basic_builtin_consts, basic_builtins};
use crate::evaluator::eval_result::{EvalError, EvalResult, EvalValue};
use crate::expressions::{Expression, OperationType, Statement};
use crate::token::TokenKind;
use std::collections::HashMap;

use cancellation_token::CancellationToken;
use tcalc_num::{angle_unit::AngleUnit, error::NumError, number::Number};

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
            constants: basic_builtin_consts(),
            variables: HashMap::new(),
            functions: basic_builtins(),
            angle_unit: AngleUnit::Degrees,
        }
    }

    pub fn apply_evaluation_effects(&mut self, val: EvalValue<Num>) {
        match val {
            EvalValue::Numeric(n) => { self.variables.insert("ans".to_string(), n); },
            EvalValue::AssignedVariable { variable_name, value } => { self.variables.insert(variable_name, value); },
            EvalValue::Comparison(_) => (),
        }
    }

    pub fn evaluate(&self, statement: &Statement, ct: CancellationToken) -> EvalResult<Num> {
        match statement {
            Statement::Arithmetic(expression) => Ok(EvalValue::Numeric(
                self.evaluate_arithmetic(expression, ct)?,
            )),
            Statement::Assignment {
                var,
                comp,
                position: _,
            } => self.evaluate_assignment(var, comp, ct),
            Statement::Boolean {
                lhs,
                rhs,
                kind,
                position: _,
            } => self.evaluate_boolean(lhs, rhs, *kind, ct),
        }
    }

    fn evaluate_assignment(&self, var: &String, comp: &Expression, ct: CancellationToken) -> EvalResult<Num> {
        if self.constants.contains_key(var) {
            Err(EvalError::AssignToConstant)
        } else {
            Ok(EvalValue::AssignedVariable { variable_name: var.clone(), value: self.evaluate_arithmetic(comp, ct)? })
        }
    }

    fn evaluate_boolean(&self, lhs: &Expression, rhs: &Expression, kind: TokenKind, ct: CancellationToken) -> EvalResult<Num> {
        let left = self.evaluate_arithmetic(lhs, ct.clone())?;
        let right = self.evaluate_arithmetic(rhs, ct)?;
        let fun = match kind {
            TokenKind::GreaterThan => Num::gt,
            TokenKind::GreaterOrEqual => Num::ge,
            TokenKind::LessThan => Num::lt,
            TokenKind::LessOrEqual => Num::le,
            TokenKind::Equality => Num::eq,
            TokenKind::Equal => Num::eq,
            TokenKind::NotEqual => Num::ne,
            _ => Err(EvalError::InvalidProgram)?,
        };
        Ok(EvalValue::Comparison(fun(&left, &right)?))
    }

    fn evaluate_arithmetic(
        &self,
        expression: &Expression,
        ct: CancellationToken,
    ) -> Result<Num, EvalError> {
        let mut stack = Vec::new();

        for operation in &expression.operations {
            match &operation.op_type {
                OperationType::Binary(kind) => self.evaluate_binary(kind, &mut stack)?,

                OperationType::Unary(kind) => self.evaluate_unary(kind, &mut stack)?,

                OperationType::Literal(n) => {
                    let n = Num::from_str(n)
                        .or(Err(EvalError::InvalidProgram))?
                        .with_cancellation(ct.clone());
                    stack.push(n);
                }

                OperationType::VarRef(name) => {
                    let num = if let Some(const_ref) = self.constants.get(name) {
                        const_ref
                    } else if let Some(var_ref) = self.variables.get(name) {
                        var_ref
                    } else {
                        return Err(EvalError::UndefinedVariable);
                    };

                    stack.push(num.clone().with_cancellation(ct.clone()))
                }

                OperationType::FnCall { name, arity } => {
                    let Some(funs) = self.functions.get(name) else {
                        return Err(EvalError::UndefinedFunction);
                    };
                    let Some(EvalFunction(_, fun)) =
                        funs.iter().find(|EvalFunction(a, _)| a == arity)
                    else {
                        return Err(EvalError::InvalidArgumentCount);
                    };
                    fun.call(&mut stack, self)?
                }
            }
        }

        stack.pop_result()
    }


    fn evaluate_binary(
        &self,
        token_kind: &TokenKind,
        stack: &mut Vec<Num>,
    ) -> Result<(), EvalError> {
        let rhs = stack.pop_result()?;
        let lhs = stack.pop_result()?;

        let result = match token_kind {
            TokenKind::Plus => lhs + rhs,
            TokenKind::Minus => lhs - rhs,
            TokenKind::Multiply => lhs * rhs,
            TokenKind::Divide => lhs / rhs,
            TokenKind::Exponentiate => lhs.pow(rhs),
            TokenKind::Radical => rhs.pow(lhs.inv()?), // TODO
            _ => return Err(EvalError::InvalidProgram),
        }?;
        stack.push(result);
        Ok(())
    }

    fn evaluate_unary(
        &self,
        token_kind: &TokenKind,
        stack: &mut Vec<Num>,
    ) -> Result<(), EvalError> {
        let val = stack.pop_result()?;
        let result = match token_kind {
            TokenKind::Minus => Ok(val.neg()),
            TokenKind::Radical => val.sqrt(),
            TokenKind::CubeRoot => val.pow(Num::from(3).inv()?), // TODO
            TokenKind::FourthRoot => val.pow(Num::from(4).inv()?), // TODO
            TokenKind::Percent => val.div(Num::from(100)),
            TokenKind::Factorial => val.fact(),
            TokenKind::Deg => angle_unit_to_radians(val, AngleUnit::Degrees),
            TokenKind::Rad => angle_unit_to_radians(val, AngleUnit::Radians),
            TokenKind::Grad => angle_unit_to_radians(val, AngleUnit::Gradians),
            _ => return Err(EvalError::InvalidProgram),
        }?;
        stack.push(result);
        Ok(())
    }
}

impl<Num: Number> Default for Evaluator<Num> {
    fn default() -> Self {
        Self::new()
    }
}
trait VecEvalErrorExtensions<T> {
    fn pop_result(&mut self) -> Result<T, EvalError>;
}

impl<T> VecEvalErrorExtensions<T> for Vec<T> {
    fn pop_result(&mut self) -> Result<T, EvalError> {
        match self.pop() {
            Some(x) => Ok(x),
            None => Err(EvalError::InvalidProgram),
        }
    }
}

fn angle_unit_to_radians<Num: Number>(num: Num, angle_unit: AngleUnit) -> Result<Num, NumError> {
    Ok(match angle_unit {
        AngleUnit::Degrees => num.degrees_to_radians()?,
        AngleUnit::Radians => num,
        AngleUnit::Gradians => num.gradians_to_radians()?,
    })
}

fn radians_to_angle_unit<Num: Number>(num: Num, angle_unit: AngleUnit) -> Result<Num, NumError> {
    Ok(match angle_unit {
        AngleUnit::Degrees => num.radians_to_degrees()?,
        AngleUnit::Radians => num,
        AngleUnit::Gradians => num.radians_to_gradians()?,
    })
}

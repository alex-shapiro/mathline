use crate::{
    MLResult,
    error::MathlineError,
    interpreter::value::Value,
    parser::expression::{
        Expression, FnExpression, InfixExpression, InfixOp, PrefixExpression, PrefixOp,
        ValueExpression,
    },
};

pub struct Evaluator {
    expression: Expression,
}

impl Evaluator {
    /// Returns a new evaluator for the expression
    pub fn new(expression: Expression) -> Self {
        Evaluator { expression }
    }

    /// Evaluates the expression
    pub fn eval(self) -> MLResult<Value> {
        Self::eval_expr(self.expression)
    }

    fn eval_expr(expr: Expression) -> MLResult<Value> {
        let expr_string = format!("{expr}");
        let mut print_log = true;
        let value: Value = match expr {
            Expression::Value(expr) => {
                print_log = false;
                Self::eval_value(expr)
            }
            Expression::Fn(expr) => Self::eval_function(expr),
            Expression::Infix(expr) => Self::eval_infix(expr),
            Expression::Prefix(expr) => Self::eval_prefix(expr),
        }?;
        if print_log {
            println!("Step: {expr_string} => {value}");
        }
        Ok(value)
    }

    fn eval_value(expr: ValueExpression) -> MLResult<Value> {
        Ok(match expr {
            ValueExpression::Bool(b) => Value::Bool(b),
            ValueExpression::I64(n) => Value::I64(n),
            ValueExpression::F64(n) => Value::F64(n),
            ValueExpression::Variable(v) => match v.as_str() {
                "math" => Value::Module("math"),
                "pi" | "PI" | "π" => Value::F64(std::f64::consts::PI),
                "tau" | "TAU" | "τ" => Value::F64(std::f64::consts::TAU),
                "e" => Value::F64(std::f64::consts::E),
                _ => return Err(MathlineError::UnknownVariable(v.to_string())),
            },
        })
    }

    fn eval_prefix(expr: PrefixExpression) -> MLResult<Value> {
        let rhs = Self::eval_expr(*expr.rhs)?;
        Ok(match (expr.op, rhs) {
            (PrefixOp::Not, Value::Bool(bool)) => Value::Bool(!bool),
            (PrefixOp::Plus, Value::I64(n)) => Value::I64(n),
            (PrefixOp::Plus, Value::F64(n)) => Value::F64(n),
            (PrefixOp::Minus, Value::I64(n)) => Value::I64(-n),
            (PrefixOp::Minus, Value::F64(n)) => Value::F64(-n),
            (op, rhs) => {
                let rhs = Box::new(Expression::from(rhs));
                let expr = Expression::Prefix(PrefixExpression { op, rhs });
                return Err(MathlineError::InvalidExpression(Box::new(expr)));
            }
        })
    }

    fn eval_infix(expr: InfixExpression) -> MLResult<Value> {
        let lhs = Self::eval_expr(*expr.lhs)?;
        let rhs = Self::eval_expr(*expr.rhs)?;
        let value = match expr.op {
            InfixOp::Add => {
                if let Value::I64(l) = lhs
                    && let Value::I64(r) = rhs
                {
                    Value::I64(l + r)
                } else {
                    let l = lhs.as_f64()?;
                    let r = rhs.as_f64()?;
                    Value::F64(l + r)
                }
            }
            InfixOp::Subtract => {
                if let Value::I64(l) = lhs
                    && let Value::I64(r) = rhs
                {
                    Value::I64(l - r)
                } else {
                    let l = lhs.as_f64()?;
                    let r = rhs.as_f64()?;
                    Value::F64(l - r)
                }
            }
            InfixOp::Multiply => {
                if let Value::I64(l) = lhs
                    && let Value::I64(r) = rhs
                {
                    Value::I64(l * r)
                } else {
                    let l = lhs.as_f64()?;
                    let r = rhs.as_f64()?;
                    Value::F64(l * r)
                }
            }
            InfixOp::Divide => {
                if let Value::I64(l) = lhs
                    && let Value::I64(r) = rhs
                    && l % r == 0
                {
                    Value::I64(l / r)
                } else {
                    let l = lhs.as_f64()?;
                    let r = rhs.as_f64()?;
                    Value::F64(l / r)
                }
            }
            InfixOp::Modulo => {
                if let Value::I64(l) = lhs
                    && let Value::I64(r) = rhs
                {
                    Value::I64(l % r)
                } else {
                    let l = lhs.as_f64()?;
                    let r = rhs.as_f64()?;
                    Value::F64(l % r)
                }
            }
            InfixOp::Exponent => {
                if let Value::I64(l) = lhs
                    && let Value::I64(r) = rhs
                    && r >= 0
                    && r < u32::MAX as i64
                {
                    if let Some(result) = l.checked_pow(r as u32) {
                        Value::I64(result)
                    } else {
                        let expr = Expression::Infix(InfixExpression {
                            op: expr.op,
                            lhs: Box::new(Expression::from(lhs)),
                            rhs: Box::new(Expression::from(rhs)),
                        });
                        return Err(MathlineError::InvalidExpression(Box::new(expr)));
                    }
                } else {
                    let l = lhs.as_f64()?;
                    let r = rhs.as_f64()?;
                    Value::F64(l.powf(r))
                }
            }
            InfixOp::And => {
                let l = lhs.as_bool()?;
                let r = rhs.as_bool()?;
                Value::Bool(l && r)
            }
            InfixOp::Or => {
                let l = lhs.as_bool()?;
                let r = rhs.as_bool()?;
                Value::Bool(l || r)
            }
            InfixOp::Equal => {
                let l = lhs.as_f64()?;
                let r = rhs.as_f64()?;
                Value::Bool(l == r)
            }
            InfixOp::NotEqual => {
                let l = lhs.as_f64()?;
                let r = rhs.as_f64()?;
                Value::Bool(l != r)
            }
            InfixOp::LessThan => {
                let l = lhs.as_f64()?;
                let r = rhs.as_f64()?;
                Value::Bool(l < r)
            }
            InfixOp::LessThanOrEqual => {
                let l = lhs.as_f64()?;
                let r = rhs.as_f64()?;
                Value::Bool(l <= r)
            }
            InfixOp::GreaterThan => {
                let l = lhs.as_f64()?;
                let r = rhs.as_f64()?;
                Value::Bool(l > r)
            }
            InfixOp::GreaterThanOrEqual => {
                let l = lhs.as_f64()?;
                let r = rhs.as_f64()?;
                Value::Bool(l >= r)
            }
            InfixOp::Dot => {
                lhs.as_module()?;
                rhs
            }
        };
        Ok(value)
    }

    fn eval_function(expr: FnExpression) -> MLResult<Value> {
        let parameters = expr
            .parameters
            .into_iter()
            .map(Self::eval_expr)
            .collect::<MLResult<Vec<_>>>()?;

        match expr.name.as_str() {
            "sin" => Self::eval_sin(parameters),
            "cos" => Self::eval_cos(parameters),
            "tan" => Self::eval_tan(parameters),
            "log" => Self::eval_log10(parameters),
            "ln" => Self::eval_ln(parameters),
            "print" => Self::eval_print(parameters),
            _ => Err(MathlineError::FunctionIsNotSupported(expr.name)),
        }
    }

    fn eval_sin(parameters: Vec<Value>) -> MLResult<Value> {
        if parameters.len() != 1 {
            return Err(MathlineError::InvalidFnParameterLength {
                name: "sin".to_string(),
                len: parameters.len(),
            });
        }
        let result = parameters[0].as_f64()?.sin();
        Ok(Value::F64(result))
    }

    fn eval_cos(parameters: Vec<Value>) -> MLResult<Value> {
        if parameters.len() != 1 {
            return Err(MathlineError::InvalidFnParameterLength {
                name: "cos".to_string(),
                len: parameters.len(),
            });
        }
        let result = parameters[0].as_f64()?.cos();
        Ok(Value::F64(result))
    }

    fn eval_tan(parameters: Vec<Value>) -> MLResult<Value> {
        if parameters.len() != 1 {
            return Err(MathlineError::InvalidFnParameterLength {
                name: "tan".to_string(),
                len: parameters.len(),
            });
        }
        let result = parameters[0].as_f64()?.tan();
        Ok(Value::F64(result))
    }

    fn eval_log10(parameters: Vec<Value>) -> MLResult<Value> {
        if parameters.len() != 1 {
            return Err(MathlineError::InvalidFnParameterLength {
                name: "log".to_string(),
                len: parameters.len(),
            });
        }
        let result = parameters[0].as_f64()?.log10();
        Ok(Value::F64(result))
    }

    fn eval_ln(parameters: Vec<Value>) -> MLResult<Value> {
        if parameters.len() != 1 {
            return Err(MathlineError::InvalidFnParameterLength {
                name: "ln".to_string(),
                len: parameters.len(),
            });
        }
        let result = parameters[0].as_f64()?.ln();
        Ok(Value::F64(result))
    }

    fn eval_print(parameters: Vec<Value>) -> MLResult<Value> {
        if parameters.len() != 1 {
            return Err(MathlineError::InvalidFnParameterLength {
                name: "print".to_string(),
                len: parameters.len(),
            });
        }
        let result = parameters[0];
        Ok(result)
    }
}

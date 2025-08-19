use std::fmt::Display;

use crate::{
    MLResult,
    error::MathlineError,
    parser::expression::{Expression, ValueExpression},
};

#[derive(Clone, Copy)]
pub enum Value {
    Bool(bool),
    I64(i64),
    F64(f64),
}

impl Value {
    pub fn as_bool(self) -> MLResult<bool> {
        match self {
            Value::Bool(b) => Ok(b),
            Value::I64(_) => Err(MathlineError::CannotUseNumberAsBool),
            Value::F64(_) => Err(MathlineError::CannotUseNumberAsBool),
        }
    }

    pub fn as_f64(self) -> MLResult<f64> {
        match self {
            Value::Bool(_) => Err(MathlineError::CannotUseBoolAsNumber),
            Value::I64(n) => Ok(n as f64),
            Value::F64(n) => Ok(n),
        }
    }
}

impl From<Value> for Expression {
    fn from(value: Value) -> Self {
        Expression::Value(match value {
            Value::Bool(b) => ValueExpression::Bool(b),
            Value::I64(n) => ValueExpression::I64(n),
            Value::F64(n) => ValueExpression::F64(n),
        })
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(v) => v.fmt(f),
            Value::I64(v) => v.fmt(f),
            Value::F64(v) => write!(f, "{v:.5}"),
        }
    }
}

use std::fmt::Display;

use crate::{error::MathlineError, parser::token::Op};

#[derive(Debug)]
pub enum Expression {
    Value(ValueExpression),
    Fn(FnExpression),
    Infix(InfixExpression),
    Prefix(PrefixExpression),
}

#[derive(Debug)]
pub enum ValueExpression {
    Number(Number),
    Variable(String),
}

#[derive(Debug, Default, PartialEq)]
pub struct Number {
    pub whole: u64,
    pub fraction: Option<u64>,
}

#[derive(Debug)]
pub struct FnExpression {
    pub name: String,
    pub parameters: Vec<Expression>,
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub op: PrefixOp,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub enum PrefixOp {
    Not,
    Plus,
    Minus,
}

impl TryFrom<Op> for PrefixOp {
    type Error = MathlineError;

    fn try_from(op: Op) -> Result<Self, Self::Error> {
        match op {
            Op::Not => Ok(Self::Not),
            Op::Plus => Ok(Self::Plus),
            Op::Minus => Ok(Self::Minus),
            _ => Err(MathlineError::InvalidSyntax),
        }
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    pub op: InfixOp,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub enum InfixOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent,
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Dot,
}

impl TryFrom<Op> for InfixOp {
    type Error = MathlineError;

    fn try_from(op: Op) -> Result<Self, Self::Error> {
        Ok(match op {
            Op::Plus => Self::Add,
            Op::Minus => Self::Subtract,
            Op::Multiply => Self::Multiply,
            Op::Divide => Self::Divide,
            Op::Modulo => Self::Modulo,
            Op::Exponent => Self::Exponent,
            Op::And => Self::And,
            Op::Or => Self::Or,
            Op::Equal => Self::Equal,
            Op::NotEqual => Self::NotEqual,
            Op::LessThan => Self::LessThan,
            Op::LessThanOrEqual => Self::LessThanOrEqual,
            Op::GreaterThan => Self::GreaterThan,
            Op::GreaterThanOrEqual => Self::GreaterThanOrEqual,
            Op::Dot => Self::Dot,
            Op::Not => return Err(MathlineError::InvalidSyntax),
        })
    }
}

impl Number {
    pub fn whole(n: u64) -> Self {
        Number {
            whole: n,
            fraction: None,
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Value(expr) => expr.fmt(f),
            Expression::Fn(expr) => expr.fmt(f),
            Expression::Infix(expr) => expr.fmt(f),
            Expression::Prefix(expr) => expr.fmt(f),
        }
    }
}

impl Display for ValueExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueExpression::Number(num) => num.fmt(f),
            ValueExpression::Variable(v) => v.fmt(f),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.whole)?;
        if let Some(fraction) = self.fraction {
            write!(f, ".{fraction}")?;
        }
        Ok(())
    }
}

impl Display for FnExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)?;
        write!(f, "(")?;
        if !self.parameters.is_empty() {
            write!(f, "{}", &self.parameters[0])?;
        }
        for p in &self.parameters[1..] {
            write!(f, ", {p}")?;
        }
        write!(f, ")")
    }
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.op, self.rhs)
    }
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.op, self.rhs)
    }
}

impl Display for PrefixOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrefixOp::Not => write!(f, "not "),
            PrefixOp::Plus => write!(f, "+"),
            PrefixOp::Minus => write!(f, "-"),
        }
    }
}

impl Display for InfixOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfixOp::Add => write!(f, "+"),
            InfixOp::Subtract => write!(f, "-"),
            InfixOp::Multiply => write!(f, "*"),
            InfixOp::Divide => write!(f, "/"),
            InfixOp::Modulo => write!(f, "%"),
            InfixOp::Exponent => write!(f, "**"),
            InfixOp::And => write!(f, "and"),
            InfixOp::Or => write!(f, "or"),
            InfixOp::Equal => write!(f, "=="),
            InfixOp::NotEqual => write!(f, "!="),
            InfixOp::LessThan => write!(f, "<"),
            InfixOp::LessThanOrEqual => write!(f, "<="),
            InfixOp::GreaterThan => write!(f, ">"),
            InfixOp::GreaterThanOrEqual => write!(f, ">="),
            InfixOp::Dot => write!(f, "."),
        }
    }
}

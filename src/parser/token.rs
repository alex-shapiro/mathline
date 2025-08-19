use std::fmt::Display;

use super::expression::Number;

#[derive(PartialEq)]
pub enum Token {
    Bool(bool),
    Number(Number),
    Symbol(String),
    Op(Op),
    LeftParen,
    RightParen,
    Comma,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Op {
    Not,
    Plus,
    Minus,
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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Bool(bool) => bool.fmt(f),
            Token::Number(num) => num.fmt(f),
            Token::Symbol(symbol) => symbol.fmt(f),
            Token::Op(op) => op.fmt(f),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::Comma => write!(f, ","),
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Not => write!(f, "not"),
            Op::Plus => write!(f, "+"),
            Op::Minus => write!(f, "-"),
            Op::Multiply => write!(f, "*"),
            Op::Divide => write!(f, "/"),
            Op::Modulo => write!(f, "%"),
            Op::Exponent => write!(f, "^"),
            Op::Dot => write!(f, "."),
            Op::And => write!(f, "and"),
            Op::Or => write!(f, "or"),
            Op::Equal => write!(f, "="),
            Op::NotEqual => write!(f, "≠"),
            Op::LessThan => write!(f, "<"),
            Op::LessThanOrEqual => write!(f, "≤"),
            Op::GreaterThan => write!(f, ">"),
            Op::GreaterThanOrEqual => write!(f, "≥"),
        }
    }
}

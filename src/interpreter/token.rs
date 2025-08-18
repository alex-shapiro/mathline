use std::fmt::Display;

pub enum Token {
    Number(Number),
    Symbol(String),
    Equal,
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
    LeftParen,
    RightParen,
}

#[derive(Default)]
pub struct Number {
    pub whole: u64,
    pub fraction: Option<u64>,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(n) => n.fmt(f),
            Token::Symbol(symbol) => symbol.fmt(f),
            Token::Equal => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Exponent => write!(f, "^"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.whole)?;
        if let Some(fraction) = &self.fraction {
            write!(f, ".{}", fraction)?;
        }
        Ok(())
    }
}

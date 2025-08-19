use std::{iter::Peekable, str::Chars};

use fallible_iterator::FallibleIterator;

use crate::{
    MLResult,
    error::MathlineError,
    parser::token::{Op, Token},
};

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    next: Option<Token>,
}

impl<'a> FallibleIterator for Lexer<'a> {
    type Item = Token;
    type Error = MathlineError;

    /// Return the next token in the input.
    /// Does not implement std Iterator trait because
    /// its outer type is a Result.
    fn next(&mut self) -> MLResult<Option<Token>> {
        if let Some(token) = self.next.take() {
            return Ok(Some(token));
        }
        while let Some(c) = self.input.next() {
            let token = match c {
                ' ' | '\t' | '\n' => continue,
                '+' => Token::Op(Op::Plus),
                '-' => Token::Op(Op::Minus),
                '*' => {
                    if let Some('*') = self.input.peek() {
                        self.input.next();
                        Token::Op(Op::Exponent)
                    } else {
                        Token::Op(Op::Multiply)
                    }
                }
                '/' => {
                    // ignore second slash bc it represents integer division
                    if let Some('/') = self.input.peek() {
                        self.input.next();
                    }
                    Token::Op(Op::Divide)
                }
                '%' => Token::Op(Op::Modulo),
                '^' => Token::Op(Op::Exponent),
                '.' => Token::Op(Op::Dot),
                '&' => {
                    let Some('&') = self.input.next() else {
                        return Err(MathlineError::InvalidSyntax);
                    };
                    Token::Op(Op::And)
                }
                '|' => {
                    let Some('|') = self.input.next() else {
                        return Err(MathlineError::InvalidSyntax);
                    };
                    Token::Op(Op::Or)
                }
                '=' => {
                    if let Some('=') = self.input.peek() {
                        self.input.next();
                    }
                    Token::Op(Op::Equal)
                }
                '!' => {
                    if let Some('=') = self.input.peek() {
                        self.input.next();
                        Token::Op(Op::NotEqual)
                    } else {
                        Token::Op(Op::Not)
                    }
                }
                '<' => {
                    if let Some('=') = self.input.peek() {
                        self.input.next();
                        Token::Op(Op::LessThanOrEqual)
                    } else {
                        Token::Op(Op::LessThan)
                    }
                }
                '>' => {
                    if let Some('=') = self.input.peek() {
                        self.input.next();
                        Token::Op(Op::GreaterThanOrEqual)
                    } else {
                        Token::Op(Op::GreaterThan)
                    }
                }
                '≠' => Token::Op(Op::NotEqual),        // alt for neq
                '≥' => Token::Op(Op::LessThanOrEqual), // alt for lte
                '≤' => Token::Op(Op::GreaterThanOrEqual), // alt for gte
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                ',' => Token::Comma,
                '⁰' => {
                    self.next = Some(Token::I64(0));
                    Token::Op(Op::Exponent)
                }
                '¹' => {
                    self.next = Some(Token::I64(1));
                    Token::Op(Op::Exponent)
                }
                '²' => {
                    self.next = Some(Token::I64(2));
                    Token::Op(Op::Exponent)
                }
                '³' => {
                    self.next = Some(Token::I64(3));
                    Token::Op(Op::Exponent)
                }
                '⁴' => {
                    self.next = Some(Token::I64(4));
                    Token::Op(Op::Exponent)
                }
                _ if c.is_ascii_digit() => self.lex_number(c)?,
                _ if c.is_ascii_alphabetic() => self.lex_symbol(c)?,
                _ => return Err(MathlineError::InvalidChar(c)),
            };
            return Ok(Some(token));
        }
        Ok(None)
    }
}

impl<'a> Lexer<'a> {
    /// Return a new lexer from the input
    pub fn new(input: &'a str) -> Self {
        let input = input.chars().peekable();
        Lexer { input, next: None }
    }

    fn lex_number(&mut self, c: char) -> MLResult<Token> {
        let mut whole = c.to_digit(10).unwrap() as i64;
        let mut fraction = 0.0f64;
        let mut fraction_multiplier = 0.0f64;

        while let Some(c) = self.input.peek() {
            if let Some(digit) = c.to_digit(10) {
                if fraction_multiplier == 0.0 {
                    whole *= 10;
                    whole += digit as i64;
                } else {
                    fraction += fraction_multiplier * (digit as f64);
                    fraction_multiplier *= 0.1;
                }
                self.input.next();
            } else if *c == '.' {
                fraction_multiplier = 0.1;
                self.input.next();
            } else {
                break;
            }
        }

        if fraction == 0.0 {
            Ok(Token::I64(whole))
        } else {
            Ok(Token::F64(whole as f64 + fraction))
        }
    }

    fn lex_symbol(&mut self, c: char) -> MLResult<Token> {
        let mut symbol = c.to_string();
        while let Some(c) = self.input.peek() {
            if c.is_ascii_alphanumeric() {
                symbol.push(*c);
                self.input.next();
            } else {
                break;
            }
        }
        match symbol.as_str() {
            "True" | "true" => return Ok(Token::Bool(true)),
            "False" | "false" => return Ok(Token::Bool(false)),
            "not" => return Ok(Token::Op(Op::Not)),
            "and" => return Ok(Token::Op(Op::And)),
            "or" => return Ok(Token::Op(Op::Or)),
            _ => Ok(Token::Symbol(symbol)),
        }
    }
}

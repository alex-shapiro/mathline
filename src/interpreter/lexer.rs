use std::{iter::Peekable, str::Chars};

use crate::{
    MLResult,
    error::MathlineError,
    interpreter::token::{Number, Token},
};

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let input = input.chars().peekable();
        Lexer { input }
    }

    pub fn next(&mut self) -> MLResult<Option<Token>> {
        while let Some(c) = self.input.next() {
            let token = match c {
                ' ' | '\t' | '\n' => continue,
                '=' => Token::Equal,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Multiply,
                '/' => Token::Divide,
                '^' => Token::Exponent,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                _ if c.is_ascii_digit() => self.lex_number(c)?,
                _ if c.is_ascii_alphabetic() => self.lex_symbol(c)?,
                _ => return Err(MathlineError::InvalidChar(c)),
            };
            return Ok(Some(token));
        }
        Ok(None)
    }

    fn lex_number(&mut self, c: char) -> MLResult<Token> {
        let mut number = Number::default();
        number.whole = c.to_digit(10).unwrap() as u64;

        while let Some(c) = self.input.peek() {
            if let Some(digit) = c.to_digit(10) {
                if let Some(fraction) = number.fraction {
                    number.fraction = Some(fraction * 10 + (digit as u64));
                } else {
                    number.whole = number.whole * 10 + (digit as u64);
                }
                self.input.next();
            } else if *c == '.' {
                number.fraction = Some(0);
                self.input.next();
            } else {
                break;
            }
        }

        Ok(Token::Number(number))
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
        Ok(Token::Symbol(symbol))
    }
}

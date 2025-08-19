pub mod expression;
pub mod lexer;
pub mod token;

use fallible_iterator::{FallibleIterator, Peekable};

use self::{
    expression::{
        Expression, FnExpression, InfixExpression, InfixOp, PrefixExpression, PrefixOp,
        ValueExpression,
    },
    lexer::Lexer,
    token::Token,
};
use crate::{MLResult, error::MathlineError};

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    /// Return a new parser from the input
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input).peekable();
        Parser { lexer }
    }
}

impl<'a> FallibleIterator for Parser<'a> {
    type Item = Expression;
    type Error = MathlineError;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        if self.lexer.peek()?.is_none() {
            return Ok(None);
        };
        let expression = self.parse_expr(0)?;
        Ok(Some(expression))
    }
}

impl<'a> Parser<'a> {
    fn parse_expr(&mut self, min_precedence: u8) -> MLResult<Expression> {
        let Some(token) = self.lexer.next()? else {
            return Err(MathlineError::InvalidEOF);
        };
        let mut lhs = match token {
            Token::Symbol(symbol) => {
                if let Some(Token::LeftParen) = self.lexer.peek()? {
                    self.lexer.next()?;
                    let fn_expression = self.parse_fn(symbol)?;
                    Expression::Fn(fn_expression)
                } else {
                    Expression::Value(ValueExpression::Variable(symbol))
                }
            }
            Token::Bool(b) => Expression::Value(ValueExpression::Bool(b)),
            Token::I64(n) => Expression::Value(ValueExpression::I64(n)),
            Token::F64(n) => Expression::Value(ValueExpression::F64(n)),
            Token::LeftParen => {
                let lhs = self.parse_expr(0)?;
                self.expect(Token::RightParen)?;
                lhs
            }
            Token::Op(op) => {
                let op = PrefixOp::try_from(op)?;
                let precedence = op.precedence();
                let rhs = Box::new(self.parse_expr(precedence)?);
                Expression::Prefix(PrefixExpression { op, rhs })
            }
            other => {
                tracing::error!("unhandled token: {other}");
                return Err(MathlineError::InvalidSyntax);
            }
        };
        while let Some(Token::Op(op)) = self.lexer.peek()? {
            let op = InfixOp::try_from(*op)?;
            let (lhs_precedence, rhs_precedence) = op.precedence();
            if lhs_precedence < min_precedence {
                break;
            }
            self.lexer.next()?;
            let rhs = self.parse_expr(rhs_precedence)?;
            lhs = Expression::Infix(InfixExpression {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
        }
        Ok(lhs)
    }

    fn parse_fn(&mut self, name: String) -> MLResult<FnExpression> {
        let mut parameters = vec![];

        if let Some(Token::RightParen) = self.lexer.peek()? {
            self.lexer.next()?;
            return Ok(FnExpression { name, parameters });
        }

        loop {
            let parameter = self.parse_expr(0)?;
            parameters.push(parameter);
            let token = self
                .lexer
                .next()?
                .ok_or_else(|| MathlineError::InvalidEOF)?;
            match token {
                Token::RightParen => break,
                Token::Comma => continue,
                _ => return Err(MathlineError::InvalidSyntax),
            }
        }

        Ok(FnExpression { name, parameters })
    }

    fn expect(&mut self, token: Token) -> MLResult<()> {
        let t = self.lexer.next()?;
        if t == Some(token) {
            Ok(())
        } else {
            Err(MathlineError::InvalidSyntax)
        }
    }
}

impl PrefixOp {
    fn precedence(&self) -> u8 {
        // follow python operator precedence
        // https://docs.python.org/3/reference/expressions.html#operator-precedence
        match self {
            PrefixOp::Not => 5,
            PrefixOp::Plus | PrefixOp::Minus => 15,
        }
    }
}

impl InfixOp {
    fn precedence(&self) -> (u8, u8) {
        // follow python operator precedence
        // https://docs.python.org/3/reference/expressions.html#operator-precedence
        match self {
            InfixOp::Or => (1, 2),
            InfixOp::And => (3, 4),
            InfixOp::Equal
            | InfixOp::NotEqual
            | InfixOp::LessThan
            | InfixOp::LessThanOrEqual
            | InfixOp::GreaterThan
            | InfixOp::GreaterThanOrEqual => (7, 8),
            InfixOp::Add | InfixOp::Subtract => (11, 12),
            InfixOp::Multiply | InfixOp::Divide | InfixOp::Modulo => (13, 14),
            InfixOp::Exponent => (16, 9), // binds tightly to the left, loosely to the right
            InfixOp::Dot => (18, 17),     // right associative
        }
    }
}

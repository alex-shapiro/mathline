use crate::parser::expression::Expression;

#[derive(Debug, thiserror::Error)]
pub enum MathlineError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error("invalid char: {0}")]
    InvalidChar(char),
    #[error("invalid syntax")]
    InvalidSyntax,
    #[error("invalid EOF")]
    InvalidEOF,
    #[error("invalid expression {0}")]
    InvalidExpression(Box<Expression>),
    #[error("unknown variable {0}")]
    UnknownVariable(String),
    #[error("function {name} cannot have {len} parameters")]
    InvalidFnParameterLength { name: String, len: usize },
    #[error("cannot use a non-numeric value as a number")]
    CannotUseAsNumber,
    #[error("cannot use a non-boolean value as a bool")]
    CannotUseAsBool,
    #[error("cannot use a non-module value as a module")]
    CannotUseAsModule,
    #[error("function {0} is not supported")]
    FunctionIsNotSupported(String),
}

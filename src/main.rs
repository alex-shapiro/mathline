use fallible_iterator::FallibleIterator;

use crate::{agent::call_agent, error::MathlineError, parser::Parser};

mod agent;
mod error;
mod interpreter;
mod parser;

pub type MLResult<T> = std::result::Result<T, MathlineError>;

#[tokio::main]
async fn main() -> MLResult<()> {
    let mut args = std::env::args();
    args.next(); // skip program name
    let Some(request) = args.next() else {
        eprintln!(r#"request an expression, e.g. "what is the cosine of three times eleven?""#);
        return Ok(());
    };

    println!("{request}");
    let expression_string = match call_agent(&request).await {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{error}");
            return Ok(());
        }
    };

    println!("| {expression_string}");

    let expressions: Vec<_> = Parser::new(&expression_string).collect()?;
    for expr in expressions {
        println!("{expr}");
    }
    Ok(())
}

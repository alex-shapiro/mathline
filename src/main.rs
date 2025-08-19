use fallible_iterator::FallibleIterator;

use crate::{
    agent::call_agent, error::MathlineError, interpreter::evaluator::Evaluator, parser::Parser,
};

mod agent;
mod error;
mod interpreter;
mod parser;

pub type MLResult<T> = std::result::Result<T, MathlineError>;

#[tokio::main]
async fn main() {
    if let Err(error) = main_inner().await {
        eprintln!("ERROR: {error}");
    }
}

async fn main_inner() -> MLResult<()> {
    let mut args = std::env::args();
    args.next(); // skip program name
    let Some(request) = args.next() else {
        eprintln!(r#"request an expression, e.g. "what is the cosine of three times eleven?""#);
        return Ok(());
    };

    let expression_string = match call_agent(&request).await {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{error}");
            return Ok(());
        }
    };

    println!("LLM: {expression_string}");

    let expressions: Vec<_> = Parser::new(&expression_string).collect()?;

    for expr in expressions {
        println!("Parse: {expr}");
        let evaluator = Evaluator::new(expr);
        let value = evaluator.eval()?;
        println!("Answer: {value}");
    }
    Ok(())
}

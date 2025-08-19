use argh::FromArgs;
use fallible_iterator::FallibleIterator;

use crate::{
    agent::call_agent, error::MathlineError, interpreter::evaluator::Evaluator, parser::Parser,
};

mod agent;
mod error;
mod interpreter;
mod parser;

pub type MLResult<T> = std::result::Result<T, MathlineError>;

#[derive(FromArgs)]
/// Natural language solver for mathematical expressions
struct Args {
    /// ollama model ID (default is "gemma3:4b")
    #[argh(option)]
    model: Option<String>,
    /// natural language request
    #[argh(positional)]
    request: String,
}

#[tokio::main]
async fn main() {
    if let Err(error) = main_inner().await {
        eprintln!("ERROR: {error}");
    }
}

async fn main_inner() -> MLResult<()> {
    let args: Args = argh::from_env();
    let model = args
        .model.as_deref()
        .unwrap_or("gemma3:4b");

    let expression_string = match call_agent(&args.request, model).await {
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

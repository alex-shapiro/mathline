use crate::{agent::call_agent, error::MathlineError, interpreter::lexer::Lexer};

mod agent;
mod error;
mod interpreter;

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

    let mut lexer = Lexer::new(&expression_string);

    while let Some(token) = lexer.next()? {
        println!("{token}");
    }

    Ok(())
}

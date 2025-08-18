use crate::{agent::call_agent, error::MathlineError};

mod agent;
mod error;
mod interpreter;

pub type MLResult<T> = std::result::Result<T, MathlineError>;

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    args.next(); // skip program name
    let Some(request) = args.next() else {
        eprintln!(r#"request an expression, e.g. "what is the cosine of three times eleven?""#);
        return;
    };
    println!("{request}");
    let response = match call_agent(&request).await {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };
    println!("| {response}");
}

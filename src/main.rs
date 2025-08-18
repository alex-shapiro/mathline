mod agent;
mod error;
mod interpreter;

fn main() {
    let mut args = std::env::args();
    args.next(); // skip program name
    let Some(request) = args.next() else {
        eprintln!(r#"request an expression, e.g. "what is the cosine of three times eleven?""#);
        return;
    };
    println!("{request}");
}

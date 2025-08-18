pub enum Expression {
    Value(ValueExpression),
    Fn(FnExpression),
    Infix(InfixOp),
    Prefix(PrefixOp),
}

enum ValueExpression {
    I64(i64),
    F64(f64),
    Constant(String),
}

struct FnExpression {
    name: String,
    parameters: Vec<ValueExpression>,
}

struct PrefixExpression {}

enum PrefixOp {}

struct InfixExpression {}

enum InfixOp {
    Add,
    Sub,
    Multiply,
    Divide,
    Exponent,
}

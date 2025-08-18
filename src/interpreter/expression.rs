pub enum Expression {
    Value(ValueExpression),
    Fn(FnExpression),
    Infix(InfixOp),
    Prefix(PrefixOp),
}

pub enum ValueExpression {
    I64(i64),
    F64(f64),
    Constant(String),
}

pub struct FnExpression {
    pub name: String,
    pub parameters: Vec<ValueExpression>,
}

pub struct PrefixExpression {
    pub op: PrefixOp,
    pub rhs: Box<Expression>,
}

pub enum PrefixOp {
    Plus,
    Minus,
}

pub struct InfixExpression {
    pub op: InfixOp,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

pub enum InfixOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
}

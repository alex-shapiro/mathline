use super::expression::Expression;

pub struct Evaluator {
    expression: Expression,
}

impl Evaluator {
    /// Returns a new evaluator for the expression
    pub fn new(expression: Expression) -> Self {
        Evaluator { expression }
    }

    /// Runs one
    pub fn run(self) -> Expression {
        self.expression
    }
}

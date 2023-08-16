use crate::expression::{Expression, Identifier};

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Variable {
        name: Identifier,
        initializer: Option<Expression>,
    },
    Block {
        statements: Vec<Statement>,
    },
}

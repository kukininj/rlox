use crate::expression::{Expression, Identifier};

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Variable {
        name: Identifier,
        initializer: Option<Expression>,
    },
    Block(Block),
    If {
        condition: Expression,
        then_branch: Block,
        else_branch: Option<Block>,
    },
}

use crate::expression::{Expression, Identifier};

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Nop,
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
    While {
        condition: Expression,
        body: Block,
    },
    Function {
        name: Identifier,
        args: Vec<Identifier>,
        body: Block,
    },
    Return {
        value: Option<Expression>,
    },
}

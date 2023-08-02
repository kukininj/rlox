use crate::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Variable {
        name: String,
        initializer: Option<Expression>,
    },
}

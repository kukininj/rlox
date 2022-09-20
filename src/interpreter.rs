use crate::expression::Binary;
use crate::expression::BinaryOperator;
use crate::expression::Expression;
use crate::expression::Grouping;
use crate::expression::LiteralValue;
use crate::expression::Unary;
use crate::expression::UnaryOperator;
use crate::lox_object::LoxObject;
use crate::lox_value::LoxValue;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }
    pub fn evaluate(self: &mut Self, expr: Expression) -> LoxValue {
        match expr {
            Expression::Binary(binary) => self.visit_binary(binary),
            Expression::Grouping(grouping) => self.visit_grouping(grouping),
            Expression::Literal(literal) => self.visit_literal(literal.value),
            Expression::Unary(unary) => self.visit_unary(unary),
        }
    }

    fn visit_binary(self: &mut Self, binary: Box<Binary>) -> LoxValue {
        let left = self.evaluate(binary.left);
        let right = self.evaluate(binary.right);

        match binary.operator {
            BinaryOperator::Add(_) => LoxValue::add(left, right),
            BinaryOperator::Subtract(_) => LoxValue::subtract(left, right),
            BinaryOperator::Multiply(_) => LoxValue::multiply(left, right),
            BinaryOperator::Divide(_) => LoxValue::divide(left, right),
            BinaryOperator::Equal(_) => todo!(),
            BinaryOperator::NotEqual(_) => todo!(),
            BinaryOperator::Less(_) => todo!(),
            BinaryOperator::LessEqual(_) => todo!(),
            BinaryOperator::Greater(_) => todo!(),
            BinaryOperator::GreaterEqual(_) => todo!(),
        }
    }

    fn visit_grouping(self: &mut Self, grouping: Box<Grouping>) -> LoxValue {
        self.evaluate(grouping.expression)
    }

    fn visit_literal(self: &mut Self, literal: LiteralValue) -> LoxValue {
        match literal {
            LiteralValue::String(s, _) => LoxValue::String(s),
            LiteralValue::Number(n, _) => LoxValue::Number(n),
            LiteralValue::True(_) => LoxValue::Bool(true),
            LiteralValue::False(_) => LoxValue::Bool(false),
        }
    }

    fn visit_unary(self: &mut Self, unary: Box<Unary>) -> LoxValue {
        let right = self.evaluate(unary.right);
        match unary.operator {
            UnaryOperator::Negative(_) => LoxValue::negative(right),
            UnaryOperator::Not(_) => todo!(),
        }
    }
}

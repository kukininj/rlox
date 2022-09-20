use crate::error::Error;
use crate::expression::Binary;
use crate::expression::BinaryOperator;
use crate::expression::DebugInfo;
use crate::expression::Expression;
use crate::expression::Grouping;
use crate::expression::LiteralValue;
use crate::expression::Unary;
use crate::expression::UnaryOperator;
use crate::lox_value::LoxValue;

pub struct Interpreter {
    line: usize,
    position: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            line: 0,
            position: 0,
        }
    }

    fn set_debug(self: &mut Self, debug: DebugInfo) {
        self.line = debug.line;
        self.position = debug.position;
    }
    pub fn evaluate(self: &mut Self, expr: Expression) -> Result<LoxValue, Error> {
        let result = match expr {
            Expression::Binary(binary) => self.visit_binary(binary),
            Expression::Grouping(grouping) => self.visit_grouping(grouping),
            Expression::Literal(literal) => Ok(self.visit_literal(literal.value)),
            Expression::Unary(unary) => self.visit_unary(unary),
        };
        match result {
            Ok(value) => Ok(value),
            Err(Error::InternalRuntimeError { message }) => Err(Error::RuntimeError {
                line: self.line,
                position: self.position,
                message,
            }),
            Err(error) => Err(error),
        }
    }

    fn visit_binary(self: &mut Self, binary: Box<Binary>) -> Result<LoxValue, Error> {
        let left = self.evaluate(binary.left)?;
        let right = self.evaluate(binary.right)?;

        match binary.operator {
            BinaryOperator::Add(debug) => {
                self.set_debug(debug);
                LoxValue::add(left, right)
            }
            BinaryOperator::Subtract(debug) => {
                self.set_debug(debug);
                LoxValue::subtract(left, right)
            },
            BinaryOperator::Multiply(debug) => {
                self.set_debug(debug);
                LoxValue::multiply(left, right)
            },
            BinaryOperator::Divide(debug) => {
                self.set_debug(debug);
                LoxValue::divide(left, right)
            },
            BinaryOperator::Equal(_) => todo!(),
            BinaryOperator::NotEqual(_) => todo!(),
            BinaryOperator::Less(_) => todo!(),
            BinaryOperator::LessEqual(_) => todo!(),
            BinaryOperator::Greater(_) => todo!(),
            BinaryOperator::GreaterEqual(_) => todo!(),
        }
    }

    fn visit_grouping(self: &mut Self, grouping: Box<Grouping>) -> Result<LoxValue, Error> {
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

    fn visit_unary(self: &mut Self, unary: Box<Unary>) -> Result<LoxValue, Error> {
        let right = self.evaluate(unary.right)?;
        match unary.operator {
            UnaryOperator::Negative(debug) => {
                self.set_debug(debug);
                LoxValue::negative(right)
            }
            UnaryOperator::Not(_) => todo!(),
        }
    }
}

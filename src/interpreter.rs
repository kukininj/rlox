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
            }
            BinaryOperator::Multiply(debug) => {
                self.set_debug(debug);
                LoxValue::multiply(left, right)
            }
            BinaryOperator::Divide(debug) => {
                self.set_debug(debug);
                LoxValue::divide(left, right)
            }
            BinaryOperator::Equal(debug) => {
                self.set_debug(debug);
                LoxValue::equal(left, right)
            }
            BinaryOperator::NotEqual(debug) => {
                self.set_debug(debug);
                LoxValue::not_equal(left, right)
            }
            BinaryOperator::Less(debug) => {
                self.set_debug(debug);
                LoxValue::less(left, right)
            }
            BinaryOperator::LessEqual(debug) => {
                self.set_debug(debug);
                LoxValue::less_equal(left, right)
            }
            BinaryOperator::Greater(debug) => {
                self.set_debug(debug);
                LoxValue::greater(left, right)
            }
            BinaryOperator::GreaterEqual(debug) => {
                self.set_debug(debug);
                LoxValue::greater_equal(left, right)
            }
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
            UnaryOperator::Not(debug) => {
                self.set_debug(debug);
                let b = LoxValue::is_truthy(right);
                Ok(LoxValue::Bool(!b))
            }
        }
    }
}

#[test]
fn runtime_error_string_negation() {
    use crate::parser;
    use crate::scanner;
    let source = "-\"asdf\"".to_string();
    let tokens = scanner::scan_tokens(&source).unwrap();
    let tree = parser::parse(tokens).unwrap();
    let mut interp = Interpreter::new();
    if let Error::RuntimeError {
        line,
        position,
        message,
    } = interp.evaluate(tree).unwrap_err()
    {
        assert_eq!(line, 1);
        assert_eq!(position, 1);
        assert_eq!(message, "Cannot negate: String(\"asdf\")");
    };
}

#[test]
fn basic_arithmetics() {
    use crate::parser;
    use crate::scanner;
    let source = "2 + 2 * 2 / (3-2) * 1".to_string();
    let tokens = scanner::scan_tokens(&source).unwrap();
    let tree = parser::parse(tokens).unwrap();
    let mut interp = Interpreter::new();
    if let LoxValue::Number(n) = interp.evaluate(tree).unwrap() {
        assert_eq!(n, 6.);
    };
}

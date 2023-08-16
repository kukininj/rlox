use crate::environment::Environment;
use crate::error::Error;
use crate::expression::Binary;
use crate::expression::BinaryOperator;
use crate::expression::DebugInfo;
use crate::expression::Expression;
use crate::expression::Grouping;
use crate::expression::Identifier;
use crate::expression::LiteralValue;
use crate::expression::Unary;
use crate::expression::UnaryOperator;
use crate::lox_value::LoxValue;
use crate::statement::Statement;

pub struct Interpreter {
    line: usize,
    position: usize,
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            line: 0,
            position: 0,
            environment: Environment::new(),
        }
    }

    fn set_debug(self: &mut Self, debug: &DebugInfo) {
        self.line = debug.line;
        self.position = debug.position;
    }

    pub fn run(self: &mut Self, statements: &Vec<Statement>) -> Result<(), Error> {
        for stmt in statements {
            match stmt {
                Statement::Expression(expr) => {
                    self.evaluate(expr)?;
                }
                Statement::Print(expr) => {
                    let value = self.evaluate(expr)?;
                    println!("{}", value);
                }
                Statement::Variable {
                    name,
                    initializer: Some(initializer),
                } => {
                    let value = self.evaluate(initializer)?;
                    self.environment.define(name, value.clone())?;
                }
                Statement::Variable {
                    name,
                    initializer: None,
                } => {
                    self.environment.define(name, LoxValue::Nil)?;
                }
                Statement::Block { statements } => {
                    self.environment.push();
                    self.run(statements)?;
                    self.environment.pop();
                }
            };
        }
        Ok(())
    }

    pub fn evaluate(self: &mut Self, expr: &Expression) -> Result<LoxValue, Error> {
        let result = match expr {
            Expression::Binary(binary) => self.visit_binary(binary),
            Expression::Grouping(grouping) => self.visit_grouping(grouping),
            Expression::Literal(literal) => Ok(self.visit_literal(&literal.value)),
            Expression::Unary(unary) => self.visit_unary(unary),
            Expression::Identifier(identifier) => self.visit_identifier(identifier),
            Expression::Assignment(assignment) => {
                self.visit_assignment(&assignment.target, &assignment.value)
            }
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

    fn visit_binary(self: &mut Self, binary: &Binary) -> Result<LoxValue, Error> {
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;

        match binary {
            Binary {
                operator: BinaryOperator::Add(debug),
                ..
            } => {
                self.set_debug(&debug);
                LoxValue::add(left, right)
            }
            Binary {
                operator: BinaryOperator::Subtract(debug),
                ..
            } => {
                self.set_debug(&debug);
                LoxValue::subtract(left, right)
            }
            Binary {
                operator: BinaryOperator::Multiply(debug),
                ..
            } => {
                self.set_debug(&debug);
                LoxValue::multiply(left, right)
            }
            Binary {
                operator: BinaryOperator::Divide(debug),
                ..
            } => {
                self.set_debug(&debug);
                LoxValue::divide(left, right)
            }
            Binary {
                operator: BinaryOperator::Equal(debug),
                ..
            } => {
                self.set_debug(&debug);
                LoxValue::equal(left, right)
            }
            Binary {
                operator: BinaryOperator::NotEqual(debug),
                ..
            } => {
                self.set_debug(&debug);
                LoxValue::not_equal(left, right)
            }
            Binary {
                operator: BinaryOperator::Less(debug),
                ..
            } => {
                self.set_debug(&debug);
                LoxValue::less(left, right)
            }
            Binary {
                operator: BinaryOperator::LessEqual(debug),
                ..
            } => {
                self.set_debug(&debug);
                LoxValue::less_equal(left, right)
            }
            Binary {
                operator: BinaryOperator::Greater(debug),
                ..
            } => {
                self.set_debug(&debug);
                LoxValue::greater(left, right)
            }
            Binary {
                operator: BinaryOperator::GreaterEqual(debug),
                ..
            } => {
                self.set_debug(&debug);
                LoxValue::greater_equal(left, right)
            }
        }
    }

    fn visit_grouping(self: &mut Self, grouping: &Grouping) -> Result<LoxValue, Error> {
        self.evaluate(&grouping.expression)
    }

    fn visit_literal(self: &mut Self, literal: &LiteralValue) -> LoxValue {
        match literal {
            LiteralValue::String(s, _) => LoxValue::String(s.clone()),
            LiteralValue::Number(n, _) => LoxValue::Number(n.clone()),
            LiteralValue::True(_) => LoxValue::Bool(true),
            LiteralValue::False(_) => LoxValue::Bool(false),
            LiteralValue::Nil(_) => LoxValue::Nil,
        }
    }

    fn visit_unary(self: &mut Self, unary: &Unary) -> Result<LoxValue, Error> {
        let right = self.evaluate(&unary.right)?;
        match unary {
            Unary {
                operator: UnaryOperator::Negative(debug),
                ..
            } => {
                self.set_debug(&debug);
                LoxValue::negative(right)
            }
            Unary {
                operator: UnaryOperator::Not(debug),
                ..
            } => {
                self.set_debug(&debug);
                let b = LoxValue::is_truthy(right);
                Ok(LoxValue::Bool(!b))
            }
        }
    }

    fn visit_identifier(self: &mut Self, identifier: &Identifier) -> Result<LoxValue, Error> {
        self.environment.get(identifier)
    }

    fn visit_assignment(
        self: &mut Self,
        target: &Identifier,
        value: &Expression,
    ) -> Result<LoxValue, Error> {
        let value = self.evaluate(&value)?;
        self.environment.assign(target, value)
    }
}

#[test]
fn runtime_error_string_negation() {
    use crate::parser;
    use crate::scanner;
    let source = "-\"asdf\";".to_string();
    let tokens = scanner::scan_tokens(&source).unwrap();
    let tree = parser::parse(tokens).unwrap();
    let mut interp = Interpreter::new();
    if let Error::RuntimeError {
        line,
        position,
        message,
    } = interp.run(&tree).unwrap_err()
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
    let source = "print 2 + 2 * 2 / (3-2) * 1;".to_string();
    let tokens = scanner::scan_tokens(&source).unwrap();
    let tree = parser::parse(tokens).unwrap();
    let mut interp = Interpreter::new();
    interp.run(&tree).unwrap();
}

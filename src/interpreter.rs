use crate::environment::Environment;
use crate::error::Error;
use crate::expression::Binary;
use crate::expression::BinaryOperator;
use crate::expression::Call;
use crate::expression::DebugInfo;
use crate::expression::Expression;
use crate::expression::Grouping;
use crate::expression::Identifier;
use crate::expression::LiteralValue;
use crate::expression::Logical;
use crate::expression::LogicalOperator;
use crate::expression::Unary;
use crate::expression::UnaryOperator;
use crate::lox_function::LoxFun;
use crate::lox_value::LoxValue;
use crate::resolver::AccessTable;
use crate::statement::Block;
use crate::statement::Statement;

pub struct Interpreter {
    pub line: usize,
    pub position: usize,
    pub environment: Environment,
}

#[derive(Debug)]
pub enum LoxResult {
    Return(LoxValue),
    None,
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

    pub fn execute(
        &mut self,
        statements: &Vec<Statement>,
        access_table: AccessTable,
    ) -> Result<LoxResult, Error> {
        self.environment
            .extend_access_table(access_table)
            .map_err(|_| self.error("Error while updating access_table"))?;

        self.run(statements)
    }

    fn run(self: &mut Self, statements: &Vec<Statement>) -> Result<LoxResult, Error> {
        for stmt in statements {
            match stmt {
                Statement::Nop => {}
                Statement::Expression(expr) => {
                    self.evaluate(expr)?;
                }
                Statement::Print(expr) => {
                    let value = self.evaluate(expr)?;
                    println!("{}", LoxValue::to_string(&value));
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
                Statement::Block(block) => {
                    self.run_block(&block)?;
                }
                Statement::If {
                    condition,
                    then_branch,
                    else_branch,
                } => {
                    if LoxValue::is_truthy(&self.evaluate(condition)?) {
                        self.run_block(&then_branch)?;
                    } else if let Some(else_branch) = else_branch {
                        self.run_block(&else_branch)?;
                    }
                }
                Statement::While { condition, body } => {
                    while LoxValue::is_truthy(&self.evaluate(condition)?) {
                        self.run_block(body)?;
                    }
                }
                Statement::Function { name, args, body } => {
                    self.define_function(name, args, body)?;
                }
                Statement::Return { value: Some(value) } => {
                    let value = self.evaluate(value)?;

                    return Ok(LoxResult::Return(value));
                }
                Statement::Return { value: None } => {
                    return Ok(LoxResult::Return(LoxValue::Nil));
                }
            };
        }
        Ok(LoxResult::None)
    }

    pub fn run_block(&mut self, block: &Block) -> Result<LoxResult, Error> {
        self.environment.push();
        let result = self.run(&block.statements);
        self.environment.pop();
        result
    }

    pub fn define_function(
        &mut self,
        name: &Identifier,
        args: &Vec<Identifier>,
        body: &Block,
    ) -> Result<(), Error> {
        let frame_id = self.environment.get_current_frame_id();
        let lox_function = LoxFun::new(
            name.clone(),
            frame_id,
            args.clone().into_boxed_slice(),
            body.clone(),
        );
        self.environment
            .define(name, LoxValue::LoxFun(lox_function.into()))?;
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
            Expression::Logical(logical) => self.visit_logical(logical),
            Expression::Call(call) => self.visit_call(call),
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
                let b = LoxValue::is_truthy(&right);
                Ok(LoxValue::Bool(!b))
            }
        }
    }

    fn visit_identifier(self: &mut Self, identifier: &Identifier) -> Result<LoxValue, Error> {
        let Identifier {
            name,
            debug_info: DebugInfo { line, position, .. },
            id,
        } = identifier;
        self.environment
            .get(name, id)
            .ok_or_else(|| Error::RuntimeError {
                line: *line,
                position: *position,
                message: format!("Variable {name} not defined!"),
            })
    }

    fn visit_assignment(
        self: &mut Self,
        target: &Identifier,
        value: &Expression,
    ) -> Result<LoxValue, Error> {
        let value = self.evaluate(&value)?;

        let Identifier {
            name,
            debug_info: DebugInfo { line, position, .. },
            id,
        } = target;

        self.environment
            .assign(&name, id, value)
            .ok_or_else(|| Error::RuntimeError {
                line: *line,
                position: *position,
                message: format!("Variable {name} already declared at {line}:{position}!"),
            })
    }

    fn visit_logical(self: &mut Self, logical: &Logical) -> Result<LoxValue, Error> {
        let left = self.evaluate(&logical.left)?;
        match &logical.operator {
            LogicalOperator::Or(debug) => {
                self.set_debug(&debug);
                if LoxValue::is_truthy(&left) {
                    return Ok(left);
                }
            }
            LogicalOperator::And(debug) => {
                self.set_debug(&debug);
                if !LoxValue::is_truthy(&left) {
                    return Ok(left);
                }
            }
        }
        let right = self.evaluate(&logical.right)?;
        Ok(right)
    }

    fn visit_call(self: &mut Self, call: &Call) -> Result<LoxValue, Error> {
        let Call { calle, args, .. } = call;

        let calle = self.evaluate(calle)?;

        match calle {
            LoxValue::LoxFun(fun) => {
                let mut arg_values: Vec<LoxValue> = Vec::new();

                for exp in args {
                    arg_values.push(self.evaluate(exp)?);
                }

                if fun.arity() != args.len() {
                    return Err(self.error(format!(
                        "Expected {} arguments, got {}.",
                        fun.arity(),
                        args.len()
                    )));
                }

                let parent = self.environment.push_closure(fun.captured_scope);
                for (identifier, value) in
                    std::iter::zip(fun.args.into_iter(), arg_values.into_iter())
                {
                    self.environment.define(identifier, value.clone())?;
                }
                let ret_value = match self.run(&fun.body.statements) {
                    // napotkano Statement::Return podczas wykonywania funkcji
                    Ok(LoxResult::Return(value)) => Ok(value),
                    // ciało funkcji nie zawierało wyrażenia return, być może inne przypadki
                    Ok(LoxResult::None) => Ok(LoxValue::Nil),
                    // RuntimeError
                    Err(e) => Err(e),
                };
                self.environment.pop_closure(parent);

                ret_value
            }
            LoxValue::ForeinFun(fun) => {
                let mut arg_values: Vec<LoxValue> = Vec::new();

                for exp in args {
                    arg_values.push(self.evaluate(exp)?);
                }

                if fun.arity() != args.len() {
                    Err(self.error(format!(
                        "Expected {} arguments, got {}.",
                        fun.arity(),
                        args.len()
                    )))
                } else {
                    Ok((fun.fun)(self, arg_values.into_boxed_slice())?)
                }
            }
            _ => Err(self.error("Expected a function")),
        }
    }

    fn error<S: Into<String>>(&self, message: S) -> Error {
        Error::RuntimeError {
            line: self.line,
            position: self.position,
            message: message.into(),
        }
    }
}

#[test]
fn runtime_error_string_negation() {
    use crate::parser::Parser;
    use crate::scanner;
    let source = "-\"asdf\";".to_string();
    let tokens = scanner::scan_tokens(&source).unwrap();
    let tree = Parser::new().parse(tokens).unwrap();
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
    use crate::parser::Parser;
    use crate::resolver;
    use crate::scanner;
    let source = "print 2 + 2 * 2 / (3-2) * 1;".to_string();
    let tokens = scanner::scan_tokens(&source).unwrap();
    let tree = Parser::new().parse(tokens).unwrap();
    let access_table = resolver::resolve(&tree).unwrap();
    let mut interp = Interpreter::new();
    interp.execute(&tree, access_table).unwrap();
}

#[test]
fn variables() {
    use crate::parser::Parser;
    use crate::resolver;
    use crate::scanner;
    let source = "var a = 1; a = a +2;".to_string();
    let tokens = scanner::scan_tokens(&source).unwrap();
    let tree = Parser::new().parse(tokens).unwrap();
    let access_table = resolver::resolve(&tree).unwrap();
    let mut interp = Interpreter::new();
    interp.execute(&tree, access_table).unwrap();
    let val = interp
        .environment
        .get_global(&"a".to_string())
        .expect("Expected variable `a` to be defined.");

    assert_eq!(val, LoxValue::Number(3.));
}

#[test]
fn loops() {
    use crate::parser::Parser;
    use crate::resolver;
    use crate::scanner;
    let source = concat!(
        "var a = 1;",
        "for (var i = 0; i<10; i = i + 1)",
        "{a = a+2;}"
    )
    .to_string();
    let mut parser = Parser::new();
    let tokens = scanner::scan_tokens(&source).unwrap();
    let program = parser.parse(tokens).unwrap();
    let access_table = resolver::resolve(&program).unwrap();
    let mut interp = Interpreter::new();
    interp.execute(&program, access_table).unwrap();
    let val = interp
        .environment
        .get_global(&"a".to_string())
        .expect("Expected variable `a` to be defined.");

    assert_eq!(val, LoxValue::Number(21.));
}

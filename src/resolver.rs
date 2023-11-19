use std::{collections::HashMap, num::NonZeroUsize};

use crate::{
    error::Error,
    expression::{DebugInfo, Expression, Identifier, IdentifierId},
    statement::{Block, Statement},
};

/*
pub enum IdentifierScope {
    Global,
    Local(NonZeroUsize),
}
*/

#[derive(Clone, Copy, Debug)]
pub struct ScopeDepth(NonZeroUsize);

impl ScopeDepth {
    pub fn get(&self) -> usize {
        self.0.get() - 1
    }

    fn from(depth: usize, number_of_parent_scopes: usize) -> Option<ScopeDepth> {
        NonZeroUsize::new(if depth == number_of_parent_scopes {
            0
        } else {
            depth + 1
        })
        .and_then(|v| Some(ScopeDepth(v)))
    }
}

#[derive(Debug)]
pub struct AccessTable {
    access_table: HashMap<IdentifierId, ScopeDepth>,
}

impl AccessTable {
    pub fn empty() -> Self {
        Self {
            access_table: HashMap::new(),
        }
    }

    /// returned value represents a depth at which to
    /// look for a value of given IdentifierId,
    /// if id is not found, then identifier
    /// refers to a value in global scope
    pub fn get(&self, id: &IdentifierId) -> Option<ScopeDepth> {
        self.access_table.get(id).copied()
    }

    /// store the depth of scope at which to look for identifier `i`
    fn put(&mut self, i: IdentifierId, depth: Option<ScopeDepth>) -> Result<(), ()> {
        if let Some(depth) = depth {
            if self.access_table.insert(i, depth).is_none() {
                // identifier refers to an object in local scope
                Ok(())
            } else {
                // redefinition of the same identifier, should not be possible
                Err(())
            }
        } else {
            // identifier refers to an object in global scope
            Ok(())
        }
    }

    pub fn add_all(&mut self, other: AccessTable) -> Result<(), ()> {
        for (id, depth) in other.access_table {
            self.put(id, Some(depth))?;
        }
        Ok(())
    }
}

pub struct Resolver {
    pub access_table: AccessTable,
    pub scopes: Vec<HashMap<String, bool>>,
    pub line: usize,
    pub position: usize,
}

impl Resolver {
    pub fn resolve(&mut self, statements: &Vec<Statement>) -> Result<(), Error> {
        statements
            .iter()
            .try_for_each(|statement| self.visit_statement(statement))
    }

    fn visit_statement(&mut self, statement: &Statement) -> Result<(), Error> {
        match statement {
            Statement::Nop => Ok(()),
            Statement::Expression(e) => self.visit_expression(e),
            Statement::Print(e) => self.visit_expression(e),
            Statement::Block(block) => self.visit_block(block),
            Statement::Return { value: Some(value) } => self.visit_expression(value),
            Statement::Return { value: None } => Ok(()),
            Statement::Variable {
                name: identifier,
                initializer: Some(initializer),
            } => {
                self.declare(&identifier.name)?;
                self.visit_expression(initializer)?;
                self.define(&identifier.name)?;
                Ok(())
            }
            Statement::Variable {
                name: identifier,
                initializer: None,
            } => {
                self.declare(&identifier.name)?;
                self.define(&identifier.name)?;
                Ok(())
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expression(condition)?;
                self.visit_block(then_branch)?;
                if let Some(else_branch) = else_branch.as_ref() {
                    self.visit_block(else_branch)?;
                }
                Ok(())
            }
            Statement::While { condition, body } => {
                self.visit_expression(condition)?;
                self.visit_block(body)?;
                Ok(())
            }
            Statement::Function {
                name: identifier,
                args,
                body,
            } => {
                self.declare(&identifier.name)?;
                self.define(&identifier.name)?;

                self.resolve_function(args, body)?;
                Ok(())
            }
        }
    }

    fn resolve_local_identifier(&mut self, id: IdentifierId, name: String) -> Result<(), Error> {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name) {
                return self
                    .access_table
                    .put(id, ScopeDepth::from(i, self.scopes.len()))
                    .map_err(|_| self.error("Tried to resolve the same identifier twice."));
            }
        }
        Ok(())
    }

    fn declare(&mut self, name: &String) -> Result<(), Error> {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.clone(), false);
        } else {
            // identifier is declared in global scope
        }
        Ok(())
    }

    fn define(&mut self, name: &String) -> Result<(), Error> {
        if let Some(scope) = self.scopes.last_mut() {
            *scope
                .get_mut(name)
                .expect("Variable or should be declared before definition") = true;
        } else {
            // identifier is defined in global scope
        }
        Ok(())
    }

    fn visit_block(&mut self, block: &Block) -> Result<(), Error> {
        self.scopes.push(HashMap::new());
        self.resolve(&block.statements)?;
        self.scopes.pop();

        Ok(())
    }

    fn resolve_function(&mut self, args: &[Identifier], body: &Block) -> Result<(), Error> {
        self.scopes.push(HashMap::new());
        for arg in args {
            self.set_location(&arg.debug_info);
            self.declare(&arg.name)?;
            self.define(&arg.name)?;
        }
        self.resolve(&body.statements)?;
        self.scopes.pop();
        Ok(())
    }

    fn visit_expression(&mut self, expression: &Expression) -> Result<(), Error> {
        match expression {
            Expression::Binary(op) => {
                self.visit_expression(&op.left)?;
                self.visit_expression(&op.right)?;
                Ok(())
            }
            Expression::Grouping(grouping) => {
                self.visit_expression(&grouping.expression)?;
                Ok(())
            }
            Expression::Literal(_) => Ok(()),
            Expression::Unary(op) => {
                self.visit_expression(&op.right)?;
                Ok(())
            }
            Expression::Identifier(identifier) => {
                self.visit_identifier(identifier)?;
                Ok(())
            }
            Expression::Assignment(assignment) => {
                self.visit_expression(&assignment.value)?;
                let target = &assignment.target;
                self.set_location(&target.debug_info);
                self.resolve_local_identifier(target.id, target.name.clone())?;
                Ok(())
            }
            Expression::Logical(op) => {
                self.visit_expression(&op.left)?;
                self.visit_expression(&op.right)?;
                Ok(())
            }
            Expression::Call(call) => {
                self.visit_expression(&call.calle)?;
                for arg in &call.args {
                    self.visit_expression(&arg)?;
                }
                Ok(())
            }
        }
    }

    fn visit_identifier(&mut self, identifier: &Identifier) -> Result<(), Error> {
        self.set_location(&identifier.debug_info);

        if self
            .scopes
            .last()
            .and_then(|scope| scope.get(&identifier.name))
            .is_some_and(|defined| *defined == false)
        {
            return Err(self.error("Can't read local variable in its initializer."));
        }

        self.resolve_local_identifier(identifier.id, identifier.name.clone())?;
        Ok(())
    }

    fn error<S: Into<String>>(&self, message: S) -> Error {
        Error::ResolverError {
            line: self.line,
            position: self.position,
            message: message.into(),
        }
    }

    fn set_location(&mut self, debug_info: &DebugInfo) {
        self.line = debug_info.line;
        self.position = debug_info.position;
    }
}

pub fn resolve(statements: &Vec<Statement>) -> Result<AccessTable, Error> {
    let mut resolver = Resolver {
        line: 0,
        position: 0,
        access_table: AccessTable::empty(),
        scopes: Vec::new(),
    };

    resolver.resolve(statements)?;

    return Ok(resolver.access_table);
}

#[test]
fn test_resolver() {
    use crate::interpreter::Interpreter;
    use crate::lox_function::ForeinFun;
    use crate::lox_value::LoxValue;
    use crate::parser::Parser;
    use crate::resolver;
    use crate::scanner;
    let source = concat!(
        "var a = \"global\";",
        "{",
        "fun local_fun() {test(a);}",
        "local_fun();",
        "var a = \"local\";",
        "local_fun();",
        "}"
    )
    .to_string();

    let tokens = scanner::scan_tokens(&source).unwrap();
    let tree = Parser::new().parse(tokens).unwrap();
    let access_table = resolver::resolve(&tree).unwrap();
    let mut interp = Interpreter::new();

    let global_identifier = Identifier {
        name: "test".to_owned(),
        id: 0,
        debug_info: DebugInfo {
            line: 0,
            position: 0,
            lexeme: "<native test>".to_owned(),
        },
    };

    static mut VALUES_OF_A: Vec<String> = Vec::new();
    fn test(_env: &mut Interpreter, args: Box<[LoxValue]>) -> Result<LoxValue, Error> {
        unsafe {
            VALUES_OF_A.push(args[0].to_string());
        }
        Ok(LoxValue::Nil)
    }

    let fun = ForeinFun::new("test".to_owned(), 1, test);

    interp
        .environment
        .define(&global_identifier, LoxValue::ForeinFun(fun.into()))
        .unwrap();

    interp.execute(&tree, access_table).unwrap();

    unsafe {
        assert_eq!(VALUES_OF_A, ["global", "global"]);
    }
}

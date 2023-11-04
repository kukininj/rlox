use std::{collections::HashMap, num::NonZeroUsize};

use crate::{
    error::Error,
    expression::{DebugInfo, Expression, Identifier, IdentifierId},
    parser::ParserContext,
    statement::{Block, Statement},
};

pub struct AccessTable {
    access_table: HashMap<IdentifierId, NonZeroUsize>,
}

impl AccessTable {
    // TODO: może zmienić nazę depth na level, po implementacji pobierania wartości z Environment
    pub fn empty() -> Self {
        Self {
            access_table: HashMap::new(),
        }
    }

    /// returned value represents a depth at which to
    /// look for a value of given IdentifierId,
    /// if id is not found, then identifier
    /// refers to a value in global scope
    pub fn get(&self, id: &IdentifierId) -> Option<NonZeroUsize> {
        self.access_table.get(id).copied()
    }

    /// store the depth of scope at which to look for identifier `i`
    fn put(&mut self, i: IdentifierId, depth: Option<NonZeroUsize>) -> Result<(), ()> {
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
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name) {
                return self
                    .access_table
                    .put(id, NonZeroUsize::new(i))
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
        self.visit_block(body)?;
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
                self.resolve_local_identifier(target.id, target.name.clone())?;
                Ok(())
            }
            Expression::Logical(op) => {
                self.visit_expression(&op.left)?;
                self.visit_expression(&op.right)?;
                Ok(())
            }
            Expression::Call(call) => {
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
            Err(self.error("Can't read local variable in its initializer."))
        } else {
            self.resolve_local_identifier(identifier.id, identifier.name.clone())?;
            Ok(())
        }
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

pub fn resolve(statements: &Vec<Statement>, parser_context: ParserContext) -> AccessTable {
    let mut resolver = Resolver {
        line: 0,
        position: 0,
        access_table: AccessTable::empty(),
        scopes: Vec::new(),
    };

    resolver.resolve(statements).unwrap();

    return resolver.access_table;
}

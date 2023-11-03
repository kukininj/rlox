use std::collections::HashMap;

use crate::{
    environment::Environment,
    expression::IdentifierId,
    lox_value::LoxValue,
    statement::{Block, Statement},
};

pub struct AccessTable {
    // access_table holds depth at which to look for a variable holding a specific identifier
    // IdentifierId is used as an index into the vector
    access_table: Vec<usize>,
}

impl AccessTable {
    pub fn empty() -> Self {
        Self {
            access_table: Vec::new(),
        }
    }
    pub fn resolve(&self, id: IdentifierId) -> usize {
        todo!()
    }
}

pub struct Resolver {
    pub access_table: AccessTable,
    // boolean value tells if a variable was initialized
    pub scopes: Vec<HashMap<String, bool>>,
    pub line: usize,
    pub position: usize,
}

impl Resolver {
    pub fn visit(&mut self, statement: &Statement) {
        match statement {
            Statement::Nop => {}
            Statement::Expression(_) => todo!(),
            Statement::Print(_) => todo!(),
            Statement::Variable { name, initializer } => todo!(),
            Statement::Block(block) => self.visit_block(block),
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => todo!(),
            Statement::While { condition, body } => todo!(),
            Statement::Function { name, args, body } => todo!(),
            Statement::Return { value } => todo!(),
        }
    }

    fn visit_block(&mut self, block: &Block) {
        self.scopes.push(HashMap::new());
        for stmt in &block.statements {
            todo!();
        }
        self.scopes.pop();
    }
}

pub fn resolve(statements: &Vec<Statement>) -> AccessTable {
    let mut resolver = Resolver {
        line: 0,
        position: 0,
        access_table: AccessTable::empty(),
        scopes: Vec::new(),
    };

    for stmt in statements {
        resolver.visit(stmt);
    }

    return resolver.access_table;
}

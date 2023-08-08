use std::collections::HashMap;

use crate::error::Error;
use crate::expression::{DebugInfo, Identifier};
use crate::lox_value::LoxValue;

pub struct Variable {
    value: LoxValue,
    defined_at: DebugInfo,
}

pub struct Environment {
    values: HashMap<String, Variable>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }
    pub fn define(
        &mut self,
        Identifier(name, debug): Identifier,
        value: LoxValue,
    ) -> Result<(), Error> {
        if self.values.contains_key(&name) {
            Err(Error::RuntimeError {
                line: debug.line,
                position: debug.position,
                message: format!("Variable {name} already declared!"),
            })
        } else {
            self.values.insert(
                name,
                Variable {
                    value,
                    defined_at: debug,
                },
            );
            Ok(())
        }
    }

    pub fn get(&self, Identifier(name, debug): Identifier) -> Result<LoxValue, Error> {
        if let Some(Variable { value, .. }) = self.values.get(&name) {
            Ok(value.clone())
        } else {
            Err(Error::RuntimeError {
                line: debug.line,
                position: debug.position,
                message: format!("Variable {name} not defined!"),
            })
        }
    }
}

use std::collections::HashMap;

use crate::error::Error;
use crate::expression::{DebugInfo, Identifier};
use crate::lox_value::LoxValue;

pub struct Variable {
    value: LoxValue,
    defined_at: DebugInfo,
}

pub struct Environment {
    stack: Vec<Frame>,
}

pub struct Frame {
    pub values: HashMap<String, Variable>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            stack: vec![Frame::new()],
        }
    }

    pub fn push(&mut self) {
        self.stack.push(Frame::new());
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn head(&mut self) -> &mut Frame {
        let index = self.stack.len() - 1;
        self.stack.get_mut(index).unwrap()
    }

    pub fn define(
        &mut self,
        Identifier(name, debug): &Identifier,
        value: LoxValue,
    ) -> Result<(), Error> {
        if let Some(Variable {
            defined_at: DebugInfo { line, position, .. },
            ..
        }) = self.head().values.get(name)
        {
            // TODO: dont use `name` here, Variable should store its identifier
            Err(Error::RuntimeError {
                line: debug.line,
                position: debug.position,
                message: format!("Variable {name} already declared at {line}:{position}!"),
            })
        } else {
            self.head().values.insert(
                name.clone(),
                Variable {
                    value,
                    defined_at: debug.clone(),
                },
            );
            Ok(())
        }
    }

    pub fn get(&self, identifier: &Identifier) -> Result<LoxValue, Error> {
        for frame in self.stack.iter().rev() {
            if let Some(value) = frame.values.get(&identifier.0).map(|v| v.value.clone()) {
                return Ok(value);
            }
        }
        let Identifier(name, DebugInfo { line, position, .. }) = identifier;
        Err(Error::RuntimeError {
            line: *line,
            position: *position,
            message: format!("Variable {name} not defined!"),
        })
    }

    pub fn assign(&mut self, target: &Identifier, value: LoxValue) -> Result<LoxValue, Error> {
        if let Some(target) = self
            .stack
            .iter_mut()
            .rev()
            .find_map(|frame| frame.values.get_mut(&target.0))
        {
            target.value = value.clone();
            Ok(value)
        } else {
            let Identifier(name, DebugInfo { line, position, .. }) = target;
            Err(Error::RuntimeError {
                line: *line,
                position: *position,
                message: format!("Variable {name} already declared at {line}:{position}!"),
            })
        }
    }
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            values: HashMap::new(),
        }
    }
}

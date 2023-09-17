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

    pub fn get(&self, identifier: &String) -> Option<LoxValue> {
        self.stack
            .iter()
            .rev()
            .find_map(|frame| frame.values.get(identifier))
            .map(|var| var.value.clone())
    }

    pub fn assign(&mut self, target: &String, value: LoxValue) -> Option<LoxValue> {
        self.stack
            .iter_mut()
            .rev()
            .find_map(|frame| frame.values.get_mut(target))
            .map(|var| {
                var.value = value.clone();
                value
            })
    }
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            values: HashMap::new(),
        }
    }
}

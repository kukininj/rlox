use std::collections::HashMap;
use std::num::NonZeroUsize;

use crate::error::Error;
use crate::expression::{DebugInfo, Identifier};
use crate::lox_function::ForeinFun;
use crate::lox_value::LoxValue;
use crate::resolver::ScopeDepth;

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
        Identifier {
            name,
            debug_info: debug,
            ..
        }: &Identifier,
        value: LoxValue,
    ) -> Result<(), Error> {
        if let Some(Variable {
            defined_at: DebugInfo { line, position, .. },
            ..
        }) = self.head().values.get(name)
        {
            Err(Error::RuntimeError {
                line: debug.line,
                position: debug.position,
                message: format!("Variable {name} already defined at {line}:{position}!"),
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

    pub fn get(&self, identifier: &String, depth: Option<ScopeDepth>) -> Option<LoxValue> {
        if let Some(depth) = depth {
            self.stack
                .iter()
                .rev()
                .nth(depth.get())
                .and_then(|frame| frame.values.get(identifier))
                .map(|var| var.value.clone())
        } else {
            self.stack
                .first()
                .and_then(|frame| frame.values.get(identifier))
                .map(|var| var.value.clone())
        }
    }

    pub fn assign(
        &mut self,
        target: &String,
        depth: Option<ScopeDepth>,
        value: LoxValue,
    ) -> Option<LoxValue> {
        if let Some(depth) = depth {
            self.stack
                .iter_mut()
                .rev()
                .nth(depth.get())
                .and_then(|frame| frame.values.get_mut(target))
                .map(|var| {
                    var.value = value.clone();
                    value
                })
        } else {
            self.stack
                .first_mut()
                .and_then(|frame| frame.values.get_mut(target))
                .map(|var| {
                    var.value = value.clone();
                    value
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

#[test]
fn test_function_call() {
    use crate::interpreter::Interpreter;
    use crate::parser;
    use crate::scanner;
    let source = concat!("var a = test(123);",).to_string();
    let tokens = scanner::scan_tokens(&source).unwrap();
    let tree = parser::parse(tokens).unwrap();
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

    fn test(_env: &mut Interpreter, args: Box<[LoxValue]>) -> Result<LoxValue, Error> {
        println!("Woo, called a native function!! args: {args:?}");
        let a = args.get(0).unwrap();

        let str = format!("({})", LoxValue::to_string(a));

        Ok(LoxValue::String(str))
    }

    let fun = ForeinFun::new("test".to_owned(), 1, test);

    interp
        .environment
        .define(&global_identifier, LoxValue::Function(fun.into()))
        .unwrap();

    interp.run(&tree).unwrap();
    let val = interp
        .environment
        .get(&"a".to_string())
        .expect("Expected variable `a` to be defined.");

    assert_eq!(val, LoxValue::String("(123)".to_owned()));
}

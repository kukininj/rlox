use std::collections::HashMap;

use crate::error::Error;
use crate::expression::{DebugInfo, Identifier};
use crate::lox_value::LoxValue;
use crate::resolver::ScopeDepth;

#[derive(Debug)]
pub struct Variable {
    value: LoxValue,
    defined_at: DebugInfo,
}

#[derive(Debug)]
pub struct Environment {
    stack: HashMap<u32, Frame>,
    head: u32,
}

#[derive(Debug)]
pub struct Frame {
    pub values: HashMap<String, Variable>,
    pub parent: Option<u32>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            stack: HashMap::from([(0, Frame::new())]),
            head: 0,
        }
    }

    pub fn push(&mut self) {
        let parent = self.head;
        self.head = self.stack.len() as u32;
        self.stack.insert(self.head, Frame::with_parent(parent));
    }

    pub fn pop(&mut self) {
        self.head = self
            .head()
            .parent
            .expect("tried to get parent of global scope");
    }

    pub fn head(&mut self) -> &mut Frame {
        self.stack.get_mut(&self.head).unwrap()
    }

    pub fn global(&mut self) -> &mut Frame {
        self.stack.get_mut(&0).unwrap()
    }

    fn get_nth_scope(&mut self, n: usize) -> &mut Frame {
        let mut nth_scope = self.head;

        for _ in 0..n {
            nth_scope = self
                .stack
                .get_mut(&nth_scope)
                .expect("found invalid scope identifier")
                .parent
                .expect("tried to get parent scope of global scope");
        }

        return self.stack.get_mut(&nth_scope).unwrap();
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

    pub fn get(&mut self, identifier: &String, depth: Option<ScopeDepth>) -> Option<LoxValue> {
        if let Some(depth) = depth {
            self.get_nth_scope(depth.get())
                .values
                .get(identifier)
                .map(|var| var.value.clone())
        } else {
            self.global()
                .values
                .get(identifier)
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
            self.get_nth_scope(depth.get())
                .values
                .get_mut(target)
                .map(|var| {
                    var.value = value.clone();
                    value
                })
        } else {
            self.global().values.get_mut(target).map(|var| {
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
            parent: None,
        }
    }

    pub fn with_parent(parent: u32) -> Frame {
        Frame {
            values: HashMap::new(),
            parent: Some(parent),
        }
    }
}

impl Iterator for Frame {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[test]
fn test_function_call() {
    use crate::interpreter::Interpreter;
    use crate::lox_function::ForeinFun;
    use crate::parser::Parser;
    use crate::resolver;
    use crate::scanner;
    let source = concat!("var a = test(123);",).to_string();
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

    fn test(_env: &mut Interpreter, args: Box<[LoxValue]>) -> Result<LoxValue, Error> {
        println!("Woo, called a native function!! args: {args:?}");
        let a = args.get(0).unwrap();

        let str = format!("({})", LoxValue::to_string(a));

        Ok(LoxValue::String(str))
    }

    let fun = ForeinFun::new("test".to_owned(), 1, test);

    interp
        .environment
        .define(&global_identifier, LoxValue::ForeinFun(fun.into()))
        .unwrap();

    interp.execute(&tree, access_table).unwrap();
    let val = interp
        .environment
        .get(&"a".to_string(), None)
        .expect("Expected variable `a` to be defined.");

    assert_eq!(val, LoxValue::String("(123)".to_owned()));
}

#[test]
fn test_closure_capturing() {
    use crate::interpreter::Interpreter;
    use crate::lox_value::LoxValue;
    use crate::parser::Parser;
    use crate::resolver;
    use crate::scanner;
    let source = vec![
        "fun funkcja() {",
        "    var a = 123;",
        "    print a;",
        "    fun local_fun() {",
        "        print a;",
        "    }",
        "    return local_fun;",
        "}",
        "var a = (funkcja())();",
    ]
    .join("\n");

    let tokens = scanner::scan_tokens(&source).unwrap();
    let tree = Parser::new().parse(tokens).unwrap();
    let access_table = resolver::resolve(&tree).unwrap();
    // panic!("{:?}", access_table);
    let mut interp = Interpreter::new();

    interp.execute(&tree, access_table).unwrap();

    let val = interp
        .environment
        .get(&"a".to_string(), None)
        .expect("Expected variable `a` to be defined.");

    // TODO: fix when return statements implemented
    assert_eq!(val, LoxValue::Number(123.));
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::error::Error;
use crate::expression::{DebugInfo, Identifier, IdentifierId};
use crate::lox_value::LoxValue;
use crate::resolver::AccessTable;

#[derive(Debug)]
pub struct Variable {
    value: LoxValue,
    defined_at: DebugInfo,
}

#[derive(Debug, Clone)]
pub struct FrameRef(Rc<RefCell<Frame>>);
impl FrameRef {
    fn global() -> FrameRef {
        FrameRef(Rc::new(RefCell::new(Frame {
            values: HashMap::new(),
            parent: None,
        })))
    }

    fn with_parent(parent: FrameRef) -> FrameRef {
        FrameRef(Rc::new(RefCell::new(Frame {
            values: HashMap::new(),
            parent: Some(parent),
        })))
    }

    fn get_parent(&self) -> Option<FrameRef> {
        self.0.as_ref().borrow().parent.clone()
    }

    fn get(&self, name: &String) -> Option<LoxValue> {
        self.0
            .as_ref()
            .borrow()
            .values
            .get(name)
            .map(|v| v.value.clone())
    }

    fn assign(&self, name: &String, value: LoxValue) -> Option<LoxValue> {
        let mut frame = self.0.as_ref().borrow_mut();
        let variable = frame.values.get_mut(name);

        if let Some(variable) = variable {
            variable.value = value;
            Some(variable.value.clone())
        } else {
            None
        }
    }

    fn define(&self, name: &String, variable: Variable) -> Result<(), DebugInfo> {
        let mut frame = self.0.as_ref().borrow_mut();

        if let Some(Variable {
            value: _,
            defined_at,
        }) = frame.values.get(name)
        {
            Err(defined_at.clone())
        } else {
            frame.values.insert(name.clone(), variable);
            Ok(())
        }
    }
}

impl Deref for FrameRef {
    type Target = Rc<RefCell<Frame>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FrameRef {
    fn deref_mut(&mut self) -> &mut Rc<RefCell<Frame>> {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct Environment {
    closure_stack: Vec<FrameRef>,
    pub access_table: AccessTable,
    // head: FrameId,
    head: FrameRef,
    global: FrameRef,
}

#[derive(Debug)]
pub struct Frame {
    values: HashMap<String, Variable>,
    // parent: Option<FrameId>,
    parent: Option<FrameRef>,
}

// impl Drop for Frame {
//     fn drop(&mut self) {
//         dbg!(&self.values);
//     }
// }

impl Environment {
    pub fn new() -> Self {
        let global = FrameRef::global();
        Environment {
            closure_stack: Vec::new(),
            access_table: AccessTable::empty(),
            head: global.clone(),
            global,
        }
    }

    pub fn get_current_frame(&self) -> FrameRef {
        self.head.clone()
    }

    pub fn extend_access_table(&mut self, access_table: AccessTable) -> Result<(), ()> {
        self.access_table.add_all(access_table)?;
        Ok(())
    }

    pub fn push(&mut self) {
        let parent = self.head.clone();
        self.head = FrameRef::with_parent(parent);
    }

    pub fn push_closure(&mut self, frame: FrameRef) {
        let parent = self.head.clone();
        self.head = FrameRef::with_parent(frame);
        self.closure_stack.push(parent);
    }

    pub fn pop(&mut self) {
        let head = self.head.get_parent();
        self.head = head.expect("tried to get parent of global scope");
    }

    pub fn pop_closure(&mut self) {
        self.head = self
            .closure_stack
            .pop()
            .expect("tried to pop closure scope, when no closure scope was pushed before");
    }

    fn get_nth_scope(&mut self, n: usize) -> FrameRef {
        let mut nth_scope = self.head.clone();

        for _ in 0..n {
            let tmp = nth_scope.get_parent().clone();
            nth_scope = tmp.expect("tried to get parent scope of global scope");
        }

        return nth_scope;
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
        match self.head.define(
            name,
            Variable {
                value,
                defined_at: debug.clone(),
            },
        ) {
            Ok(_) => Ok(()),
            Err(DebugInfo {
                line,
                position,
                lexeme: _,
            }) => Err(Error::RuntimeError {
                line,
                position,
                message: format!("Variable {name} already defined at {line}:{position}!"),
            }),
        }
    }

    pub fn get(&mut self, name: &String, id: &IdentifierId) -> Option<LoxValue> {
        if let Some(depth) = self.access_table.get(id) {
            self.get_nth_scope(depth.get()).get(name)
        } else {
            self.global.get(name)
        }
    }

    #[allow(dead_code)]
    pub fn get_global(&mut self, name: &String) -> Option<LoxValue> {
        self.global.get(name)
    }

    pub fn assign(
        &mut self,
        target: &String,
        id: &IdentifierId,
        value: LoxValue,
    ) -> Option<LoxValue> {
        if let Some(depth) = self.access_table.get(id) {
            self.get_nth_scope(depth.get()).assign(target, value)
        } else {
            self.global.assign(target, value)
        }
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
        .get_global(&"a".to_string())
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
        "    fun local_fun() {",
        "        return a;",
        "    }",
        "    return local_fun;",
        "}",
        "var ret_val = (funkcja())();",
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
        .get_global(&"ret_val".to_string())
        .expect("Expected variable `ret_val` to be defined.");

    // TODO: fix when return statements implemented
    assert_eq!(val, LoxValue::Number(123.));
}

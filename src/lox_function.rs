use core::fmt;
use std::rc::Rc;

use crate::{
    expression::Identifier, interpreter::Interpreter, lox_value::LoxValue, statement::Block, Error,
};

#[derive(Clone)]
pub struct LoxFunction(Rc<dyn LoxFunction_>);

impl LoxFunction {
    pub fn call(
        &mut self,
        env: &mut Interpreter,
        args: Box<[LoxValue]>,
    ) -> Result<LoxValue, Error> {
        self.0.call(env, args)
    }
    pub fn arity(&self) -> usize {
        self.0.arity()
    }
    pub fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Debug for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        std::rc::Rc::ptr_eq(&self.0, &other.0)
    }
}

trait LoxFunction_ {
    fn call(&self, env: &mut Interpreter, args: Box<[LoxValue]>) -> Result<LoxValue, Error>;
    fn arity(&self) -> usize;
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

#[derive(Debug)]
pub struct ForeinFun {
    name: String,
    arity: usize,
    fun: fn(&mut Interpreter, Box<[LoxValue]>) -> Result<LoxValue, Error>,
}

impl ForeinFun {
    pub fn new(
        name: String,
        arity: usize,
        fun: fn(&mut Interpreter, Box<[LoxValue]>) -> Result<LoxValue, Error>,
    ) -> Self {
        Self { name, arity, fun }
    }
}

impl LoxFunction_ for ForeinFun {
    fn call(&self, env: &mut Interpreter, args: Box<[LoxValue]>) -> Result<LoxValue, Error> {
        if self.arity != args.len() {
            Err(Error::RuntimeError {
                line: 0,
                position: 0,
                message: format!("Expected {} arguments, got {}.", self.arity, args.len()),
            })
        } else {
            Ok((self.fun)(env, args)?)
        }
    }

    fn arity(&self) -> usize {
        self.arity
    }

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Into<LoxFunction> for ForeinFun {
    fn into(self) -> LoxFunction {
        LoxFunction(Rc::new(self))
    }
}

#[derive(Debug)]
pub struct NativeFun {
    name: Identifier,
    args: Box<[Identifier]>,
    body: Block,
}

impl LoxFunction_ for NativeFun {
    fn call(&self, env: &mut Interpreter, args: Box<[LoxValue]>) -> Result<LoxValue, Error> {
        if self.args.len() != args.len() {
            Err(Error::RuntimeError {
                line: 0,
                position: 0,
                message: format!(
                    "Expected {} arguments, got {}.",
                    self.args.len(),
                    args.len()
                ),
            })
        } else {
            env.environment.push();
            for (identifier, value) in std::iter::zip(self.args.into_iter(), args.into_iter()) {
                env.environment.define(identifier, value.clone())?;
            }
            let ret_value = env.run(&self.body.statements).map(|_| LoxValue::Nil);
            env.environment.pop();

            ret_value
        }
    }

    fn arity(&self) -> usize {
        self.args.len()
    }

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl NativeFun {
    pub(crate) fn new(name: Identifier, args: Box<[Identifier]>, body: Block) -> Self {
        NativeFun { name, args, body }
    }
}

impl Into<LoxFunction> for NativeFun {
    fn into(self) -> LoxFunction {
        LoxFunction(Rc::new(self))
    }
}

#[test]
fn test_fun_stmt() {
    use crate::parser;
    use crate::scanner;
    let source = concat!("fun funkcja(arg) {print arg;}", "var a = funkcja(123);",).to_string();
    let tokens = scanner::scan_tokens(&source).unwrap();
    let tree = parser::parse(tokens).unwrap();
    let mut interp = Interpreter::new();
    interp.run(&tree).unwrap();
    let val = interp
        .environment
        .get(&"a".to_string())
        .expect("Expected variable `a` to be defined.");

    // TODO: fix when return statements implemented
    // assert_eq!(val, LoxValue::Number(123.));
    assert_eq!(val, LoxValue::Nil);
}

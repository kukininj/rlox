use core::fmt;
use std::rc::Rc;

use crate::{
    environment::Environment, expression::Identifier, lox_value::LoxValue, statement::Block, Error,
};

#[derive(Clone)]
pub struct LoxFunction(Rc<dyn LoxFunction_>);

impl LoxFunction {
    pub fn call(&self, env: &mut Environment, args: Box<[LoxValue]>) -> Result<LoxValue, Error> {
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
    fn call(&self, env: &mut Environment, args: Box<[LoxValue]>) -> Result<LoxValue, Error>;
    fn arity(&self) -> usize;
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

#[derive(Debug)]
pub struct ForeinFun {
    name: String,
    arity: usize,
    fun: fn(&mut Environment, Box<[LoxValue]>) -> Result<LoxValue, Error>,
}

impl ForeinFun {
    pub fn new(
        name: String,
        arity: usize,
        fun: fn(&mut Environment, Box<[LoxValue]>) -> Result<LoxValue, Error>,
    ) -> Self {
        Self { name, arity, fun }
    }
}

impl Into<LoxFunction> for ForeinFun {
    fn into(self) -> LoxFunction {
        LoxFunction(Rc::new(self))
    }
}

impl LoxFunction_ for ForeinFun {
    fn call(&self, env: &mut Environment, args: Box<[LoxValue]>) -> Result<LoxValue, Error> {
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

pub struct LoxFunc {
    name: Identifier,
    args: Vec<Identifier>,
    body: Block,
}

use core::fmt;

use crate::{
    expression::Identifier, interpreter::Interpreter, lox_value::LoxValue, statement::Block, Error,
};

#[derive(PartialEq, Clone, Debug)]
pub struct ForeinFun {
    pub name: String,
    arity: usize,
    pub fun: fn(&mut Interpreter, Box<[LoxValue]>) -> Result<LoxValue, Error>,
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

impl core::fmt::Display for ForeinFun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl ForeinFun {
    pub fn arity(&self) -> usize {
        self.arity
    }
}

#[derive(Debug)]
pub struct LoxFun {
    pub name: Identifier,
    pub args: Box<[Identifier]>,
    pub body: Block,
}

impl fmt::Display for LoxFun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl LoxFun {
    pub fn arity(&self) -> usize {
        self.args.len()
    }
}

impl LoxFun {
    pub(crate) fn new(name: Identifier, args: Box<[Identifier]>, body: Block) -> Self {
        LoxFun { name, args, body }
    }
}

#[test]
fn test_fun_stmt() {
    use crate::parser::Parser;
    use crate::resolver;
    use crate::scanner;
    let source = concat!("fun funkcja(arg) {return arg;}", "var a = funkcja(123);",).to_string();
    let tokens = scanner::scan_tokens(&source).unwrap();
    let tree = Parser::new().parse(tokens).unwrap();
    let access_table = resolver::resolve(&tree).unwrap();
    let mut interp = Interpreter::new();
    interp.execute(&tree, access_table).unwrap();
    let val = interp
        .environment
        .get_global(&"a".to_string())
        .expect("Expected variable `a` to be defined.");

    // TODO: fix when return statements implemented
    assert_eq!(val, LoxValue::Number(123.));
    // assert_eq!(val, LoxValue::Nil);
}

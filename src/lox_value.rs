use std::mem::discriminant;

use crate::{
    error::Error,
    lox_function::{ForeinFun, LoxFun},
};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum LoxValue {
    Number(f64),
    Bool(bool),
    String(String),
    // Object(LoxObject),
    LoxFun(Rc<LoxFun>),
    ForeinFun(Rc<ForeinFun>),
    Nil,
}

impl PartialEq for LoxValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LoxValue::Number(a), LoxValue::Number(b)) => a == b,
            (LoxValue::Number(_), _) => false,
            (LoxValue::Bool(a), LoxValue::Bool(b)) => a == b,
            (LoxValue::Bool(_), _) => false,
            (LoxValue::String(a), LoxValue::String(b)) => a == b,
            (LoxValue::String(_), _) => false,
            (LoxValue::LoxFun(a), LoxValue::LoxFun(b)) => Rc::ptr_eq(a, b),
            (LoxValue::LoxFun(_), _) => false,
            (LoxValue::ForeinFun(a), LoxValue::ForeinFun(b)) => Rc::ptr_eq(a, b),
            (LoxValue::ForeinFun(_), _) => false,
            (LoxValue::Nil, LoxValue::Nil) => true,
            (LoxValue::Nil, _) => false,
        }
    }
}

impl core::fmt::Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxValue::Number(n) => write!(f, "{}", n),
            LoxValue::Bool(b) => write!(f, "{}", b),
            LoxValue::String(s) => write!(f, "{}", s),
            // LoxValue::Object(o) => write!(f, "{}", o.to_string()),
            LoxValue::Nil => write!(f, "nil"),
            LoxValue::LoxFun(fun) => write!(f, "{}", fun),
            LoxValue::ForeinFun(fun) => write!(f, "{}", fun),
        }
    }
}

impl LoxValue {
    pub fn add(left: LoxValue, right: LoxValue) -> Result<LoxValue, Error> {
        match (left, right) {
            (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l + r)),
            (LoxValue::String(l), LoxValue::String(r)) => {
                Ok(LoxValue::String(format!("{}{}", l, r)))
            }
            (left, right) => Err(Error::InternalRuntimeError {
                message: format!("Cannot add: {:?} and {:?}", left, right),
            }),
        }
    }

    pub fn subtract(left: LoxValue, right: LoxValue) -> Result<LoxValue, Error> {
        match (left, right) {
            (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l - r)),
            (left, right) => Err(Error::InternalRuntimeError {
                message: format!("Cannot subtract: {:?} from {:?}", left, right),
            }),
        }
    }

    pub fn multiply(left: LoxValue, right: LoxValue) -> Result<LoxValue, Error> {
        match (left, right) {
            (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l * r)),
            (left, right) => Err(Error::InternalRuntimeError {
                message: format!("Cannot multiply: {:?} by {:?}", left, right),
            }),
        }
    }

    pub fn divide(left: LoxValue, right: LoxValue) -> Result<LoxValue, Error> {
        match (left, right) {
            (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l / r)),
            (left, right) => Err(Error::InternalRuntimeError {
                message: format!("Cannot divide: {:?} over {:?}", left, right),
            }),
        }
    }

    // Follows IEEE 754, ie: (NaN == NaN) -> False
    pub fn equal(left: LoxValue, right: LoxValue) -> Result<LoxValue, Error> {
        Ok(LoxValue::Bool(left == right))
    }

    pub fn not_equal(left: LoxValue, right: LoxValue) -> Result<LoxValue, Error> {
        Ok(LoxValue::Bool(left != right))
    }

    pub fn greater(left: LoxValue, right: LoxValue) -> Result<LoxValue, Error> {
        match (left, right) {
            (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Bool(l > r)),
            (left, right) => Err(Error::InternalRuntimeError {
                message: format!("Cannot check if: {:?} > {:?}", left, right),
            }),
        }
    }

    pub fn greater_equal(left: LoxValue, right: LoxValue) -> Result<LoxValue, Error> {
        match (left, right) {
            (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Bool(l >= r)),
            (left, right) => Err(Error::InternalRuntimeError {
                message: format!("Cannot check if: {:?} >= {:?}", left, right),
            }),
        }
    }

    pub fn less(left: LoxValue, right: LoxValue) -> Result<LoxValue, Error> {
        match (left, right) {
            (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Bool(l < r)),
            (left, right) => Err(Error::InternalRuntimeError {
                message: format!("Cannot check if: {:?} < {:?}", left, right),
            }),
        }
    }

    pub fn less_equal(left: LoxValue, right: LoxValue) -> Result<LoxValue, Error> {
        match (left, right) {
            (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Bool(l <= r)),
            (left, right) => Err(Error::InternalRuntimeError {
                message: format!("Cannot check if: {:?} <= {:?}", left, right),
            }),
        }
    }

    pub fn negative(value: LoxValue) -> Result<LoxValue, Error> {
        match value {
            LoxValue::Number(value) => Ok(LoxValue::Number(-value)),
            _ => Err(Error::InternalRuntimeError {
                message: format!("Cannot negate: {:?}", value),
            }),
        }
    }

    pub fn is_truthy(value: &LoxValue) -> bool {
        match value {
            LoxValue::Bool(b) => *b,
            LoxValue::Nil => false,
            _ => true,
        }
    }

    pub fn to_string(value: &LoxValue) -> String {
        match value {
            LoxValue::Number(n) => n.to_string(),
            LoxValue::Bool(b) => b.to_string(),
            LoxValue::String(s) => s.clone(),
            LoxValue::Nil => "nil".to_owned(),
            LoxValue::LoxFun(f) => f.to_string(),
            LoxValue::ForeinFun(f) => f.to_string(),
        }
    }
}

#[test]
fn comparison_tests() {
    use crate::interpreter::Interpreter;
    use crate::parser;
    use crate::scanner;
    for (source, expected) in [
        ("1<2;", true),
        ("1<=2;", true),
        ("1>2;", false),
        ("1>=2;", false),
        ("1==2;", false),
        ("1==1;", true),
        ("1==2-1;", true),
        ("\"asdf\"==1;", false),
        ("\"asdf\"==\"asdf\";", true),
        ("!(\"asdf\"==\"asdf\");", false),
        ("!!(\"asdf\"==\"asdf\");", true),
    ] {
        let tokens = scanner::scan_tokens(&source.to_string()).unwrap();
        let tree = parser::parse(tokens).unwrap();
        let mut interp = Interpreter::new();
        interp.run(&tree).unwrap();
    }
}

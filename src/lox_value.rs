use crate::{error::Error, lox_object::LoxObject};

#[derive(Debug)]
pub enum LoxValue {
    Number(f64),
    Bool(bool),
    String(String),
    Object(LoxObject),
    Null,
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

    pub fn negative(value: LoxValue) -> Result<LoxValue, Error> {
        match value {
            LoxValue::Number(value) => Ok(LoxValue::Number(-value)),
            _ => Err(Error::InternalRuntimeError {
                message: format!("Cannot negate: {:?}", value),
            }),
        }
    }
}

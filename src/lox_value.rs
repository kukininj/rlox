use crate::lox_object::LoxObject;

#[derive(Debug)]
pub enum LoxValue {
    Number(f64),
    Bool(bool),
    String(String),
    Object(LoxObject),
    Null,
}

impl LoxValue {
    pub fn add(left: LoxValue, right: LoxValue) -> LoxValue {
        match (left, right) {
            (LoxValue::Number(l), LoxValue::Number(r)) => LoxValue::Number(l + r),
            (LoxValue::String(l), LoxValue::String(r)) => LoxValue::String(format!("{}{}", l, r)),
            _ => LoxValue::Null,
        }
    }

    pub fn subtract(left: LoxValue, right: LoxValue) -> LoxValue {
        match (left, right) {
            (LoxValue::Number(l), LoxValue::Number(r)) => LoxValue::Number(l - r),
            _ => LoxValue::Null,
        }
    }

    pub fn multiply(left: LoxValue, right: LoxValue) -> LoxValue {
        match (left, right) {
            (LoxValue::Number(l), LoxValue::Number(r)) => LoxValue::Number(l * r),
            _ => LoxValue::Null,
        }
    }

    pub fn divide(left: LoxValue, right: LoxValue) -> LoxValue {
        match (left, right) {
            (LoxValue::Number(l), LoxValue::Number(r)) => LoxValue::Number(l / r),
            _ => LoxValue::Null,
        }
    }

    pub fn negative(value: LoxValue) -> LoxValue {
        match value {
            LoxValue::Number(value) => LoxValue::Number(-value),
            _ => LoxValue::Null,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

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

#[derive(Debug)]
pub struct LoxObject(Rc<RefCell<LoxObjectData>>);

#[derive(Debug)]
struct LoxObjectData {
    class: String,
}

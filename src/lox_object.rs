use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct LoxObject(Rc<RefCell<LoxObjectData>>);

#[derive(Debug)]
struct LoxObjectData {
    class: String,
}

impl LoxObject {
    pub fn to_string(&self) -> String {
        todo!()
    }
}

impl PartialEq for LoxObject {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

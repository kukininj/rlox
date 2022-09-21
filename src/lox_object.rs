use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug)]
pub struct LoxObject(Rc<RefCell<LoxObjectData>>);

#[derive(Debug)]
struct LoxObjectData {
    class: String,
}

impl PartialEq for LoxObject {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

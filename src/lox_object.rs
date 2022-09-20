use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug)]
pub struct LoxObject(Rc<RefCell<LoxObjectData>>);

#[derive(Debug)]
struct LoxObjectData {
    class: String,
}

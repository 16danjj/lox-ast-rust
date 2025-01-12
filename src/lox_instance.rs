use crate::lox_class::*;
use std::rc::Rc;


#[derive(Debug, Clone, PartialEq)]
pub struct LoxInstance {
    pub klass: Rc<LoxClass>
}

impl LoxInstance {
    pub fn new(klass: Rc<LoxClass>) -> Self {
        Self { klass: Rc::clone(&klass) }
    }
}
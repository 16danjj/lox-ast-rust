use std::rc::Rc;

use crate::callable::*;
use crate::interpreter::*;
use crate::object::*;
use crate::error::*;
use crate::lox_instance::*;


#[derive(Debug, Clone, PartialEq)]
pub struct LoxClass{
    name: String
}

impl LoxClass {
    pub fn new(name: &String) -> Self {
        Self { name: name.clone()}
    }

    pub fn instantiate(&self, interpreter: &Interpreter, arguments: Vec<Object>, klass: Rc<LoxClass>) -> Result<Object, LoxResult>{
        Ok(Object::Instance(LoxInstance::new(klass)))
    }
}
/* 
impl std::string::ToString for LoxClass { 
    fn to_string(&self) -> String {
        self.name.clone()
    }
}*/

impl LoxCallable for LoxClass{
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {        
        Err(LoxResult::SystemError { message: "tried to call a class ".to_string() })
    }

    fn arity(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        self.name.clone()
    }
}
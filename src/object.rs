use std::cmp::*;
use std::fmt;
use crate::interpreter::*;
use crate::LoxResult;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Num(f64),
    Str(String),
    Bool(bool),
    Func(Callable),
    Nil,
    ArithmeticError,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "{x}"),
            Object::Bool(x) => {
                if *x {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            },
            Object::Func(_) => write!(f, "<func>"),
            Object::Nil => write!(f, "nil"),
            Object::ArithmeticError => panic!("Should not be trying to print this"),
        }
    }
}

//------

#[derive(Debug, Clone, PartialEq)]
pub struct Callable;

impl  Callable {
    pub fn call(&self, terp: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult>{
        Ok(Object::Nil)
    }
}

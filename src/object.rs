use crate::callable::*;
use crate::lox_class::*;
use crate::lox_instance::*;
use std::cmp::*;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Num(f64),
    Str(String),
    Bool(bool),
    Func(Callable),
    Class(Rc<LoxClass>),
    Instance(LoxInstance),
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
            }
            Object::Func(_) => write!(f, "<Func>"),
            Object::Class(c) => write!(f, "<Class {}>", c.to_string()),
            Object::Instance(i) => write!(f, "{}", i.to_string()),
            Object::Nil => write!(f, "nil"),
            Object::ArithmeticError => panic!("Should not be trying to print this"),
        }
    }
}

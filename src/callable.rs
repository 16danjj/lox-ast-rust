use crate::interpreter::*;
use crate::object::*;
use crate::LoxResult;
use std::fmt::{self, Debug};

use std::rc::Rc;

#[derive(Clone)]
pub struct Callable {
    pub func: Rc<dyn LoxCallable>,
}

impl Debug for Callable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<callable>")
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func)
    }
}

pub trait LoxCallable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult>;
    fn arity(&self) -> usize;
}

impl LoxCallable for Callable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {
        self.func.call(interpreter, arguments) // Check !
    }

    fn arity(&self) -> usize {
        self.func.arity()
    }
}

use crate::callable::*;
use crate::environment::Environment;
use crate::error::*;
use crate::interpreter::*;
use crate::object::*;
use crate::stmt::*;
use crate::token::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct LoxFunction {
    name: Token,
    params: Rc<Vec<Token>>,
    body: Rc<Vec<Stmt>>,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(declaration: &FunctionStmt, closure: &Rc<RefCell<Environment>>) -> Self {
        Self {
            name: declaration.name.dup(),
            params: Rc::clone(&declaration.params),
            body: Rc::clone(&declaration.body),
            closure: Rc::clone(&closure),
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {
        let mut e = Environment::new_with_enclosing(Rc::clone(&self.closure));

        for (param, arg) in self.params.iter().zip(arguments.iter()) {
            e.define(&param.as_string(), arg.clone());
        }

        match interpreter.execute_block(&self.body, e) {
            Err(LoxResult::ReturnValue { value }) => Ok(value),
            Err(e) => return Err(e),
            Ok(_) => Ok(Object::Nil),
        }
    }

    fn arity(&self) -> usize {
        self.params.len()
    }

    fn to_string(&self) -> String {
        self.name.as_string()
    }
}

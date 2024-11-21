use crate::callable::*;
use crate::error::*;
use crate::interpreter::*;
use crate::object::*;
use std::time::SystemTime;

pub struct NativeClock;

impl LoxCallable for NativeClock {
    fn call(
        &self,
        _interpreter: &Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, LoxResult> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => return Ok(Object::Num(n.as_millis() as f64)),
            Err(e) => Err(LoxResult::SystemError { message: format!("Clock returned invalid duration : {:?}", e.duration() )  })
        }
    }

    fn arity(&self) -> usize {
        0
    }
}

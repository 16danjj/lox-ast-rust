use crate::callable::*;
use crate::error::*;
use crate::interpreter::*;
use crate::lox_class::*;
use crate::object::*;
use std::fmt;
use std::rc::Rc;
use std::time::SystemTime;
pub struct NativeClock;

impl fmt::Display for NativeClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<NativeClock>")
    }
}

impl fmt::Debug for NativeClock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Clone for NativeClock {
    fn clone(&self) -> Self {
        Self {}
    }
}
impl PartialEq for NativeClock {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl LoxCallable for NativeClock {
    fn call(
        &self,
        _interpreter: &Interpreter,
        _arguments: Vec<Object>,
        _klass: Option<Rc<LoxClass>>,
    ) -> Result<Object, LoxResult> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => return Ok(Object::Num(n.as_millis() as f64)),
            Err(e) => Err(LoxResult::SystemError {
                message: format!("Clock returned invalid duration : {:?}", e.duration()),
            }),
        }
    }

    fn arity(&self) -> usize {
        0
    }
}

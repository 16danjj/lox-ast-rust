use crate::callable::*;
use crate::environment::Environment;
use crate::error::*;
use crate::expr::*;
use crate::lox_function::*;
use crate::native_functions::*;
use crate::object::*;
use crate::stmt::*;
use crate::token_type::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    environment: RefCell<Rc<RefCell<Environment>>>,
    nest: RefCell<usize>,
}

impl StmtVisitor<()> for Interpreter {
    fn visit_return_stmt(&self, stmt: &ReturnStmt) -> Result<(), LoxResult> {
        if let Some(value) = &stmt.value {
            Err(LoxResult::return_value(self.evaluate(value)?))
        } else {
            Err(LoxResult::return_value(Object::Nil))
        }
    }
    fn visit_function_stmt(&self, stmt: &FunctionStmt) -> Result<(), LoxResult> {
        let function = LoxFunction::new(stmt, self.environment.borrow().deref());
        self.environment.borrow().borrow_mut().define(
            &stmt.name.as_string(),
            Object::Func(Callable {
                func: Rc::new(function),
            }),
        );
        Ok(())
    }

    fn visit_break_stmt(&self, stmt: &BreakStmt) -> Result<(), LoxResult> {
        if *self.nest.borrow() == 0 {
            Err(LoxResult::runtime_error(
                &stmt.token,
                "Break outside of while/for loop.",
            ))
        } else {
            Err(LoxResult::Break)
        }
    }
    fn visit_while_stmt(&self, stmt: &WhileStmt) -> Result<(), LoxResult> {
        *self.nest.borrow_mut() += 1;
        while self.is_truthy(&self.evaluate(&stmt.condition)?) {
            match self.execute(&stmt.body) {
                Err(LoxResult::Break) => break,
                Err(e) => return Err(e),
                Ok(_) => {}
            }
        }
        *self.nest.borrow_mut() -= 1;
        Ok(())
    }
    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<(), LoxResult> {
        if self.is_truthy(&self.evaluate(&stmt.condition)?) {
            self.execute(&stmt.then_branch)
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }

    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), LoxResult> {
        let e = Environment::new_with_enclosing(self.environment.borrow().clone());

        self.execute_block(&stmt.statements, e)
    }

    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), LoxResult> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), LoxResult> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{value}");
        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), LoxResult> {
        let value = if let Some(initializer) = &stmt.initializer {
            self.evaluate(initializer)?
        } else {
            Object::Nil
        };

        self.environment
            .borrow()
            .borrow_mut()
            .define(stmt.name.as_string(), value);
        Ok(())
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_call_expr(&self, expr: &CallExpr) -> Result<Object, LoxResult> {
        let callee = self.evaluate(&expr.callee)?;
        let mut arguments = Vec::new();
        for argument in &expr.arguments {
            arguments.push(self.evaluate(argument)?);
        }

        if let Object::Func(function) = callee {
            if arguments.len() != function.func.arity() {
                return Err(LoxResult::runtime_error(
                    &expr.paren,
                    &format!(
                        "Expected {} arguments but got {}.",
                        function.func.arity(),
                        arguments.len()
                    ),
                ));
            }
            function.func.call(self, arguments) // Check !
        } else {
            Err(LoxResult::runtime_error(
                &expr.paren,
                "Can only call functions and classes",
            ))
        }
    }
    fn visit_logical_expr(&self, expr: &LogicalExpr) -> Result<Object, LoxResult> {
        let left = self.evaluate(&expr.left)?;

        if expr.operator.is(TokenType::Or) {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !self.is_truthy(&left) {
                return Ok(left);
            }
        }

        self.evaluate(&expr.right)
    }
    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<Object, LoxResult> {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow()
            .borrow_mut()
            .assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxResult> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxResult> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        let op = expr.operator.token_type();

        let result = match (left, right) {
            (Object::Num(left), Object::Num(right)) => match op {
                TokenType::Minus => Object::Num(left - right),
                TokenType::Slash => Object::Num(left / right),
                TokenType::Star => Object::Num(left * right),
                TokenType::Plus => Object::Num(left + right),
                TokenType::Greater => Object::Bool(left > right),
                TokenType::GreaterEqual => Object::Bool(left >= right),
                TokenType::Less => Object::Bool(left < right),
                TokenType::LessEqual => Object::Bool(left <= right),
                TokenType::BangEqual => Object::Bool(left != right),
                TokenType::Equals => Object::Bool(left == right),
                _ => Object::ArithmeticError,
            },
            (Object::Str(left), Object::Num(right)) => match op {
                TokenType::Plus => Object::Str(format!("{left}{right}")),
                _ => Object::ArithmeticError,
            },
            (Object::Num(left), Object::Str(right)) => match op {
                TokenType::Plus => Object::Str(format!("{left}{right}")),
                _ => Object::ArithmeticError,
            },
            (Object::Str(left), Object::Str(right)) => match op {
                TokenType::Plus => Object::Str(format!("{left}{right}")),
                TokenType::BangEqual => Object::Bool(left != right),
                TokenType::Equals => Object::Bool(left == right),
                _ => Object::ArithmeticError,
            },

            (Object::Bool(left), Object::Bool(right)) => match op {
                TokenType::BangEqual => Object::Bool(left != right),
                TokenType::Equals => Object::Bool(left == right),
                _ => Object::ArithmeticError,
            },
            (Object::Nil, Object::Nil) => match op {
                TokenType::BangEqual => Object::Bool(false),
                TokenType::Equals => Object::Bool(true),
                _ => Object::ArithmeticError,
            },
            (Object::Nil, _) => match op {
                TokenType::BangEqual => Object::Bool(true),
                TokenType::Equals => Object::Bool(false),
                _ => Object::ArithmeticError,
            },
            _ => Object::ArithmeticError,
        };

        if result == Object::ArithmeticError {
            Err(LoxResult::runtime_error(
                &expr.operator,
                "Illegal expression",
            ))
        } else {
            Ok(result)
        }
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxResult> {
        self.evaluate(&expr.expression)
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxResult> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type() {
            TokenType::Minus => match right {
                Object::Num(n) => Ok(Object::Num(-n)),
                _ => Ok(Object::Nil),
            },
            TokenType::Bang => Ok(Object::Bool(!self.is_truthy(&right))),
            _ => Err(LoxResult::error(expr.operator.line, "Unreachable")),
        }
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Object, LoxResult> {
        self.environment.borrow().borrow().get(&expr.name)
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let globals = Rc::new(RefCell::new(Environment::new()));
        globals.borrow_mut().define(
            "clock",
            Object::Func(Callable {
                func: Rc::new(NativeClock {}),
            }),
        );

        Interpreter {
            globals: Rc::clone(&globals),
            environment: RefCell::new(Rc::clone(&globals)),
            nest: RefCell::new(0),
        }
    }
    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxResult> {
        expr.accept(self)
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), LoxResult> {
        stmt.accept(self)
    }

    pub fn execute_block(
        &self,
        statements: &[Stmt],
        environment: Environment,
    ) -> Result<(), LoxResult> {
        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));

        let result = statements
            .iter()
            .try_for_each(|statement| self.execute(statement));

        self.environment.replace(previous);

        result
    }

    // anything that is not Nil or False is true
    fn is_truthy(&self, object: &Object) -> bool {
        !matches!(object, Object::Bool(false) | Object::Nil)
    }

    pub fn interpret(&self, statements: &[Stmt]) -> bool {
        let mut success = true;
        *self.nest.borrow_mut() = 0;

        for statement in statements {
            if self.execute(statement).is_err() {
                success = false;
                break;
            }
        }
        success
    }

    pub fn print_environment(&self) {
        println!("{:?}", self.environment);
    }
}



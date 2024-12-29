use crate::error::*;
use crate::token::*;
use crate::object::*;
use std::rc::Rc;

pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Call(CallExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Logical(LogicalExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
}

impl Expr {
    pub fn accept<T>(&self, expr_visitor: &dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        match self {
            Expr::Assign(v) => expr_visitor.visit_assign_expr(self, &v),
            Expr::Binary(v) => expr_visitor.visit_binary_expr(self, &v),
            Expr::Call(v) => expr_visitor.visit_call_expr(self, &v),
            Expr::Grouping(v) => expr_visitor.visit_grouping_expr(self, &v),
            Expr::Literal(v) => expr_visitor.visit_literal_expr(self, &v),
            Expr::Logical(v) => expr_visitor.visit_logical_expr(self, &v),
            Expr::Unary(v) => expr_visitor.visit_unary_expr(self, &v),
            Expr::Variable(v) => expr_visitor.visit_variable_expr(self, &v),
        }
    }
}

pub struct AssignExpr {
    pub name: Token,
    pub value: Rc<Expr>,
}

pub struct BinaryExpr {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
}

pub struct CallExpr {
    pub callee: Rc<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

pub struct GroupingExpr {
    pub expression: Rc<Expr>,
}

pub struct LiteralExpr {
    pub value: Option<Object>,
}

pub struct LogicalExpr {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Rc<Expr>,
}

pub struct VariableExpr {
    pub name: Token,
}

pub trait ExprVisitor<T> {
    fn visit_assign_expr(&self, wrapper: &Expr, expr: &AssignExpr) -> Result<T, LoxResult>;
    fn visit_binary_expr(&self, wrapper: &Expr, expr: &BinaryExpr) -> Result<T, LoxResult>;
    fn visit_call_expr(&self, wrapper: &Expr, expr: &CallExpr) -> Result<T, LoxResult>;
    fn visit_grouping_expr(&self, wrapper: &Expr, expr: &GroupingExpr) -> Result<T, LoxResult>;
    fn visit_literal_expr(&self, wrapper: &Expr, expr: &LiteralExpr) -> Result<T, LoxResult>;
    fn visit_logical_expr(&self, wrapper: &Expr, expr: &LogicalExpr) -> Result<T, LoxResult>;
    fn visit_unary_expr(&self, wrapper: &Expr, expr: &UnaryExpr) -> Result<T, LoxResult>;
    fn visit_variable_expr(&self, wrapper: &Expr, expr: &VariableExpr) -> Result<T, LoxResult>;
}


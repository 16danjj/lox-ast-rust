use crate::error::*;
use crate::expr::*;
use crate::token::*;
use std::rc::Rc;

pub enum Stmt {
    Block(BlockStmt),
    Break(BreakStmt),
    Expression(ExpressionStmt),
    Function(FunctionStmt),
    If(IfStmt),
    Print(PrintStmt),
    Return(ReturnStmt),
    Var(VarStmt),
    While(WhileStmt),
}

impl Stmt {
    pub fn accept<T>(&self, stmt_visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
        match self {
            Stmt::Block(v) => stmt_visitor.visit_block_stmt(self, &v),
            Stmt::Break(v) => stmt_visitor.visit_break_stmt(self, &v),
            Stmt::Expression(v) => stmt_visitor.visit_expression_stmt(self, &v),
            Stmt::Function(v) => stmt_visitor.visit_function_stmt(self, &v),
            Stmt::If(v) => stmt_visitor.visit_if_stmt(self, &v),
            Stmt::Print(v) => stmt_visitor.visit_print_stmt(self, &v),
            Stmt::Return(v) => stmt_visitor.visit_return_stmt(self, &v),
            Stmt::Var(v) => stmt_visitor.visit_var_stmt(self, &v),
            Stmt::While(v) => stmt_visitor.visit_while_stmt(self, &v),
        }
    }
}

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

pub struct BreakStmt {
    pub token: Token,
}

pub struct ExpressionStmt {
    pub expression: Rc<Expr>,
}

pub struct FunctionStmt {
    pub name: Token,
    pub params: Rc<Vec<Token>>,
    pub body: Rc<Vec<Stmt>>,
}

pub struct IfStmt {
    pub condition: Rc<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

pub struct PrintStmt {
    pub expression: Rc<Expr>,
}

pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Rc<Expr>>,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Rc<Expr>>,
}

pub struct WhileStmt {
    pub condition: Rc<Expr>,
    pub body: Box<Stmt>,
}

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&self, wrapper: &Stmt, stmt: &BlockStmt) -> Result<T, LoxResult>;
    fn visit_break_stmt(&self, wrapper: &Stmt, stmt: &BreakStmt) -> Result<T, LoxResult>;
    fn visit_expression_stmt(&self, wrapper: &Stmt, stmt: &ExpressionStmt) -> Result<T, LoxResult>;
    fn visit_function_stmt(&self, wrapper: &Stmt, stmt: &FunctionStmt) -> Result<T, LoxResult>;
    fn visit_if_stmt(&self, wrapper: &Stmt, stmt: &IfStmt) -> Result<T, LoxResult>;
    fn visit_print_stmt(&self, wrapper: &Stmt, stmt: &PrintStmt) -> Result<T, LoxResult>;
    fn visit_return_stmt(&self, wrapper: &Stmt, stmt: &ReturnStmt) -> Result<T, LoxResult>;
    fn visit_var_stmt(&self, wrapper: &Stmt, stmt: &VarStmt) -> Result<T, LoxResult>;
    fn visit_while_stmt(&self, wrapper: &Stmt, stmt: &WhileStmt) -> Result<T, LoxResult>;
}


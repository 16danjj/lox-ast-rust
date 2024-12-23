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
            Stmt::Block(v) => v.accept(stmt_visitor),
            Stmt::Break(v) => v.accept(stmt_visitor),
            Stmt::Expression(v) => v.accept(stmt_visitor),
            Stmt::Function(v) => v.accept(stmt_visitor),
            Stmt::If(v) => v.accept(stmt_visitor),
            Stmt::Print(v) => v.accept(stmt_visitor),
            Stmt::Return(v) => v.accept(stmt_visitor),
            Stmt::Var(v) => v.accept(stmt_visitor),
            Stmt::While(v) => v.accept(stmt_visitor),
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
    pub expression: Expr,
}

pub struct FunctionStmt {
    pub name: Token,
    pub params: Rc<Vec<Token>>,
    pub body: Rc<Vec<Stmt>>,
}

pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

pub struct PrintStmt {
    pub expression: Expr,
}

pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Expr>,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&self, expr: &BlockStmt) -> Result<T, LoxResult>;
    fn visit_break_stmt(&self, expr: &BreakStmt) -> Result<T, LoxResult>;
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> Result<T, LoxResult>;
    fn visit_function_stmt(&self, expr: &FunctionStmt) -> Result<T, LoxResult>;
    fn visit_if_stmt(&self, expr: &IfStmt) -> Result<T, LoxResult>;
    fn visit_print_stmt(&self, expr: &PrintStmt) -> Result<T, LoxResult>;
    fn visit_return_stmt(&self, expr: &ReturnStmt) -> Result<T, LoxResult>;
    fn visit_var_stmt(&self, expr: &VarStmt) -> Result<T, LoxResult>;
    fn visit_while_stmt(&self, expr: &WhileStmt) -> Result<T, LoxResult>;
}

impl BlockStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
      visitor.visit_block_stmt(self)
    }
}

impl BreakStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
      visitor.visit_break_stmt(self)
    }
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
      visitor.visit_expression_stmt(self)
    }
}

impl FunctionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
      visitor.visit_function_stmt(self)
    }
}

impl IfStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
      visitor.visit_if_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
      visitor.visit_print_stmt(self)
    }
}

impl ReturnStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
      visitor.visit_return_stmt(self)
    }
}

impl VarStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
      visitor.visit_var_stmt(self)
    }
}

impl WhileStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
      visitor.visit_while_stmt(self)
    }
}


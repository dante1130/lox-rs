use crate::{stmt_visitor::StmtVisitor, lexer::token::Token};

use super::expr::Expr;

pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Block(stmt) => visitor.visit_block_stmt(stmt),
            Stmt::Expression(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::Var(stmt) => visitor.visit_var_stmt(stmt),
        }
    }
}

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct PrintStmt {
    pub expression: Expr,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

impl BlockStmt {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Self { statements }
    }
}

impl ExpressionStmt {
    pub fn new(expression: Expr) -> Self {
        Self { expression }
    }
}

impl PrintStmt {
    pub fn new(expression: Expr) -> Self {
        Self { expression }
    }
}

impl VarStmt {
    pub fn new(name: Token, initializer: Option<Expr>) -> Self {
        Self { name, initializer }
    }
}

impl From<BlockStmt> for Stmt {
    fn from(statement: BlockStmt) -> Self {
        Stmt::Block(statement)
    }
}

impl From<ExpressionStmt> for Stmt {
    fn from(statement: ExpressionStmt) -> Self {
        Stmt::Expression(statement)
    }
}

impl From<PrintStmt> for Stmt {
    fn from(statement: PrintStmt) -> Self {
        Stmt::Print(statement)
    }
}

impl From<VarStmt> for Stmt {
    fn from(statement: VarStmt) -> Self {
        Stmt::Var(statement)
    }
}

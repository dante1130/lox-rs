use crate::ast::statement::{ExpressionStmt, PrintStmt, VarStmt, BlockStmt, IfStmt};

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> T;
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> T;
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> T;
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> T;
}

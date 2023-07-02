use crate::ast::expr::{
    AssignExpr, BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr, VariableExpr, LogicalExpr,
};

pub trait ExprVisitor<T> {
    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> T;
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> T;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> T;
    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> T;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> T;
    fn visit_variable_expr(&self, expr: &VariableExpr) -> T;
}

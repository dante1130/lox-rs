use super::expr::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr};

pub trait Visitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> T;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> T;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> T;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> T;
}


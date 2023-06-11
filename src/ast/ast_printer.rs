use super::{
    expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    visitor::Visitor,
};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> String {
        format!(
            "({} {} {})",
            expr.operator.lexeme,
            expr.left.accept(self),
            expr.right.accept(self)
        )
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> String {
        format!("(group {})", expr.expression.accept(self))
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> String {
        match &expr.value {
            Some(literal) => literal.to_string(),
            None => "nil".to_string(),
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> String {
        format!("({} {})", expr.operator.lexeme, expr.right.accept(self))
    }
}

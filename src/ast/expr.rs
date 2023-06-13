use crate::{ast::visitor::Visitor, lexer::token::Token, value::Value};

#[derive(Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
        }
    }
}

#[derive(Clone)]
pub struct BinaryExpr {
    pub operator: Token,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Clone)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

#[derive(Clone)]
pub struct LiteralExpr {
    pub value: Option<Value>,
}

#[derive(Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(operator: Token, left: Expr, right: Expr) -> Self {
        Self {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl GroupingExpr {
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

impl LiteralExpr {
    pub fn new(value: Value) -> Self {
        Self { value: Some(value) }
    }
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}

impl From<BinaryExpr> for Expr {
    fn from(expr: BinaryExpr) -> Self {
        Expr::Binary(expr)
    }
}

impl From<GroupingExpr> for Expr {
    fn from(expr: GroupingExpr) -> Self {
        Expr::Grouping(expr)
    }
}

impl From<LiteralExpr> for Expr {
    fn from(expr: LiteralExpr) -> Self {
        Expr::Literal(expr)
    }
}

impl From<UnaryExpr> for Expr {
    fn from(expr: UnaryExpr) -> Self {
        Expr::Unary(expr)
    }
}


use crate::{expr_visitor::ExprVisitor, lexer::token::Token, value::Value};

#[derive(Clone)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Logical(LogicalExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Assign(expr) => visitor.visit_assign_expr(expr),
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Logical(expr) => visitor.visit_logical_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
            Expr::Variable(expr) => visitor.visit_variable_expr(expr),
        }
    }
}

#[derive(Clone)]
pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,
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
pub struct LogicalExpr {
    pub operator: Token,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Clone)]
pub struct VariableExpr {
    pub name: Token,
}

impl AssignExpr {
    pub fn new(name: Token, value: Expr) -> Self {
        Self {
            name,
            value: Box::new(value),
        }
    }
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

impl LogicalExpr {
    pub fn new(operator: Token, left: Expr, right: Expr) -> Self {
        Self {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
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

impl VariableExpr {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}

impl From<AssignExpr> for Expr {
    fn from(expr: AssignExpr) -> Self {
        Expr::Assign(expr)
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

impl From<LogicalExpr> for Expr {
    fn from(expr: LogicalExpr) -> Self {
        Expr::Logical(expr)
    }
}

impl From<UnaryExpr> for Expr {
    fn from(expr: UnaryExpr) -> Self {
        Expr::Unary(expr)
    }
}

impl From<VariableExpr> for Expr {
    fn from(expr: VariableExpr) -> Self {
        Expr::Variable(expr)
    }
}

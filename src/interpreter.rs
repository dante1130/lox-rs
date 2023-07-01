use crate::{
    ast::{
        expr::{AssignExpr, BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, VariableExpr},
        statement::{ExpressionStmt, PrintStmt, Stmt, VarStmt},
    },
    environment::Environment,
    error::RuntimeError,
    expr_visitor::ExprVisitor,
    lexer::token_type::TokenType,
    stmt_visitor::StmtVisitor,
    value::Value,
};

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for stmt in statements {
            self.execute(stmt);
        }
    }

    fn execute(&mut self, stmt: &Stmt) {
        stmt.accept(self);
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Nil => false,
        Value::Bool(b) => *b,
        _ => true,
    }
}

fn is_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Nil, Value::Nil) => true,
        (Value::Bool(l), Value::Bool(r)) => l == r,
        (Value::Number(l), Value::Number(r)) => l == r,
        (Value::String(l), Value::String(r)) => l == r,
        _ => false,
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) {
        let _ = self.evaluate(&stmt.expression);
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) {
        if let Ok(value) = self.evaluate(&stmt.expression) {
            println!("{}", value)
        }
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) {
        let value = match &stmt.initializer {
            Some(expr) => self.evaluate(expr).unwrap(),
            None => Value::Nil,
        };

        self.environment.define(stmt.name.lexeme.clone(), value);
    }
}

impl ExprVisitor<Result<Value, RuntimeError>> for Interpreter {
    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<Value, RuntimeError> {
        let value = self.evaluate(&expr.value).unwrap();
        match self.environment.assign(expr.name.clone(), value.clone()) {
            Ok(_) => Ok(value),
            Err(err) => Err(err),
        }
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<Value, RuntimeError> {
        let left = self.evaluate(&expr.left).unwrap();
        let right = self.evaluate(&expr.right).unwrap();

        match expr.operator.token_type {
            TokenType::Greater => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
                _ => Err(RuntimeError::new(
                    expr.operator.clone(),
                    String::from("Operands must be numbers."),
                )),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
                _ => Err(RuntimeError::new(
                    expr.operator.clone(),
                    String::from("Operands must be numbers."),
                )),
            },
            TokenType::Less => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
                _ => Err(RuntimeError::new(
                    expr.operator.clone(),
                    String::from("Operands must be numbers."),
                )),
            },
            TokenType::LessEqual => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
                _ => Err(RuntimeError::new(
                    expr.operator.clone(),
                    String::from("Operands must be numbers."),
                )),
            },
            TokenType::BangEqual => Ok(Value::Bool(!is_equal(&left, &right))),
            TokenType::EqualEqual => Ok(Value::Bool(is_equal(&left, &right))),
            TokenType::Minus => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                _ => Err(RuntimeError::new(
                    expr.operator.clone(),
                    String::from("Operands must be numbers."),
                )),
            },
            TokenType::Plus => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                _ => Err(RuntimeError::new(
                    expr.operator.clone(),
                    String::from("Operands must be two numbers or two strings."),
                )),
            },
            TokenType::Slash => match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    if r == 0.0 {
                        return Err(RuntimeError::new(
                            expr.operator.clone(),
                            String::from("Division by zero."),
                        ));
                    }

                    Ok(Value::Number(l / r))
                }
                _ => Err(RuntimeError::new(
                    expr.operator.clone(),
                    String::from("Operands must be numbers."),
                )),
            },
            TokenType::Star => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                _ => Err(RuntimeError::new(
                    expr.operator.clone(),
                    String::from("Operands must be numbers."),
                )),
            },
            _ => Ok(Value::Nil),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<Value, RuntimeError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Value, RuntimeError> {
        Ok(expr.value.clone().unwrap_or(Value::Nil))
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<Value, RuntimeError> {
        let right = self.evaluate(&expr.right).unwrap();

        match expr.operator.token_type {
            TokenType::Bang => Ok(Value::Bool(!is_truthy(&right))),
            TokenType::Minus => match right {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(RuntimeError::new(
                    expr.operator.clone(),
                    String::from("Operand must be a number."),
                )),
            },
            _ => Ok(Value::Nil),
        }
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Value, RuntimeError> {
        self.environment.get(expr.name.clone())
    }
}

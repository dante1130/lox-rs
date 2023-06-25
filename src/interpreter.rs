use crate::{
    ast::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    error::RuntimeError,
    lexer::token_type::TokenType,
    value::Value,
    visitor::Visitor,
};

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&self, expr: &Expr) {
        match self.evaluate(expr) {
            Ok(value) => println!("{}", value),
            Err(error) => println!("{}", error),
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<Value, RuntimeError> {
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

impl Visitor<Result<Value, RuntimeError>> for Interpreter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Value, RuntimeError> {
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

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Value, RuntimeError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Value, RuntimeError> {
        Ok(expr.value.clone().unwrap_or(Value::Nil))
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Value, RuntimeError> {
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
}

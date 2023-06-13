use crate::{
    ast::expr::{GroupingExpr, LiteralExpr},
    error,
    lexer::{token::Token, token_type::TokenType},
    value::Value,
};

use super::expr::{BinaryExpr, Expr, UnaryExpr};

#[derive(Clone)]
pub struct ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression().ok()
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr_result = self.comparison();
        let expr = match expr_result.clone() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = match self.comparison() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            expr_result = Ok(Expr::Binary(BinaryExpr::new(operator, expr.clone(), right)));
        }

        expr_result
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr_result = self.term();
        let expr = match expr_result.clone() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = match self.term() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            expr_result = Ok(Expr::Binary(BinaryExpr::new(operator, expr.clone(), right)));
        }

        expr_result
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr_result = self.factor();
        let expr = match expr_result.clone() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = match self.factor() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            expr_result = Ok(Expr::Binary(BinaryExpr::new(operator, expr.clone(), right)));
        }

        expr_result
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr_result = self.unary();
        let expr = match expr_result.clone() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = match self.unary() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            expr_result = Ok(Expr::Binary(BinaryExpr::new(operator, expr.clone(), right)));
        }

        expr_result
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = match self.unary() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            return Ok(Expr::Unary(UnaryExpr::new(operator, right)));
        }

        match self.primary() {
            Ok(expr) => Ok(expr),
            Err(err) => Err(err),
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr::new(Value::Bool(false))));
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr::new(Value::Bool(true))));
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr::new(Value::Nil)));
        }

        if self.match_token(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr::new(
                self.previous().literal.clone().unwrap(),
            )));
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = match self.expression() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            match self.consume(TokenType::RightParen, "Expected ')' after expression.") {
                Ok(_) => {}
                Err(err) => return Err(err),
            }
            return Ok(Expr::Grouping(GroupingExpr::new(expr)));
        }

        Err(self.error(self.peek(), "Expected expression."))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), message))
    }

    fn error(&self, token: &Token, message: &str) -> ParseError {
        error::error_token(token, message);
        ParseError {}
    }

    fn match_token(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}

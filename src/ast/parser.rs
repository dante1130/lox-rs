use crate::{
    ast::expr::{GroupingExpr, LiteralExpr},
    error::{self, ParseError},
    lexer::{token::Token, token_type::TokenType},
    value::Value,
};

use super::{
    expr::{AssignExpr, BinaryExpr, Expr, UnaryExpr, VariableExpr},
    statement::{BlockStmt, ExpressionStmt, PrintStmt, Stmt, VarStmt},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if let Ok(stmt) = self.declaration() {
                statements.push(stmt)
            }
        }

        statements
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        }

        if self.match_token(&[TokenType::LeftBrace]) {
            match self.block() {
                Ok(block) => return Ok(Stmt::Block(BlockStmt::new(block))),
                Err(e) => return Err(e),
            }
        }

        self.expression_statement()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            match self.declaration() {
                Ok(declaration) => statements.push(declaration),
                Err(e) => {
                    return Err(e);
                }
            }
        }

        match self.consume(TokenType::RightBrace, "Expect '}' after block.") {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::Var]) {
            return self.var_declaration();
        }

        match self.statement() {
            Ok(stmt) => Ok(stmt),
            Err(err) => {
                self.synchronize();
                Err(err)
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = match self.consume(TokenType::Identifier, "Expect variable name.") {
            Ok(token) => token.clone(),
            Err(err) => return Err(err),
        };

        if self.match_token(&[TokenType::Semicolon]) {
            return Err(self.error(&name, "Variable declaration must be initialized."));
        }

        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(match self.expression() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            })
        } else {
            None
        };

        match self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        ) {
            Ok(_) => Ok(Stmt::Var(VarStmt::new(name, initializer))),
            Err(err) => Err(err),
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = match self.expression() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };

        match self.consume(TokenType::Semicolon, "Expect ';' after value.") {
            Ok(_) => Ok(Stmt::Print(PrintStmt::new(expr))),
            Err(err) => Err(err),
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = match self.expression() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };

        match self.consume(TokenType::Semicolon, "Expect ';' after expression.") {
            Ok(_) => Ok(Stmt::Expression(ExpressionStmt::new(expr))),
            Err(err) => Err(err),
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr_result = self.equality();
        let expr = match expr_result.clone() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };

        if self.match_token(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = match self.assignment() {
                Ok(value) => value,
                Err(err) => return Err(err),
            };

            if let Expr::Variable(var) = expr {
                return Ok(Expr::Assign(AssignExpr::new(var.name, value)));
            }

            error::error_token(&equals, "Invalid assignment target.")
        }

        expr_result
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
            let value = match self.previous().literal.clone() {
                Some(value) => value,
                None => return Err(self.error(self.peek(), "Expected literal value.")),
            };

            return Ok(Expr::Literal(LiteralExpr::new(value)));
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

        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(VariableExpr::new(self.previous().clone())));
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

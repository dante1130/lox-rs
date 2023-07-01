use crate::lexer::{token::Token, token_type::TokenType};

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

#[derive(Debug, Clone)]
pub struct ParseError;

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: String) -> RuntimeError {
        RuntimeError { token, message }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe { HAD_RUNTIME_ERROR = true };

        write!(f, "[line {}] Error", self.token.line)?;
        match &self.token.token_type {
            TokenType::Eof => write!(f, " at end")?,
            _ => write!(f, " at '{}'", self.token.lexeme)?,
        }
        write!(f, ": {}", self.message)
    }
}

impl std::error::Error for RuntimeError {}

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn error_token(token: &Token, message: &str) {
    match token.token_type {
        TokenType::Eof => report(token.line, "at end", message),
        _ => report(token.line, &format!("at '{}'", token.lexeme), message),
    }
}

pub fn had_error() -> bool {
    unsafe { HAD_ERROR }
}

pub fn had_runtime_error() -> bool {
    unsafe { HAD_RUNTIME_ERROR }
}

pub fn reset_error() {
    unsafe { HAD_ERROR = false };
}

fn report(line: usize, location: &str, message: &str) {
    unsafe { HAD_ERROR = true };
    println!("[line {}] Error {}: {}", line, location, message);
}

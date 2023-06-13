use crate::lexer::{token::Token, token_type::TokenType};

static mut HAD_ERROR: bool = false;

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

pub fn reset_error() {
    unsafe { HAD_ERROR = false };
}

fn report(line: usize, location: &str, message: &str) {
    println!("[line {}] Error {}: {}", line, location, message);
    unsafe { HAD_ERROR = true };
}

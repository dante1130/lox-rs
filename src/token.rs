use std::any::Any;

use crate::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Box<dyn Any>>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Box<dyn Any>>,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let literal = match &self.literal {
            Some(literal) => {
                if literal.is::<String>() {
                    literal.downcast_ref::<String>().unwrap().to_string()
                } else if literal.is::<f64>() {
                    format!("{}", literal.downcast_ref::<f64>().unwrap())
                } else {
                    String::from("")
                }
            }
            None => String::from(""),
        };
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, literal)
    }
}

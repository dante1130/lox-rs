use crate::{error::RuntimeError, lexer::token::Token, value::Value};
use std::collections::{hash_map::Entry, HashMap};

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, token: Token, value: Value) -> Result<(), RuntimeError> {
        if let Entry::Occupied(mut e) = self.values.entry(token.lexeme.clone()) {
            e.insert(value);
            return Ok(());
        }

        Err(RuntimeError::new(
            token.clone(),
            format!("Undefined variable '{}'.", token.lexeme),
        ))
    }

    pub fn get(&self, token: Token) -> Result<Value, RuntimeError> {
        match self.values.get(&token.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(RuntimeError::new(
                token.clone(),
                format!("Undefined variable '{}'.", token.lexeme),
            )),
        }
    }
}

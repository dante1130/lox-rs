use crate::{error::RuntimeError, lexer::token::Token, value::Value};
use std::collections::{hash_map::Entry, HashMap};

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn from(enclosing: Environment) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
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

        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.assign(token, value);
        }

        Err(RuntimeError::new(
            token.clone(),
            format!("Undefined variable '{}'.", token.lexeme),
        ))
    }

    pub fn get(&self, token: Token) -> Result<Value, RuntimeError> {
        match self.values.get(&token.lexeme) {
            Some(value) => Ok(value.clone()),
            None => {
                if let Some(enclosing) = &self.enclosing {
                    return enclosing.get(token);
                };

                Err(RuntimeError::new(
                    token.clone(),
                    format!("Undefined variable '{}'.", token.lexeme),
                ))
            }
        }
    }
}

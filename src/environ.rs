use std::collections::HashMap;

use crate::token::{Object, Token};

pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }
    pub fn get(&mut self, token: &Token) -> Object {
        if self.values.contains_key(&token.lexeme) {
            return self.values.get(&token.lexeme).unwrap().clone();
        }

        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.get(token);
        }
        todo!()
    }
    pub fn assign(&mut self, name: &String, object: Object) {
        if self.values.contains_key(name) {
            self.values.insert(name.clone(), object);
            return;
        }
        if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, object);
            return;
        }
        todo!();
    }

    pub fn define(&mut self, name: &String, object: Object) {
        self.values.insert(name.clone(), object);
    }
}

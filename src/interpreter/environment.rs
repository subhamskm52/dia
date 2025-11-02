use std::collections::HashMap;
use crate::parser::expr::LiteralValue;

#[derive(Debug)]
pub struct Environment{
    scopes : Vec<HashMap<String, LiteralValue>>,
}

impl Environment{
    pub fn new() -> Environment {
        Environment{ scopes : vec![HashMap::new()] } // global scope
    }

    pub fn get(&self, name: &str) -> Option<LiteralValue> {

        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }
    pub fn set(&mut self, name: String, value: LiteralValue) {
        self.scopes.last_mut().unwrap().insert(name, value);
    }

    pub fn update(&mut self, name: String, value: LiteralValue) {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(&name) {
                scope.insert(name.clone(), value.clone());
            }
        }
    }
    pub fn contains_in_current_scope(&mut self, name: &str) -> bool {
        if let Some(scope) = self.scopes.last() {
            scope.contains_key(name)
        } else {
            false
        }    
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}
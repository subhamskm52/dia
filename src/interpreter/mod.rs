mod expr_interpreter;

use std::collections::HashMap;
use crate::parser::expr::LiteralValue;
use crate::parser::Parser;
use crate::parser::stmt::Stmt;
use crate::scanner::token::Token;

pub struct Interpreter{
    variables: Vec<Token>,
    env: HashMap<String, LiteralValue>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter{variables: vec![], env:HashMap::new()}
    }

    pub fn is_truthy(&self, value: &LiteralValue) -> bool {
        match value {
            LiteralValue::Number(n) => {n != &0.0}
            LiteralValue::String(_) => {true}
            LiteralValue::Bool(n) => {n == &true}
            LiteralValue::Nil => {false}
        }
    }
    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.evaluate(stmt);
        }
        println!("{:?}", self.env)
    }

    pub fn evaluate(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expression(expr) => {
                let val = self.evaluate_expression(expr);
            }
            _ => {}
        }
    }
}
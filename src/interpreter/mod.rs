mod expr_interpreter;
mod environment;
mod block_interpreter;
mod stmt_interpreter;

use environment::Environment;
use std::collections::HashMap;
use crate::parser::expr::{Expr, LiteralValue};
use crate::parser::stmt::Stmt;
use crate::scanner::token::Token;

pub struct Interpreter{
    variables: Vec<Token>,
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter{variables: vec![], environment:Environment::new()}
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
        println!("{:?}", self.environment)
    }

    pub fn evaluate(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate_expression(expr);
            }
            Stmt::Block{ stmts}  => {
                self.evaluate_block(stmts);
            }
            Stmt::Var {name, initializer} => {
                self.evaluate_var(name, initializer);
            }
            Stmt::If {condition, then_branch, else_branch} => {
                self.evaluate_if_stmt(&condition, then_branch, else_branch);
            }
            _ => {}
        }
    }

    fn evaluate_var(&mut self, name:Token, initializer:Option<Expr>) {
        let lex = name.get_lexeme();
        if(self.environment.contains_in_current_scope(&lex)){
            panic!("Variable `{}` already defined in the current scope", lex);
        }
        let val = match  initializer {
            Some(expr) => self.evaluate_expression(expr),
            _ => LiteralValue::Nil

        };
        self.environment.set(name.get_lexeme().clone(),val)

    }
}
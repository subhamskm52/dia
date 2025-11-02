use std::cmp::PartialEq;
use std::hint::black_box;
use crate::interpreter::Interpreter;
use crate::parser::expr::{Expr, LiteralValue};
use crate::parser::stmt::Stmt;


impl Interpreter {
    pub fn evaluate_if_stmt(&mut self, condition: &Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>>) {
        let cond = self.evaluate_expression(condition.clone());

        if(self.is_truthy(&cond)){
            let stmt = *then_branch;
            match stmt {
                Stmt::Block {stmts} => {
                    self.evaluate_block(stmts);
                }
                _ => panic!("Expected block here"),
            }
        }
        else {
            match else_branch{
                Some(else_branch) =>
                    {
                        let stmt = *else_branch;
                        match stmt {
                            Stmt::Block {stmts} => {
                                self.evaluate_block(stmts);

                            }
                            _ => panic!("Expected block here"),
                        }                    },
                _ => {}
            }
        }
    }
}
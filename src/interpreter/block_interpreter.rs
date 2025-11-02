use crate::interpreter::Interpreter;
use crate::parser::expr::LiteralValue;
use crate::parser::stmt::Stmt;

impl Interpreter{
    pub fn evaluate_block(&mut self, stmts: Vec<Stmt>)  {
        self.environment.push_scope();
        for stmt in stmts {
            self.evaluate(stmt);
        }
        self.environment.pop_scope();
    }
}
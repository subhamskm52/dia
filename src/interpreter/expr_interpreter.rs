use crate::interpreter::Interpreter;
use crate::parser::expr::{Expr, LiteralValue};
use crate::scanner::token_type::TokenType;

impl Interpreter {
    pub fn evaluate_expression(&mut self, expr: Expr) -> LiteralValue {
        match expr {
            Expr::Literal(value ) => { value.clone() }
            Expr::Variable(var_token) => {
                let name = var_token.get_lexeme().to_string();
                self.environment.get(&name).expect(&format!("Undefined Variable: {:?}", var_token))
            }

            Expr::Assign { identifier, value} => {
                let identifier = identifier.get_lexeme().to_string();
                let value = self.evaluate_expression(*value);
                self.environment.update(identifier.clone(), value);
                self.environment.get(&identifier).expect(&format!("Undefined variable '{}'", identifier))
            }

            Expr::Unary {operator, right} => {
                let value = self.evaluate_expression(*right);
                match operator {
                    TokenType::Minus => {
                        match value {
                            LiteralValue::Number(n) => { LiteralValue::Number(-n) }
                            _ => panic!("Unary '-' only works on numbers")
                        }
                    }
                    TokenType::Bang => {
                        LiteralValue::Bool(!self.is_truthy(&value))
                    }
                    _ => panic!("unexpected unary operator: {:?}", operator)
                }
            }
            Expr::Binary { left, operator, right } => {
                let left_val = self.evaluate_expression(*left);
                let right_val = self.evaluate_expression(*right);

                let result: LiteralValue = match operator {
                    TokenType::Plus => match (left_val, right_val) {
                        (LiteralValue::Number(a), LiteralValue::Number(b)) => LiteralValue::Number(a + b),
                        _ => panic!("'+' only supported for numbers"),
                    },
                    TokenType::Minus => match (left_val, right_val) {
                        (LiteralValue::Number(a), LiteralValue::Number(b)) => LiteralValue::Number(a - b),
                        _ => panic!("'-' only supported for numbers"),
                    },
                    TokenType::Star => match (left_val, right_val) {
                        (LiteralValue::Number(a), LiteralValue::Number(b)) => LiteralValue::Number(a * b),
                        _ => panic!("'*' only supported for numbers"),
                    },
                    TokenType::Slash => match (left_val, right_val) {
                        (LiteralValue::Number(a), LiteralValue::Number(b)) => LiteralValue::Number(a / b),
                        _ => panic!("'/' only supported for numbers"),
                    },
                    TokenType::EqualEqual => match (left_val, right_val) {
                        (LiteralValue::Number(a), LiteralValue::Number(b)) => LiteralValue::Bool(a == b),
                        (LiteralValue::Bool(a), LiteralValue::Bool(b)) => LiteralValue::Bool(a == b),
                        (LiteralValue::Nil, LiteralValue::Nil) => LiteralValue::Bool(true),
                        _ => LiteralValue::Bool(false),
                    },

                    TokenType::BangEqual => match (left_val, right_val) {
                        (LiteralValue::Number(a), LiteralValue::Number(b)) => LiteralValue::Bool(a != b),
                        (LiteralValue::Bool(a), LiteralValue::Bool(b)) => LiteralValue::Bool(a != b),
                        (LiteralValue::Nil, LiteralValue::Nil) => LiteralValue::Bool(false),
                        _ => LiteralValue::Bool(true),
                    },
                    _ => panic!("Unsupported binary operator"),
                };
                result
            }
            Expr::Grouping(inner) => {self.evaluate_expression(*inner)}
        }

    }
}
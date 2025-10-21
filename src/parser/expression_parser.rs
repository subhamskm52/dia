use crate::parser::expr::{Expr, LiteralValue};
use crate::parser::Parser;
use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

impl Parser {
    pub fn parse_expression(&mut self) -> Expr {
        self.parse_assignment()
    }
    fn parse_assignment(&mut self) -> Expr {
        let expr = self.parse_equality();
        if(self.match_token(&[TokenType::Equal])){
            let ident = self.previous().clone();
            let value = self.parse_expression();
            return Expr::Assign {identifier: ident, value: Box::new(value), }

        }
        expr
    }

    pub fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_comparison();

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().get_type();
            let right = self.parse_comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    /// comparison → term ((">" | ">=" | "<" | "<=") term)*
    pub fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().get_type();
            let right = self.parse_term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    /// term → factor (("+" | "-") factor)*
    pub fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().get_type();
            let right = self.parse_factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    /// factor → unary (("*" | "/") unary)*
    pub fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_unary();

        while self.match_token(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().get_type();
            let right = self.parse_unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    /// unary → ("!" | "-") unary | primary
    pub fn parse_unary(&mut self) -> Expr {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().get_type();
            let right = self.parse_unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }

        self.parse_primary()
    }

    /// primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"
    pub fn parse_primary(&mut self) -> Expr {
        if self.match_token(&[TokenType::False]) {
            return Expr::Literal(LiteralValue::Bool(false));
        }

        if self.match_token(&[TokenType::True]) {
            return Expr::Literal(LiteralValue::Bool(true));
        }

        if self.match_token(&[TokenType::Nil]) {
            return Expr::Literal(LiteralValue::Nil);
        }

        if self.match_token(&[TokenType::Number]) {
            let value: f64 = self.previous().get_lexeme().parse().unwrap();
            return Expr::Literal(LiteralValue::Number(value));
        }

        if self.match_token(&[TokenType::String]) {
            return Expr::Literal(LiteralValue::String(self.previous().get_lexeme().clone()));
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.parse_expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping(Box::new(expr));
        }
        if self.match_token(&[TokenType::Identifier]) {
            return Expr::Variable(self.previous().clone());
        }
        self.error(self.peek(), "Expected expression.");
        Expr::Literal(LiteralValue::Nil)
    }
}

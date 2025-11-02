pub mod expr;
mod expression_parser;
pub mod stmt;
mod stmt_parser;

use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser { Parser { tokens: tokens.clone(), current: 0, } }

    pub fn is_eof(&self) -> bool {
        self.peek().get_type() == TokenType::Eof
    }

    pub fn advance(&mut self) -> &Token {
        if(!self.is_eof()) {
            self.current += 1;
        }
        self.previous()
    }

    pub fn previous(&mut self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    pub fn peek (&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    pub fn peek_next(&self) -> &Token {
        self.tokens.get(self.current + 1).unwrap_or(self.peek())
    }

    pub fn check(&self, expected: TokenType) -> bool {
        self.peek().get_type() == expected.clone()
    }

    pub fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if(self.check(t.clone())){
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn error(&self, token: &Token, message: &str) {
        eprintln!(
            "[line {}] Syntax error at '{}': {}",
            token.get_line(), token.get_lexeme(), message
        );
    }

    // Consume expected token or report error
    pub fn consume(&mut self, expected: TokenType, message: &str) -> &Token {
        if self.check(expected) {
            return self.advance();
        }
        self.error(self.peek(), message);
        self.peek() // return current token so parsing can continue
    }
}
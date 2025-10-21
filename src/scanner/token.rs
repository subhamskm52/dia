use std::fmt;
use std::fmt::{Debug, Formatter};
use super::token_type::TokenType;

#[derive(Clone)]
pub struct Token{
    token_type: TokenType,
    lexeme: String,
    start: usize,
    end: usize,
    line: usize,
}
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, start: usize, end: usize, line: usize) -> Token {
        Token{token_type, lexeme, start, end, line}
    }

    pub fn get_type(&self) -> TokenType { self.token_type.clone() }
    pub fn get_lexeme(&self) -> &String { &self.lexeme }
    pub fn get_start(&self) -> usize { self.start }
    pub fn get_end(&self) -> usize { self.end }
    pub fn get_line(&self) -> usize { self.line }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {{ type: {:?}, lexeme: \"{}\", start: {}, end: {}, line: {} }}",
            self.token_type,
            self.lexeme,
            self.start,
            self.end,
            self.line
        )
    }
}
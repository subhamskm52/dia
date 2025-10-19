use std::iter::Scan;
use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

pub mod token_type;
pub mod token;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current : usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner{source, tokens: vec![], start: 0, current: 0, line: 1}
    }
    
    pub fn get_tokens(& self) -> &Vec<Token> { &self.tokens }

    pub fn scan_tokens(&mut self) -> &Vec<Token>{
        while !self.is_eof() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), self.start, self.line, self.line ));

        &self.tokens
    }

    fn add_token(&mut self, token: TokenType){
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token, text.to_string(), self.start, self.current, self.line ));
    }

    fn scan_token(&mut self){

        let c = self.advance();
        match c {

            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

        // single line comment and division
            '/' => {
                if(self.peek() == '/'){
                    while self.peek() != '\n' && !self.is_eof(){
                        self.advance();
                    }
                }
                else{
                    self.add_token(TokenType::Slash);
                }
            }

        // equality and comparison operators
            '!' => {
                if self.peek() == '=' {
                    self.advance(); // consume '='
                    self.add_token(TokenType::BangEqual); // !=
                } else {
                    self.add_token(TokenType::Bang); // !
                }
            }
            '=' => {
                if self.peek() == '=' {
                    self.advance(); // consume '='
                    self.add_token(TokenType::EqualEqual); // ==
                } else {
                    self.add_token(TokenType::Equal); // =
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.advance(); // <=
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less); // <
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance(); // >=
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater); // >
                }
            }


        //  WhiteSpaces
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,

        //  Alphanumeric, eg: identifier & keywords
            c if c.is_alphanumeric() || c == '_' => self.identifier(),

        //  Numbers
            c if c.is_ascii_digit() => self.number(),

        //  unexpected token
            _ => { println!("Unexpected character '{}' at line {}, pos {}", c, self.line, self.current ); }
        }
    }
    fn number(&mut self){
        while(self.peek().is_ascii_digit()){
            self.advance();
        }
        let number = &self.source[self.start..self.current];
        self.tokens.push(Token::new(TokenType::Number, number.to_string(), self.start, self.current, self.line ));
    }
    fn identifier(&mut self) {

        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text = &self.source[self.start..self.current];

        let token_type = match text {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        self.add_token(token_type);

    }

    fn peek(&self)-> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }
    fn is_eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let ch = self.source.chars().nth(self.current).unwrap();
        self.current += ch.len_utf8();
        ch
    }

}
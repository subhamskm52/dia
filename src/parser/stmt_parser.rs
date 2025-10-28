use crate::parser::Parser;
use crate::parser::stmt::Stmt;
use crate::scanner::token_type::TokenType;

impl Parser {
    // program → declaration* EOF ;
    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_eof() {
            statements.push(self.declaration());
        }
        statements
    }

    // declaration → funDecl | varDecl | statement ;
    fn declaration(&mut self) -> Stmt {
        if self.match_token(&[TokenType::Fun]) {
            self.fun_declaration()
        } else if self.match_token(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    // funDecl → "fun" IDENTIFIER "(" parameters? ")" block ;
    fn fun_declaration(&mut self) -> Stmt {
        let name = self.consume(TokenType::Identifier, "Expect function name.").clone();

        self.consume(TokenType::LeftParen, "Expect '(' after function name.");
        let mut params = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                params.push(self.consume(TokenType::Identifier, "Expect parameter name.").clone());
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters.");

        let body =  self.parse_block();
        Stmt::Function { name, params, body: Box::new(body)}
    }

    // varDecl → "var" IDENTIFIER ( "=" expression )? ";" ;
    fn var_declaration(&mut self) -> Stmt {
        let name = self.consume(TokenType::Identifier, "Expect variable name.").clone();

        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(self.parse_expression())
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.");

        Stmt::Var { name, initializer }
    }

    // statement → exprStmt | printStmt | block | ifStmt | whileStmt | forStmt | returnStmt | breakStmt | continueStmt ;
    fn statement(&mut self) -> Stmt {
        if self.match_token(&[TokenType::Print]) {
            self.print_statement()
        } else if self.match_token(&[TokenType::LeftBrace]) {
            self.parse_block()
        } else if self.match_token(&[TokenType::If]) {
            self.parse_if_statement()
        } else if self.match_token(&[TokenType::While]) {
            self.parse_while_statement()
        } else if self.match_token(&[TokenType::For]) {
            self.parse_for_statement()
        } else if self.match_token(&[TokenType::Return]) {
            self.parse_return_statement()
        } else if self.match_token(&[TokenType::Break]) {
            self.parse_break_statement()
        } else if self.match_token(&[TokenType::Continue]) {
            self.parse_continue_statement()
        }
        else if self.match_token(&[TokenType::Var]) {
                self.parse_var_declaration()
        }
        else {
            self.expression_statement()
        }
    }

    fn parse_var_declaration(&mut self) -> Stmt {
        let iden = self.consume(TokenType::Identifier, "Expect ';' after variable declaration.").clone();
        let contains_equal = self.check(TokenType::Equal).clone();

        let stmt  = match contains_equal {
            true => {
                self.consume(TokenType::Equal, "Expect '=' after variable declaration.");
                let expr = self.parse_expression();
                Stmt::Var {name: iden.clone(), initializer: Some(expr)}
            }
            _ => {
                Stmt::Var {name: iden.clone(), initializer: None}
            }
        };
        stmt
    }

    // exprStmt → expression ";" ;
    fn expression_statement(&mut self) -> Stmt {
        let expr = self.parse_expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        Stmt::Expression(expr)
    }

    // printStmt → "print" expression ";" ;
    fn print_statement(&mut self) -> Stmt {
        let expr = self.parse_expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        Stmt::Print(expr)
    }

    // block → "{" declaration* "}" ;
    fn parse_block(&mut self) -> Stmt {
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_eof() {
            statements.push(self.declaration());
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.");
        Stmt::Block{stmts: statements}
    }

    // ifStmt → "if" "(" expression ")" statement ( "else" statement )? ;
    fn parse_if_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let condition = self.parse_expression();
        self.consume(TokenType::RightParen, "Expect ')' after if condition.");

        let then_branch = Box::new(self.statement());
        let else_branch = if self.match_token(&[TokenType::Else]) {
            Some(Box::new(self.statement()))
        } else {
            None
        };

        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    // whileStmt → "while" "(" expression ")" statement ;
    fn parse_while_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.");
        let condition = self.parse_expression();
        self.consume(TokenType::RightParen, "Expect ')' after while condition.");

        let body = Box::new(self.statement());
        Stmt::While { condition, body }
    }

    // forStmt → "for" "(" ( varDecl | exprStmt | ";" ) expression? ";" expression? ")" statement ;
    fn parse_for_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");

        // Parse initializer
        let initializer = if self.match_token(&[TokenType::Semicolon]) {
            None
        } else if self.match_token(&[TokenType::Var]) {
            Some(Box::new(self.var_declaration()))
        } else {
            Some(Box::new(self.expression_statement()))
        };

        // Parse condition
        let condition = if !self.check(TokenType::Semicolon) {
            Some(self.parse_expression())
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.");

        // Parse increment
        let increment = if !self.check(TokenType::RightParen) {
            Some(self.parse_expression())
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.");

        let body = Box::new(self.statement());

        Stmt::For {
            initializer,
            condition,
            increment,
            body,
        }
    }

    // returnStmt → "return" expression? ";" ;
    fn parse_return_statement(&mut self) -> Stmt {
        let keyword = self.previous().clone();
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.parse_expression())
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after return value.");
        Stmt::Return { keyword, value }
    }

    // breakStmt → "break" ";" ;
    fn parse_break_statement(&mut self) -> Stmt {
        let keyword = self.previous().clone();
        self.consume(TokenType::Semicolon, "Expect ';' after 'break'.");
        Stmt::Break { keyword }
    }

    // continueStmt → "continue" ";" ;
    fn parse_continue_statement(&mut self) -> Stmt {
        let keyword = self.previous().clone();
        self.consume(TokenType::Semicolon, "Expect ';' after 'continue'.");
        Stmt::Continue { keyword }
    }
}

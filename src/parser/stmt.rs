// program        → declaration* EOF ;
//
// declaration    → funDecl
// | varDecl
// | statement ;
//
// funDecl        → "fun" IDENTIFIER "(" parameters? ")" block ;
// parameters     → IDENTIFIER ( "," IDENTIFIER )* ;
//
// varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
//
// statement      → exprStmt
// | printStmt
// | block
// | ifStmt
// | whileStmt
// | forStmt
// | returnStmt
// | breakStmt
// | continueStmt ;
//
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;
// block          → "{" declaration* "}" ;
// ifStmt         → "if" "(" expression ")" statement ( "else" statement )? ;
// whileStmt      → "while" "(" expression ")" statement ;
// forStmt        → "for" "(" ( varDecl | exprStmt | ";" )
// expression? ";"
// expression? ")" statement ;
// returnStmt     → "return" expression? ";" ;
// breakStmt      → "break" ";" ;
// continueStmt   → "continue" ";" ;

use crate::parser::expr::Expr;
use crate::scanner::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    /// expression ;
    Expression(Expr),

    /// print expression ;
    Print(Expr),

    /// var IDENTIFIER ( "=" expression )? ;
    Var {
        name: Token,
        initializer: Option<Expr>,
    },

    /// IDENTIFIER "=" expression ;
    Assignment {
        name: Token,
        value: Expr,
    },

    /// { declaration* }
    Block{stmts: Vec<Stmt>},

    /// if ( condition ) then_branch else_branch?
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    /// while ( condition ) body
    While {
        condition: Expr,
        body: Box<Stmt>,
    },

    /// for ( init ; cond ; inc ) body
    /// (Will be desugared into Block + While)
    For {
        initializer: Option<Box<Stmt>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Box<Stmt>,
    },

    /// fun IDENTIFIER (params) { body }
    Function {
        name: Token,
        params: Vec<Token>,
        body: Box<Stmt>,
    },

    /// return expression? ;
    Return {
        keyword: Token,
        value: Option<Expr>,
    },

    /// break ;
    Break {
        keyword: Token,
    },

    /// continue ;
    Continue {
        keyword: Token,
    },
}


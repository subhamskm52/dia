// ==========================================
// Expression Grammar (BNF-style)
//
// expression     → assignment ;
// assignment     → equality | IDENTIFIER "=" assignment
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
// ==========================================

use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),       // Numeric literals, e.g., 42, 3.14
    String(String),    // String literals, e.g., "hello"
    Bool(bool),        // true or false
    Nil,               // Represents absence of a value
}

/// AST node representing an expression
#[derive(Debug, Clone)]
pub enum Expr {
    /// A literal value
    Literal(LiteralValue),
    Variable(Token),
    Assign{
        identifier: Token,
        value: Box<Expr>,
    },
    /// Unary operator expression, e.g., -x or !flag
    Unary {
        operator: TokenType,
        right: Box<Expr>,   // Expression the operator applies to
    },

    /// Binary operator expression, e.g., x + y, a * b
    Binary {
        left: Box<Expr>,    // Left-hand side expression
        operator: TokenType,
        right: Box<Expr>,   // Right-hand side expression
    },

    /// Grouped expression, e.g., (a + b)
    Grouping(Box<Expr>),
}


#[derive(Debug)]
pub enum TokenType{
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace, Semicolon, Colon,
    Dot, Comma, Plus, Minus, Star, Slash,

    // one or two character token
    Equal, EqualEqual,
    Bang, BangEqual,
    Less, LessEqual,
    Greater, GreaterEqual,

    // Literals
    Identifier, String, Number,

    //Keywords
    Class, Super, This, Fun, Var,
    If, Else, For, While,
    And, Or,
    Return, True, False, Nil,
    Print,

    Eof
}
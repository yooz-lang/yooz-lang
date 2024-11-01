#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal, 
    EOF, 
    Identifier(String), 

    // Operators
    Assign,
    Plus,
    Minus,
    Slash,
    Asterisk,
    Modulo,

    // Symbols
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Hashtag,
    DoubleQuote,
    Underline,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // seperators
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Identifier(String),

    // basic data types
    Integer(i32),
    Double(f64),
    String(String),
    Boolean(bool),

    // operators
    Plus,
    Mul,
    Div,
    Min,
    Assign,

    Equals,

    Illegal,
    EOF,
}

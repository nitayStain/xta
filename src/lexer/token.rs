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

    And,
    Not,
    Or,
    Equals,
    NotEquals,
    Greater,
    GreaterOrEqu,
    Lower,
    LowerOrEqu,

    BNot,
    BAnd,
    BOr,
    Xor,
    RightSh,
    LeftSh,

    // keywords
    Const,
    Let,
    If,
    Elif,
    Else,
    For,
    Loop,

    Illegal,
    EOF,
}

pub fn lookup_keyword(identifier: &str) -> Option<Token> {
    match identifier {
        "const" => Some(Token::Const),
        "let" => Some(Token::Let),
        "if" => Some(Token::If),
        "elif" => Some(Token::Elif),
        "else" => Some(Token::Else),
        "for" => Some(Token::For),
        "loop" => Some(Token::Loop),
        _ => None,
    }
}
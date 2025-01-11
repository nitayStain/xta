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
    None,

    // represents a typename
    TypeName(String),
    ReturnTypeArrow,

    // operators
    Plus,
    Mul,
    Div,
    Min,
    Inc,
    Dec,
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
    While,

    // do-while stuff
    Loop,
    Unless,

    // general loop stuff
    Break,
    Continue,

    // func def
    Fn,

    Illegal(String),
    EOF,
}

pub fn lookup_keyword(identifier: &str) -> Option<Token> {
    match identifier {
        "const" => Some(Token::Const),
        "let" => Some(Token::Let),

        "if" => Some(Token::If),
        "elif" => Some(Token::Elif),
        "else" => Some(Token::Else),

        "fn" => Some(Token::Fn),

        "for" => Some(Token::For),

        "loop" => Some(Token::Loop),
        "unless" => Some(Token::Unless),

        "while" => Some(Token::While),

        "break" => Some(Token::Break),
        "continue" => Some(Token::Continue),

        "Str" => Some(Token::TypeName(identifier.to_string())),
        "Int" => Some(Token::TypeName(identifier.to_string())),
        "Boolean" => Some(Token::TypeName(identifier.to_string())),
        "Double" => Some(Token::TypeName(identifier.to_string())),
        "None" => Some(Token::None),

        _ => None,
    }
}

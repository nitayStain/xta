#[derive(Debug, PartialEq, Clone)]
pub struct Loc {
    pub row: u32,
    pub col: u32
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: Loc,
}

impl Token {
    pub fn new(kind: TokenKind, loc: Loc) -> Self {
        Self {
            kind,
            loc,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
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

pub fn lookup_keyword(identifier: &str) -> Option<TokenKind> {
    match identifier {
        "const" => Some(TokenKind::Const),
        "let" => Some(TokenKind::Let),

        "if" => Some(TokenKind::If),
        "elif" => Some(TokenKind::Elif),
        "else" => Some(TokenKind::Else),

        "fn" => Some(TokenKind::Fn),

        "for" => Some(TokenKind::For),

        "loop" => Some(TokenKind::Loop),
        "unless" => Some(TokenKind::Unless),

        "while" => Some(TokenKind::While),

        "break" => Some(TokenKind::Break),
        "continue" => Some(TokenKind::Continue),

        "Str" => Some(TokenKind::TypeName(identifier.to_string())),
        "Int" => Some(TokenKind::TypeName(identifier.to_string())),
        "Boolean" => Some(TokenKind::TypeName(identifier.to_string())),
        "Double" => Some(TokenKind::TypeName(identifier.to_string())),
        "None" => Some(TokenKind::None),

        _ => None,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Loc {
    pub row: u32,
    pub col: u32
}

impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: Loc,
    pub text: String
}

impl Token {
    pub fn new(kind: TokenKind, loc: Loc, text: String) -> Self {
        Self {
            kind,
            loc,
            text,
        }
    }

    pub fn from_kind(kind: TokenKind) -> Self {
        Self {
            kind,
            loc: Loc { row: 0, col: 0 },
            text: "".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // seperators
    Semicolon,
    Comma,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Identifier,

    // basic data types
    Integer,
    Double,
    String,
    Boolean,
    None,

    // points to the return type of a function (.e.g. fn foo() -> int)
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

    Illegal,
    EOF,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.kind {
            // Arithmetic operators
            TokenKind::Min => write!(f, "-"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Mul => write!(f, "*"),
            TokenKind::Div => write!(f, "/"),
            TokenKind::Dec => write!(f, "--"),
            TokenKind::Inc => write!(f, "++"),
            
            // Logical operators
            TokenKind::Greater => write!(f, ">"),
            TokenKind::GreaterOrEqu => write!(f, ">="),
            TokenKind::And => write!(f, "&&"),
            TokenKind::Equals => write!(f, "=="),
            TokenKind::Lower => write!(f, "<"),
            TokenKind::LowerOrEqu => write!(f, "<="),
            TokenKind::Not => write!(f, "!"),
            TokenKind::NotEquals => write!(f, "!="),
            TokenKind::Or => write!(f, "||"),
            
            TokenKind::Identifier => write!(f, "{}", self.text),
            
            // punctuation
            TokenKind::Assign => write!(f, "="),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::ReturnTypeArrow => write!(f, "->"),
            TokenKind::Comma => write!(f, ","),

            // bitwise operators
            TokenKind::BAnd => write!(f, "&"),
            TokenKind::BNot => write!(f, "~"),
            TokenKind::BOr => write!(f, "|"),
            TokenKind::LeftSh => write!(f, "<<"),
            TokenKind::RightSh => write!(f, ">>"),
            TokenKind::Xor => write!(f, "^"),
            
            // data values
            TokenKind::Double => write!(f, "{}", self.text),
            TokenKind::Integer => write!(f, "{}", self.text),
            TokenKind::String => write!(f, "{}", self.text),
            TokenKind::Boolean => write!(f, "{}", self.text),

            // keywords
            TokenKind::Break => write!(f, "break"),
            TokenKind::Continue => write!(f, "continue"),
            TokenKind::Const => write!(f, "const"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Elif => write!(f, "elif"),
            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::For => write!(f, "for"),
            TokenKind::Fn => write!(f, "fn"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::None => write!(f, "None"),
            TokenKind::Unless => write!(f, "unless"),
            TokenKind::While => write!(f, "while"),
            TokenKind::Loop => write!(f, "loop"),

            TokenKind::Illegal => write!(f, "{}", self.text),

        }
    }
}

pub fn lookup_keyword(identifier: &str) -> TokenKind {
    match identifier {
        "const" => TokenKind::Const,
        "let" => TokenKind::Let,

        "if" => TokenKind::If,
        "elif" => TokenKind::Elif,
        "else" => TokenKind::Else,

        "fn" => TokenKind::Fn,

        "for" => TokenKind::For,

        "loop" => TokenKind::Loop,
        "unless" => TokenKind::Unless,

        "while" => TokenKind::While,

        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,

        "None" => TokenKind::None,

        _ => TokenKind::Identifier,
    }
}

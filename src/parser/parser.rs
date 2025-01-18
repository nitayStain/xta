use crate::{scanner::Scanner, token::TokenKind};

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    token: Option<TokenKind>,
}

impl<'a> Parser<'a> {
    pub fn new(scanner: Scanner<'a>) -> Self {
        Self {
            scanner,
            token: None,
        }
    }
}

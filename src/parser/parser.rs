use crate::{scanner::Scanner, token::Token};

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(scanner: Scanner<'a>) -> Self {
        Self {
            scanner,
            token: None,
        }
    }
}

use crate::{scanner::Scanner, token::{Loc, Token, TokenKind}};

use super::ast::{Expr, LiteralExpr, Stmt, VarStmt};

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    token: Token,
    pub errors: Vec<Error>,
}

impl<'a> Parser<'a> {
    pub fn new(mut scanner: Scanner<'a>) -> Self {
        let token = scanner.next_token();
        
        Self {
            scanner,
            token,
            errors: Vec::new(),
        }
    }


    pub fn parse_statement(&mut self) -> Option<Stmt> {
        match self.peek().kind {
            TokenKind::Let => self.parse_variable_declaration(),
            _ => None,
        }
    }

    pub fn parse_variable_declaration(&mut self) -> Option<Stmt> {
        self.expect(TokenKind::Let)?;
        let name = self.expect(TokenKind::Identifier)?;
        self.expect(TokenKind::Assign)?;

        let value = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;

        Some(Stmt::Variable(VarStmt {value: value, name: name.text, is_const: false}))
    }

    pub fn parse_expression(&mut self) -> Option<Expr> {
        match self.peek().kind {
            TokenKind::Integer => self.parse_integer(),
            // TokenKind::Identifier => self.parse_identifier(),
            _ => None,
        }
    }

    pub fn parse_integer(&mut self) -> Option<Expr> {
        let token = self.expect(TokenKind::Integer)?;
        Some(Expr::Literal(LiteralExpr::Integer(token.text.parse().unwrap())))
    }
}

// Private functions
impl <'a> Parser <'a> {
    fn consume(&mut self) -> Token {
        std::mem::replace(&mut self.token, self.scanner.next_token())
    }

    fn peek(&self) -> &Token {
        &self.token
    }

    fn expect(&mut self, kind: TokenKind) -> Option<Token> {
        if self.token.kind == kind {
            Some(self.consume())
        } else {
            self.errors.push(Error::Expected {
                loc: self.token.loc.clone(),
                expected: Token::from_kind(kind),
                found: self.token.clone(),
            });

            None
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("~ ({loc}) : Expected `{expected}`, found `{found}`")]
    Expected { loc: Loc, expected: Token, found: Token },
}
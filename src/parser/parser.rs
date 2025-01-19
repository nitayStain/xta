use crate::{scanner::Scanner, token::{Loc, Token, TokenKind}};

use super::ast::{Expr, FunctionStmt, LiteralExpr, Param, Stmt, VarStmt};

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

    pub fn parse_file(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        while self.peek().kind != TokenKind::EOF {
            if let Some(stmt) = self.parse_statement() {
                stmts.push(stmt);
            } else {
                self.consume();
            }
        }

        stmts
    }

    pub fn parse_statement(&mut self) -> Option<Stmt> {
        match self.peek().kind {
            TokenKind::Let => self.parse_variable_declaration(),
            TokenKind::Fn => self.parse_function(),
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

    // Following the next syntax:
    // fn foo(a int, b int) -> int { <body> }
    pub fn parse_function(&mut self) -> Option<Stmt> {
        self.expect(TokenKind::Fn)?;
        let name = self.expect(TokenKind::Identifier)?;

        let params = match self.parse_function_params() {
            Some(params) => params,
            None => Vec::new(),
        };
        
        let return_type = if self.peek().kind == TokenKind::ReturnTypeArrow {
            self.consume();
            Some(self.expect(TokenKind::Identifier)?.text)
        } else {
            None
        };

        let body =  self.parse_scope()?;

        Some(Stmt::Function(FunctionStmt {name: name.text, params: params, body: body, return_type: return_type}))
    }

    pub fn parse_expression(&mut self) -> Option<Expr> {
        match self.peek().kind {
            TokenKind::Integer => self.parse_integer(),
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

    fn parse_function_params(&mut self) -> Option<Vec<Param>> {

        self.expect(TokenKind::LeftParen)?;
        
        // handle params
        let mut params = Vec::new();
        let has_params = self.peek().kind != TokenKind::RightParen;

        if has_params {
            loop {
                let param_name = self.expect(TokenKind::Identifier)?;
                let param_type = self.expect(TokenKind::Identifier)?;
                
                params.push(Param { name: param_name.text.clone(), param_type: param_type.text.clone()});

                if self.peek().kind == TokenKind::RightParen {
                    break;
                }

                if self.peek().kind == TokenKind::Comma {
                    self.consume();
                    continue;
                } else {
                    self.errors.push(Error::Expected {
                        loc: self.peek().loc.clone(),
                        expected: Token::from_kind(TokenKind::Comma),
                        found: self.peek().clone(),
                    });
                }

            };
        }

        self.expect(TokenKind::RightParen)?;
        Some(params)
    }

    fn parse_scope(&mut self) -> Option<Vec<Stmt>> {
        self.expect(TokenKind::LeftBrace)?;

        let mut stmts = Vec::new();
        while self.peek().kind != TokenKind::RightBrace {
            if let Some(stmt) = self.parse_statement() {
                stmts.push(stmt);
            } else {
                self.consume();
            }
        }

        self.expect(TokenKind::RightBrace)?;

        Some(stmts)
    }

    fn get_precedence(&self, kind: TokenKind) -> Option<u8> {
        match kind {
            TokenKind::Plus | TokenKind::Min => Some(1),
            TokenKind::Mul | TokenKind::Div => Some(2),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("~ ({loc}) : Expected `{expected}`, found `{found}`")]
    Expected { loc: Loc, expected: Token, found: Token },
}
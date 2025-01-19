use crate::{scanner::Scanner, token::{Loc, Token, TokenKind}};

use super::ast::{BinaryExpr, BinaryOpType, Expr, FunctionStmt, IdentifierExpr, Literal, LiteralExpr, Param, Stmt, UnaryExpr, UnaryOpType, VarDeclStmt};

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
            _ => Some(Stmt::Expr(self.parse_expression(None)?)),
        }
    }

    pub fn parse_variable_declaration(&mut self) -> Option<Stmt> {
        self.expect(TokenKind::Let)?;
        let name = self.expect(TokenKind::Identifier)?;

        self.expect(TokenKind::Assign)?;

        let value = self.parse_expression(None)?;
        self.expect(TokenKind::Semicolon)?;

        Some(Stmt::VarDecl(VarDeclStmt {value: Some(value), name: name.text, is_const: false}))
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

    pub fn parse_expression(&mut self, prec: Option<i8>) -> Option<Expr> {
        let prec = prec.unwrap_or(-1);

        let mut lhs = self.parse_unary()?;

        if self.peek().kind == TokenKind::Assign && prec < BinaryOpType::Assign.prec() as i8 {
            return self.parse_reassign(lhs);
        }


        while let Some(op) = self.parse_binary_op() {
            let op_prec = op.prec() as i8;
            if op_prec <= prec {
                break;
            }

            self.consume();
            let rhs = self.parse_expression(Some(op_prec))?;
            lhs = Expr::Binary(BinaryExpr {
                left: Box::new(lhs),
                right: Box::new(rhs),
                operator: op,
                loc: self.peek().loc.clone(),
            })
        }

        Some(lhs)
    }

    pub fn parse_unary(&mut self) -> Option<Expr> {
        match self.peek().kind {
            TokenKind::Min => {
                let loc = self.peek().loc.clone();
                self.consume();
                let op = self.parse_expression(Some(UnaryOpType::Neg as i8));
                Some(Expr::Unary(UnaryExpr {
                    operand: Box::new(op.unwrap()),
                    operator: UnaryOpType::Neg,
                    loc
                }))
            }
            TokenKind::Not => {
                let loc = self.peek().loc.clone();
                self.consume();
                let op = self.parse_expression(Some(UnaryOpType::Not as i8));
                Some(Expr::Unary(UnaryExpr {
                    operand: Box::new(op.unwrap()),
                    operator: UnaryOpType::Not,
                    loc
                }))
            }
            _ => self.parse_primary()
        }
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

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.peek().kind {
            TokenKind::Integer => self.parse_integer(),
            TokenKind::Identifier => self.parse_identifier(),
            _ => None,
        }
    }

    fn parse_binary_op(&mut self) -> Option<BinaryOpType> {
        match self.peek().kind {
            TokenKind::Plus => Some(BinaryOpType::Add),
            TokenKind::Min => Some(BinaryOpType::Sub),
            TokenKind::Mul => Some(BinaryOpType::Mul),
            TokenKind::Div => Some(BinaryOpType::Div),
            TokenKind::Assign => Some(BinaryOpType::Assign),
            TokenKind::And => Some(BinaryOpType::And),
            TokenKind::Or => Some(BinaryOpType::Or),
            TokenKind::Equals => Some(BinaryOpType::Eq),
            TokenKind::NotEquals => Some(BinaryOpType::Neq),
            TokenKind::Greater => Some(BinaryOpType::Greater),
            TokenKind::GreaterOrEqu => Some(BinaryOpType::GreaterEq),
            TokenKind::Lower => Some(BinaryOpType::Smaller),
            TokenKind::LowerOrEqu => Some(BinaryOpType::SmallerEq),
            TokenKind::BAnd => Some(BinaryOpType::BitAnd),
            TokenKind::BOr => Some(BinaryOpType::BitOr),
            TokenKind::Xor => Some(BinaryOpType::BitXor),
            TokenKind::RightSh => Some(BinaryOpType::RShift),
            TokenKind::LeftSh => Some(BinaryOpType::LShift),
            _ => None,
        }
    }

    fn parse_reassign(&mut self, lhs: Expr) -> Option<Expr> {
        self.expect(TokenKind::Assign)?;
        if ! matches!(lhs, Expr::Identifier(_)) {
            self.errors.push(Error::ExpectedId { loc: lhs.loc(), found: self.peek().clone() });
            None
        } else {
            let rhs = self.parse_expression(Some(BinaryOpType::Assign.prec() as i8))?;
            let result = Some(Expr::Binary(BinaryExpr {
                left: Box::new(lhs),
                right: Box::new(rhs),
                operator: BinaryOpType::Assign,
                loc: self.peek().loc.clone(),
            }));

            self.expect(TokenKind::Semicolon)?;
            result
        }
    }

    // primary expression parsing
    fn parse_integer(&mut self) -> Option<Expr> {
        let token = self.expect(TokenKind::Integer)?;
        Some(Expr::Literal(LiteralExpr { value: Literal::Integer(token.text.parse().unwrap()), loc: token.loc.clone() }))
    }

    fn parse_identifier(&mut self) -> Option<Expr> {
        let token = self.expect(TokenKind::Identifier)?;
        Some(Expr::Identifier(IdentifierExpr { name: token.text.clone(), loc: token.loc.clone() }))
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("~ ({loc}) : Expected `{expected}`, found `{found}`")]
    Expected { loc: Loc, expected: Token, found: Token },

    #[error("~ ({loc}) : Expected an identifier, found `{found}`")]
    ExpectedId {loc: Loc, found: Token }
}
use xta_lexer::{scanner::Scanner, token::{Loc, Token, TokenKind}};

use crate::ast::{CallExpr, ReturnStmt};

use super::ast::{BinaryExpr, BinaryOpType, Block, ElifStmt, Expr, FunctionDeclStmt, IdentifierExpr, IfStmt, Literal, LiteralExpr, Param, Stmt, UnaryExpr, UnaryOpType, VarDeclStmt};

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    token: Token<'a>,
    pub errors: Vec<Error<'a>>,
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

    pub fn parse_file(&mut self) -> Vec<Stmt<'a>> {
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

    pub fn parse_statement(&mut self) -> Option<Stmt<'a>> {
        let result = match self.peek().kind {
            TokenKind::Let => self.parse_variable_declaration(),
            TokenKind::Fn => self.parse_function(),
            TokenKind::If => self.parse_if(),
            TokenKind::Return => self.parse_return(),
            _ => Some(Stmt::Expr(self.parse_expression(None)?)),
        };

        // NOTE: add here every other statement that doesnt have a semicolon at the end.
        if matches!(result, Some(Stmt::FunctionDecl(_)) | Some(Stmt::If(_))) {
            return result;
        }

        self.expect(TokenKind::Semicolon)?;
        result
    }

    // Following the next syntax:
    // return <expression>;
    pub fn parse_return(&mut self) -> Option<Stmt<'a>> {
        self.expect(TokenKind::Return)?;
        // expect any kind of void returns
        if self.peek().kind == TokenKind::Semicolon {
            return Some(Stmt::Return(ReturnStmt { value: None }));
        }

        // expect a value to be returned
        let value = self.parse_expression(None)?;
        Some(Stmt::Return(ReturnStmt { value: Some(value) }))
    }

    // Following the next syntax:
    // let <var name> = <expression>;
    pub fn parse_variable_declaration(&mut self) -> Option<Stmt<'a>> {
        self.expect(TokenKind::Let)?;
        let name = self.expect(TokenKind::Identifier)?;

        self.expect(TokenKind::Assign)?;

        let value = self.parse_expression(None)?;

        Some(Stmt::VarDecl(VarDeclStmt {value: Some(value), name: name.text, is_const: false}))
    }

    // Following the next syntax:
    // if(a > b) { <body> } (optional) elif (...) { <body> } (optional) else { ... }
    pub fn parse_if(&mut self) -> Option<Stmt<'a>> {
        self.expect(TokenKind::If)?;
        
        let condition = self.parse_expression(None)?; 

        let mut elif_branch = Vec::new();

        let mut else_branch : Option<Block<'a>> = None;

        let then = self.parse_scope()?;

        while self.peek().kind == TokenKind::Elif {
            elif_branch.push(self.parse_elif()?); 
        }

        // TODO: fix when no else is added.
        if self.peek().kind == TokenKind::Else {
            self.consume();
            else_branch = Some(self.parse_scope()?);
        }

        Some(Stmt::If(IfStmt { condition, then, elif_branch, else_branch}))
    }

    pub fn parse_elif(&mut self) -> Option<ElifStmt<'a>> {
        self.expect(TokenKind::Elif)?;
        
        let condition = self.parse_expression(None)?;

        let then = self.parse_scope()?;

        Some(ElifStmt { condition, then })
    }

    // Following the next syntax:
    // fn foo(a int, b int) -> int { <body> }
    pub fn parse_function(&mut self) -> Option<Stmt<'a>> {
        self.expect(TokenKind::Fn)?;
        let name = self.expect(TokenKind::Identifier)?;

        let params = self.parse_function_params().unwrap_or_default();
        
        let return_type = if self.peek().kind == TokenKind::ReturnTypeArrow {
            self.consume();
            Some(self.expect(TokenKind::Identifier)?.text)
        } else {
            None
        };

        let body =  self.parse_scope()?;

        Some(Stmt::FunctionDecl(FunctionDeclStmt {name: name.text, params, body, return_type}))
    }

    pub fn parse_expression(&mut self, prec: Option<i8>) -> Option<Expr<'a>> {
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

    pub fn parse_unary(&mut self) -> Option<Expr<'a>> {
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
    fn consume(&mut self) -> Token<'a> {
        std::mem::replace(&mut self.token, self.scanner.next_token())
    }

    fn peek(&self) -> &Token<'a> {
        &self.token
    }

    fn expect(&mut self, kind: TokenKind) -> Option<Token<'a>> {
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

    fn parse_function_params(&mut self) -> Option<Vec<Param<'a>>> {

        self.expect(TokenKind::LeftParen)?;
        
        // handle params
        let mut params = Vec::new();
        let has_params = self.peek().kind != TokenKind::RightParen;

        if has_params {
            loop {
                let param_name = self.expect(TokenKind::Identifier)?;
                let param_type = self.expect(TokenKind::Identifier)?;
                
                params.push(Param { name: param_name.text, param_type: param_type.text});

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

    fn parse_scope(&mut self) -> Option<Vec<Stmt<'a>>> {
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

    fn parse_primary(&mut self) -> Option<Expr<'a>> {
        match self.peek().kind {
            TokenKind::Integer => self.parse_integer(),
            TokenKind::Identifier => self.parse_identifier(),
            TokenKind::String => self.parse_string(),
            TokenKind::LeftParen => {
                self.consume();
                let expr = self.parse_expression(None);
                self.expect(TokenKind::RightParen)?;
                expr
            }
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

    fn parse_reassign(&mut self, lhs: Expr<'a>) -> Option<Expr<'a>> {
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

            result
        }
    }

    // primary expression parsing
    fn parse_integer(&mut self) -> Option<Expr<'a>> {
        let token = self.expect(TokenKind::Integer)?;
        Some(Expr::Literal(LiteralExpr { value: Literal::Integer(token.text.parse().unwrap()), loc: token.loc.clone() }))
    }

    fn parse_string(&mut self) -> Option<Expr<'a>> {
        let token = self.expect(TokenKind::String)?;
        Some(Expr::Literal(LiteralExpr { value: Literal::String(token.text), loc: token.loc.clone() }))
    }

    fn parse_identifier(&mut self) -> Option<Expr<'a>> {
        let token = self.expect(TokenKind::Identifier)?;

        if self.peek().kind == TokenKind::LeftParen {
            self.parse_fn_call(token)
        } else {
            Some(Expr::Identifier(IdentifierExpr { name: token.text, loc: token.loc.clone() }))
        }

    }

    // Parses any function call
    fn parse_fn_call(&mut self, identifier: Token<'a>) -> Option<Expr<'a>> {
        self.expect(TokenKind::LeftParen)?;
        let mut args = Vec::new();
        while self.peek().kind != TokenKind::RightParen {
            let arg = self.parse_expression(None)?;
            args.push(arg);

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
        }

        self.expect(TokenKind::RightParen)?;
        Some(Expr::Call(CallExpr { name: identifier.text, args, loc: identifier.loc.clone() }))
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error<'a> {
    #[error("~ ({loc}) : Expected `{expected}`, found `{found}`")]
    Expected { loc: Loc, expected: Token<'a>, found: Token<'a> },

    #[error("~ ({loc}) : Expected an identifier, found `{found}`")]
    ExpectedId {loc: Loc, found: Token<'a> }
}

use crate::token::TokenKind;

pub type Block = Vec<Stmt>;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(TokenKind),
    Variable(String),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Variable(VarStmt),
    If(IfStmt),
}

// custom expressions
#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    left: Box<Expr>,
    right: Box<Expr>,
    operator: TokenKind,
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpr {
    operand: Box<Expr>,
    operator: TokenKind,
}

// custom statements
#[derive(Debug, PartialEq)]
pub struct VarStmt {
    name: String,
    value: Box<Expr>,
    is_const: bool,
}

#[derive(Debug, PartialEq)]
pub struct IfStmt {
    condition: Expr,
    then: Block,
    elif_branch: Vec<ElifStmt>,
    else_branch: Option<Block>,
}

#[derive(Debug, PartialEq)]
pub struct ElifStmt {
    condition: Expr,
    then: Block,
}

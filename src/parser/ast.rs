use crate::token::Token;

pub type Block = Vec<Stmt>;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(Token),
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
    operator: Token,
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpr {
    operand: Box<Expr>,
    operator: Token,
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

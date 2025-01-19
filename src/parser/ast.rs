use crate::token::{Loc, Token};

pub type Block = Vec<Stmt>;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Variable(VarStmt),
    Function(FunctionStmt),
    If(IfStmt),
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(LiteralExpr),
    Identifier(IdentifierExpr),
}


// custom expressions

#[derive(Debug, PartialEq)]
pub struct IdentifierExpr {
    pub name: String,
    pub loc: Loc
}

#[derive(Debug, PartialEq)]
pub enum LiteralExpr {
    Integer(i64),
    Double(f64),
    String(String),
    Boolean(bool),
    None,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Token,
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpr {
    pub operand: Box<Expr>,
    pub operator: Token,
}

// custom statements
#[derive(Debug, PartialEq)]
pub struct VarStmt {
    pub name: String,
    pub value: Expr,
    pub is_const: bool,
}

#[derive(Debug, PartialEq)]
pub struct IfStmt {
    pub condition: Expr,
    pub then: Block,
    pub elif_branch: Vec<ElifStmt>,
    pub else_branch: Option<Block>,
}

#[derive(Debug, PartialEq)]
pub struct ElifStmt {
    pub condition: Expr,
    pub then: Block,
}

#[derive(Debug, PartialEq)]
pub struct FunctionStmt {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<String>,
    pub body: Block,
}

#[derive(Debug, PartialEq)]
pub struct Param { 
    pub name: String,
    pub param_type: String,
}
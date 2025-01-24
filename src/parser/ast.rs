use crate::token::Loc;

pub type Block = Vec<Stmt>;

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOpType {
    Neg,
    Not,
    Inc,
    Dec,
    BitNot,
}


#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOpType {
    Add,
    Sub,
    Mul,
    Div,

    And,
    Or,

    Eq,
    Neq,
    Smaller,
    Greater,
    SmallerEq,
    GreaterEq,

    BitAnd,
    BitOr,
    BitXor,
    LShift,
    RShift,

    Assign,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    VarDecl(VarDeclStmt),
    Function(FunctionStmt),
    If(IfStmt),
    Expr(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(LiteralExpr),
    Identifier(IdentifierExpr),
}


// custom expressions

#[derive(Debug, PartialEq, Clone)]
pub struct IdentifierExpr {
    pub name: String,
    pub loc: Loc
}

#[derive(Debug, PartialEq, Clone)]
pub struct LiteralExpr {
    pub value: Literal,
    pub loc: Loc,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Integer(i64),
    Double(f64),
    String(String),
    Boolean(bool),
    None,
}



#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: BinaryOpType,
    pub loc: Loc,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpr {
    pub operand: Box<Expr>,
    pub operator: UnaryOpType,
    pub loc: Loc,
}

// custom statements
#[derive(Debug, PartialEq, Clone)]
pub struct VarDeclStmt {
    pub name: String,
    pub value: Option<Expr>,
    pub is_const: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub then: Block,
    pub elif_branch: Vec<ElifStmt>,
    pub else_branch: Option<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElifStmt {
    pub condition: Expr,
    pub then: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionStmt {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<String>,
    pub body: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Param { 
    pub name: String,
    pub param_type: String,
}

/// Implementations:

impl BinaryOpType {
    pub fn prec(&self) -> u8 {
        match self {
            BinaryOpType::Assign => 0,
            BinaryOpType::Or => 1,
            BinaryOpType::And => 2,
            BinaryOpType::Smaller | BinaryOpType::Greater | BinaryOpType::SmallerEq | BinaryOpType::GreaterEq => 3,
            BinaryOpType::Eq | BinaryOpType::Neq => 4,
            BinaryOpType::BitOr => 5,
            BinaryOpType::BitXor => 6,
            BinaryOpType::BitAnd => 7,
            BinaryOpType::LShift | BinaryOpType::RShift => 8,
            BinaryOpType::Add | BinaryOpType::Sub => 9,
            BinaryOpType::Mul | BinaryOpType::Div => 10,
        }
    }

    pub fn is_logical(&self) -> bool {
        matches!(self, BinaryOpType::Or | BinaryOpType::And)
    }

    pub fn is_bitwise(&self) -> bool {
        matches!(self, BinaryOpType::BitOr | BinaryOpType::BitXor | BinaryOpType::BitAnd | BinaryOpType::LShift | BinaryOpType::RShift)
    }

    pub fn is_comparison(&self) -> bool {
        matches!(self, BinaryOpType::Smaller | BinaryOpType::Greater | BinaryOpType::SmallerEq | BinaryOpType::GreaterEq | BinaryOpType::Eq | BinaryOpType::Neq)
    }
}

impl Expr {
    pub fn loc(&self) -> Loc {
        match self {
            Expr::Binary(expr) => expr.loc.clone(),
            Expr::Unary(expr) => expr.loc.clone(),
            Expr::Literal(expr) => expr.loc.clone(),
            Expr::Identifier(expr) => expr.loc.clone(),
        }
    }
}
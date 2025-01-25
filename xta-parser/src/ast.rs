use xta_lexer::token::Loc;

pub type Block<'a> = Vec<Stmt<'a>>;

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
pub enum Stmt<'a> {
    VarDecl(VarDeclStmt<'a>),
    FunctionDecl(FunctionDeclStmt<'a>),
    If(IfStmt<'a>),
    Return(ReturnStmt<'a>),
    Expr(Expr<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'a> {
    Binary(BinaryExpr<'a>),
    Unary(UnaryExpr<'a>),
    Literal(LiteralExpr<'a>),
    Identifier(IdentifierExpr<'a>),
    Call(CallExpr<'a>),
}


// custom expressions

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr<'a> {
    pub name: &'a str,
    pub args: Vec<Expr<'a>>,
    pub loc: Loc,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdentifierExpr<'a> {
    pub name: &'a str,
    pub loc: Loc
}

#[derive(Debug, PartialEq, Clone)]
pub struct LiteralExpr<'a> {
    pub value: Literal<'a>,
    pub loc: Loc,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal<'a> {
    Integer(i64),
    Double(f64),
    String(&'a str),
    Boolean(bool),
    None,
}



#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr<'a> {
    pub left: Box<Expr<'a>>,
    pub right: Box<Expr<'a>>,
    pub operator: BinaryOpType,
    pub loc: Loc,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpr<'a> {
    pub operand: Box<Expr<'a>>,
    pub operator: UnaryOpType,
    pub loc: Loc,
}

// custom statements
#[derive(Debug, PartialEq, Clone)]
pub struct VarDeclStmt<'a> {
    pub name: &'a str,
    pub value: Option<Expr<'a>>,
    pub is_const: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStmt<'a> {
    pub value: Option<Expr<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStmt<'a> {
    pub condition: Expr<'a>,
    pub then: Block<'a>,
    pub elif_branch: Vec<ElifStmt<'a>>,
    pub else_branch: Option<Block<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElifStmt<'a> {
    pub condition: Expr<'a>,
    pub then: Block<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclStmt<'a> {
    pub name: &'a str,
    pub params: Vec<Param<'a>>,
    pub return_type: Option<&'a str>,
    pub body: Block<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Param<'a> {
    pub name: &'a str,
    pub param_type: &'a str,
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

impl Expr<'_> {
    pub fn loc(&self) -> Loc {
        match self {
            Expr::Binary(expr) => expr.loc.clone(),
            Expr::Unary(expr) => expr.loc.clone(),
            Expr::Literal(expr) => expr.loc.clone(),
            Expr::Identifier(expr) => expr.loc.clone(),
            Expr::Call(expr) => expr.loc.clone(),
        }
    }
}

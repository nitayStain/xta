use std::collections::HashMap;

use xta_lexer::token::Loc;
use xta_parser::ast::{FunctionDeclStmt, Stmt};

pub struct Analyzer<'ctx> {
    functions: HashMap<&'ctx str, FunctionDeclStmt<'ctx>>,
    errors: Vec<Error>
}

impl<'ctx> Analyzer<'ctx> {
    pub fn new(stmts: Vec<Stmt<'ctx>>) -> Self {
        // TODO: make analyzer run on the given statements,
        // create an AST node called file, and give it statements to map
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("~ ({loc}) : Unexpected non-function statement")]
    UnexpectedNonFunction { loc: Loc },
    // #[error("~ ({loc}) : Expected `{expected}`, found `{found}`")]
    // Expected { loc: Loc, expected: Token<'a>, found: Token<'a> },

    // #[error("~ ({loc}) : Expected an identifier, found `{found}`")]
    // ExpectedId {loc: Loc, found: Token<'a> }
}

use std::collections::HashMap;

use xta_parser::ast::IdentifierExpr;

struct Var<'ctx> {
    name: IdentifierExpr<'ctx>,
}

struct Scope<'ctx> {
    vars: HashMap<&'ctx str, Var<'ctx>>,
}

impl<'ctx> Scope<'ctx> {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    fn insert_var(&mut self, name: IdentifierExpr<'ctx>) {
        self.vars.insert(name.name, Var { name });
    }

    fn get_var(&self, name: &str) -> Option<&Var<'ctx>> {
        self.vars.get(name)
    }
}
use crate::il::expressions::Expression;

mod evaluations;
mod label;

pub struct X86_64 {
    scopes: Vec<Scope>,
}
pub struct Scope {
    stack: Vec<Symbol>,
}

pub struct Symbol {
    name: String,
    size: usize,
}

impl X86_64 {
    pub fn init() -> Self {
        Self { scopes: vec![] }
    }
    pub fn compile(&mut self, expression: &Expression) -> String {
        match expression {
            Expression::Constant(value) => value.clone(),
            Expression::Label(label) => self.label(label),
            Expression::Result(eval) => self.evaluation(eval),
        }
    }
}

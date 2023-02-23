use crate::intermediate::expressions::Expression;

mod evaluations;
mod label;
mod statements;

#[derive(Debug)]
pub struct X86_64 {
    scopes: Vec<Scope>,
}
#[derive(Debug)]
pub struct Scope {
    stack: Vec<Symbol>,
}
#[derive(Debug)]
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
            Expression::Statement(statement) => self.statement(statement),
            Expression::Set(_) => panic!("Sets cannot be compiled"),
        }
    }
}

use crate::parser::expressions::Expression;

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
            Expression::Constant(value) => format!(
                "\
mov rax, {value}"
            ),
            Expression::Label(label) => {
                let label = self.label(label);
                format!(
                    "\
mov rax, {label}"
                )
            }
            Expression::Result(eval) => self.evaluation(eval),
            Expression::Statement(statement) => self.statement(statement),
            Expression::Unit => format!(
                "\
mov rax, 0"
            ),
            Expression::Set(_) => panic!("Sets cannot be compiled"),
        }
    }
}

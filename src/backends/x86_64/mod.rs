use crate::parser::tokens::{Token, Value};

mod declaration;
mod expressions;
mod label;
mod statements;
mod value;

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
    pub fn compile(&mut self, expression: &Token) -> String {
        match expression {
            Token::Value(value) => match value {
                Value::Constant(value) => format!(
                    "\
    mov rax, {value}"
                ),
                Value::Label(label) => {
                    let label = self.label(label);
                    format!(
                        "\
mov rax, {label}"
                    )
                }
                Value::Unit => format!(
                    "\
mov rax, 0"
                ),
                Value::Result(result) => self.expression(&*result),
            },
            Token::Statement(statement) => self.statement(statement),
            Token::Set(_) => panic!("Sets cannot be compiled"),
            Token::Declaration(_) => panic!("Declarations cannot be compiled"),
        }
    }
}

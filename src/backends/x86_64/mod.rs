use crate::parser::tokens::{Token, Value};

mod declaration;
mod definitions;
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
    pub fn compile(&mut self, tokens: Vec<Token>) -> String {
        tokens
            .iter()
            .map(|t| match t {
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
                Token::Definition(definition) => self.definition(definition),
                Token::Statement(statement) => self.statement(statement),
                Token::Set(_) | Token::Declaration(_) => {
                    panic!("Token cannot be compiled")
                }
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

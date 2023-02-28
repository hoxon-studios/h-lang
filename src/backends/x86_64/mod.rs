use crate::parser::tokens::Token;

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
                Token::Value(value) => self.value(value),
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

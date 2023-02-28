use crate::parser::tokens::Token;

mod declaration;
mod definitions;
mod expressions;
mod label;
mod pointer;
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
    _type: SymbolType,
}
impl Symbol {
    pub fn size(&self) -> usize {
        match self._type {
            SymbolType::Value(size) => size,
            SymbolType::Pointer(_) => USIZE,
        }
    }
}

#[derive(Debug)]
pub enum SymbolType {
    Value(usize),
    Pointer(usize),
}

const USIZE: usize = 8;

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

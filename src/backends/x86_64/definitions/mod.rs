use crate::parser::tokens::Definition;

use super::X86_64;

pub mod function;

impl X86_64 {
    pub fn definition(&mut self, definition: &Definition) -> String {
        match definition {
            Definition::Function(function) => self.function(function),
        }
    }
}

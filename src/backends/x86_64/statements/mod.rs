use crate::intermediate::expressions::Statement;

mod block;
mod let_statement;

use super::X86_64;

impl X86_64 {
    pub fn statement(&mut self, statement: &Statement) -> String {
        match statement {
            Statement::Let(let_statement) => self.let_statement(let_statement),
            Statement::Block(block) => self.block(block),
        }
    }
}

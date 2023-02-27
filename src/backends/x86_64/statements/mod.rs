mod assignment;

use crate::parser::tokens::Statement;

use super::X86_64;

impl X86_64 {
    pub fn statement(&mut self, statement: &Statement) -> String {
        match statement {
            Statement::Assignment(assignment) => self.assignment(assignment),
        }
    }
}

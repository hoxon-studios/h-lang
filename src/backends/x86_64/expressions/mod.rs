use crate::parser::tokens::Expression;

use super::X86_64;

pub mod addition;
pub mod block;
pub mod dereference;
pub mod function_call;

impl X86_64 {
    pub fn expression(&mut self, expression: &Expression) -> String {
        match expression {
            Expression::Addition(addition) => self.addition(addition),
            Expression::Block(block) => self.block(block, true),
            Expression::FunctionCall(function_call) => self.function_call(function_call),
            Expression::Dereference(dereference) => self.dereference(dereference),
        }
    }
}

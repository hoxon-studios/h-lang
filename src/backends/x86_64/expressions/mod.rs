use crate::parser::tokens::Expression;

use super::X86_64;

mod addition;
mod block;
mod function_call;

impl X86_64 {
    pub fn expression(&mut self, expression: &Expression) -> String {
        match expression {
            Expression::Addition(addition) => self.addition(addition),
            Expression::Block(block) => self.block(block),
            Expression::FunctionCall(function_call) => self.function_call(function_call),
        }
    }
}

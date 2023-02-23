use crate::intermediate::expressions::Evaluation;

use super::X86_64;

mod addition;
mod block;
mod function_call;

impl X86_64 {
    pub fn evaluation(&mut self, evaluation: &Evaluation) -> String {
        match evaluation {
            Evaluation::Addition(addition) => self.addition(addition),
            Evaluation::Block(block) => self.block(block),
            Evaluation::FunctionCall(function_call) => todo!(),
        }
    }
}

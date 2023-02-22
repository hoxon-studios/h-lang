use crate::il::expressions::Evaluation;

use super::X86_64;

mod addition;

impl X86_64 {
    pub fn evaluation(&self, evaluation: &Evaluation) -> String {
        match evaluation {
            Evaluation::Addition(addition) => self.addition(addition),
        }
    }
}

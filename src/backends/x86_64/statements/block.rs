use crate::{backends::x86_64::X86_64, intermediate::expressions::Block};

impl X86_64 {
    pub fn block(&mut self, block: &Block) -> String {
        block
            .0
            .iter()
            .map(|statement| self.statement(statement))
            .collect::<Vec<String>>()
            .concat()
    }
}

use crate::{backends::x86_64::X86_64, intermediate::expressions::Block};

impl X86_64 {
    pub fn block(&mut self, block: &Block) -> String {
        let mut body = block
            .body
            .iter()
            .map(|statement| self.statement(statement))
            .collect::<Vec<String>>();

        if let Some(result) = &block.result {
            body.push(self.compile(&result));
        }

        return body.concat();
    }
}

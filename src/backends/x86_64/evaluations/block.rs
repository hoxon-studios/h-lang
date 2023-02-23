use crate::{
    backends::x86_64::X86_64,
    intermediate::expressions::{Block, Expression},
};

impl X86_64 {
    pub fn block(&mut self, block: &Block) -> String {
        let mut body = block
            .body
            .iter()
            .map(|statement| self.statement(statement))
            .collect::<Vec<String>>();

        match &block.result {
            Expression::Unit => {}
            _ => {
                body.push(self.compile(&block.result));
            }
        }

        return body.concat();
    }
}

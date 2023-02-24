use crate::{
    backends::x86_64::{Scope, X86_64},
    intermediate::expressions::{Block, Expression},
};

impl X86_64 {
    pub fn block(&mut self, block: &Block) -> String {
        self.scopes.push(Scope { stack: vec![] });

        let mut body = block
            .body
            .iter()
            .map(|statement| self.statement(statement))
            .filter(|s| s != "")
            .collect::<Vec<String>>();

        match &block.result {
            Expression::Unit => {}
            _ => {
                body.push(self.compile(&block.result));
            }
        }

        let body = body.join("\n");
        let stack_size: usize = self
            .scopes
            .pop()
            .expect("Scope not found")
            .stack
            .iter()
            .map(|s| s.size)
            .sum();

        format!(
            "\
sub rsp, {stack_size}
{body}
add rsp, {stack_size}"
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{backends::x86_64::X86_64, frontend::tokenize, intermediate::parse};

    #[test]
    fn it_compiles_block() {
        let code = "let some_value; let another; some_value + 2";
        let expression = parse(tokenize(code).unwrap()).unwrap();
        // ACT
        let result = X86_64::init().compile(&expression);
        // ASSERT
        assert_eq!(
            result,
            "\
sub rsp, 16
mov rax, QWORD[rbp - 8]
add rax, 2
add rsp, 16"
        )
    }
}

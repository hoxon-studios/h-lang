use crate::{
    backends::x86_64::{Scope, X86_64},
    parser::tokens::Block,
};

impl X86_64 {
    pub fn block(&mut self, block: &Block, own_scope: bool) -> String {
        if own_scope {
            self.scopes.push(Scope { stack: vec![] });
        }

        let mut body = block
            .body
            .iter()
            .map(|statement| self.statement(statement))
            .filter(|s| s != "")
            .collect::<Vec<String>>();

        body.push(self.value(&block.result));

        let body = body.join("\n");

        if own_scope {
            let stack_size: usize = self
                .scopes
                .pop()
                .expect("Scope not found")
                .stack
                .iter()
                .map(|s| s.size())
                .sum();

            format!(
                "\
sub rsp, {stack_size}
{body}
add rsp, {stack_size}"
            )
        } else {
            format!(
                "\
{body}"
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{backends::x86_64::X86_64, parser::Parser};

    #[test]
    fn it_compiles_block() {
        let code = "some_value: usize = 1; another: usize = 2; some_value + 2";
        let tokens = Parser::parse(code).unwrap();
        // ACT
        let result = X86_64::init().compile(tokens);
        // ASSERT
        assert_eq!(
            result,
            "\
sub rsp, 16
mov QWORD[rbp - 8], 1
mov QWORD[rbp - 16], 2
mov rax, QWORD[rbp - 8]
add rax, 2
add rsp, 16"
        )
    }
}

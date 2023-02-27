use crate::{
    backends::x86_64::{
        expressions::function_call::SYSTEM_V_AMD64_ABI_CONVENTION, Scope, Symbol, X86_64,
    },
    parser::tokens::Function,
};

impl X86_64 {
    pub fn function(&mut self, function: &Function) -> String {
        let symbols = function
            .parameters
            .iter()
            .map(|p| Symbol {
                name: p.label.to_string(),
                size: 8,
            })
            .collect::<Vec<Symbol>>();
        self.scopes.push(Scope { stack: symbols });

        let parameters = function
            .parameters
            .iter()
            .zip(SYSTEM_V_AMD64_ABI_CONVENTION)
            .map(|(p, reg)| {
                let label = self.label(p.label);
                format!("mov {label}, {reg}")
            })
            .collect::<Vec<String>>()
            .join("\n");

        let label = function.label;
        let body = self.block(&function.body, false);
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
{label}:
push rbp
mov rbp, rsp
sub rsp, {stack_size}
{parameters}
{body}
add rsp, {stack_size}
pop rbp
ret"
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{backends::x86_64::X86_64, parser::parse};

    #[test]
    fn it_compiles_a_function_definition() {
        let code = "\
            fn some(x: usize, y: usize)
                a: usize = 3;
                a + x + y";
        let token = parse(code).unwrap();
        // ACT
        let result = X86_64::init().compile(&token);
        // ASSERT
        assert_eq!(
            result,
            "\
some:
push rbp
mov rbp, rsp
sub rsp, 24
mov QWORD[rbp - 8], rdi
mov QWORD[rbp - 16], rsi
mov QWORD[rbp - 24], 3
mov rax, QWORD[rbp - 24]
add rax, QWORD[rbp - 8]
add rax, QWORD[rbp - 16]
add rsp, 24
pop rbp
ret"
        )
    }
}

use crate::{
    backends::x86_64::X86_64,
    parser::tokens::{Dereference, Value},
};

impl X86_64 {
    pub fn dereference(&mut self, dereference: &Dereference) -> String {
        let label = dereference.label;
        match dereference.index.as_ref() {
            Value::Constant(index) => {
                let size = self.pointer_size(label);
                let label = self.label(label);
                format!(
                    "\
mov rax, {label}
mov rax, QWORD[rax + {index} * {size}]"
                )
            }
            Value::Label(index) => {
                let size = self.pointer_size(label);
                let label = self.label(label);
                let index = self.label(index);
                format!(
                    "\
mov rax, {index}
imul rax, {size}
add rax, {label}
mov rax, QWORD[rax]"
                )
            }
            Value::Result(index) => {
                let index = self.expression(index);
                let size = self.pointer_size(label);
                let label = self.label(label);
                format!(
                    "\
{index}
imul rax, {size}
add rax, {label}
mov rax, QWORD[rax]"
                )
            }
            Value::Reference(_) | Value::Unit => {
                panic!("Invalid operand")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{backends::x86_64::X86_64, parser::Parser};

    #[test]
    fn it_compiles_dereference() {
        let code = "data: usize = 10; pointer: &usize = &data; pointer#0";
        let tokens = Parser::parse(code).unwrap();
        // ACT
        let result = X86_64::init().compile(tokens);
        // ASSERT
        assert_eq!(
            result,
            "\
sub rsp, 16
mov QWORD[rbp - 8], 10
lea rax, QWORD[rbp - 8]
mov QWORD[rbp - 16], rax
mov rax, QWORD[rbp - 16]
mov rax, QWORD[rax + 0 * 8]
add rsp, 16"
        );
    }

    #[test]
    fn it_compiles_dereference_of_reference() {
        let code = "public fn some(x: &usize, y: usize) pointer: &usize = x; pointer#0";
        let tokens = Parser::parse(code).unwrap();
        // ACT
        let result = X86_64::init().compile(tokens);
        // ASSERT
        assert_eq!(
            result,
            "\
global some
some:
push rbp
mov rbp, rsp
sub rsp, 24
mov QWORD[rbp - 8], rdi
mov QWORD[rbp - 16], rsi
mov rax, QWORD[rbp - 8]
mov QWORD[rbp - 24], rax
mov rax, QWORD[rbp - 24]
mov rax, QWORD[rax + 0 * 8]
add rsp, 24
pop rbp
ret"
        )
    }
}

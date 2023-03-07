use crate::parser::{tokens::Token, Parser};

impl<'a> Parser<'a> {
    pub fn parse_dereference(&mut self) {
        let Some(index) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(Token::Label(label)) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let result = match index {
            Token::Constant(index) => {
                let size = self.context.pointer_size(label);
                let label = self.context.address(label);
                format!(
                    "\
mov rax, {label}
mov rax, QWORD[rax + {index} * {size}]"
                )
            }
            Token::Label(index) => {
                let size = self.context.pointer_size(label);
                let label = self.context.address(label);
                let index = self.context.address(index);
                format!(
                    "\
mov rax, {index}
imul rax, {size}
add rax, {label}
mov rax, QWORD[rax]"
                )
            }
            Token::Result(index) => {
                let size = self.context.pointer_size(label);
                let label = self.context.address(label);
                format!(
                    "\
{index}
imul rax, {size}
add rax, {label}
mov rax, QWORD[rax]"
                )
            }
            _ => panic!("Invalid operand"),
        };

        self.output.push(Token::Result(result));
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn it_compiles_dereference() {
        let code = "data: usize = 10; pointer: usize:ptr = &data; pointer#0";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov QWORD[rbp - 8], 10
lea rax, QWORD[rbp - 8]
mov QWORD[rbp - 16], rax
mov rax, QWORD[rbp - 16]
mov rax, QWORD[rax + 0 * 8]"
        );
    }

    #[test]
    fn it_compiles_dereference_of_reference() {
        let code = "public fn some(x: usize:ptr, y: usize) pointer: usize:ptr = x; pointer#0";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
global some
segment .text
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

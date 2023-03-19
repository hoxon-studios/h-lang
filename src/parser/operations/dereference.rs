use crate::parser::{
    tokens::{Code, Constant, Token},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_dereference(&mut self) {
        let Some(index) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(label) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let label = self.context.resolve(label);
        let index = self.context.resolve(index);

        if let Token::Label(label) = label {
            let result = match index {
                Token::Constant(Constant(index)) => {
                    let size = self.context.pointer_size(label.id);
                    let address = label.to_address();
                    format!(
                        "\
mov rax, {address}
mov rax, QWORD[rax + {index} * {size}]"
                    )
                }
                Token::Label(index) => {
                    let size = self.context.pointer_size(label.id);
                    let address = label.to_address();
                    let index = index.to_address();
                    format!(
                        "\
mov rax, {index}
imul rax, {size}
add rax, {address}
mov rax, QWORD[rax]"
                    )
                }
                Token::Result(Code(index)) => {
                    let size = self.context.pointer_size(label.id);
                    let address = label.to_address();
                    format!(
                        "\
{index}
imul rax, {size}
add rax, {address}
mov rax, QWORD[rax]"
                    )
                }
                _ => panic!("Invalid operand"),
            };

            self.output.push(Token::Result(Code(result)));
        } else {
            panic!("Invalid operand")
        }
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

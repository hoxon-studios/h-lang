use crate::parser::{tokens::Token, Parser};

impl<'a> Parser<'a> {
    pub fn parse_addition(&mut self) {
        let Some(right) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(left) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let result = match left {
            Token::Constant(left) => match right {
                Token::Constant(right) => format!(
                    "\
mov rax, {left} + {right}"
                ),
                Token::Label(right) => {
                    let right = self.context.address(right);
                    format!(
                        "\
mov rax, {right}
add rax, {left}"
                    )
                }
                Token::Result(right) => {
                    format!(
                        "\
{right}
add rax, {left}"
                    )
                }
                _ => panic!("Invalid operand"),
            },
            Token::Label(left) => match right {
                Token::Constant(right) => {
                    let left = self.context.address(left);
                    format!(
                        "\
mov rax, {left}
add rax, {right}"
                    )
                }
                Token::Label(right) => {
                    let left = self.context.address(left);
                    let right = self.context.address(right);
                    format!(
                        "\
mov rax, {left}
add rax, {right}"
                    )
                }
                Token::Result(right) => {
                    let left = self.context.address(left);
                    format!(
                        "\
{right}
add rax, {left}"
                    )
                }
                _ => panic!("Invalid operand"),
            },
            Token::Result(left) => match right {
                Token::Constant(right) => format!(
                    "\
{left}
add rax, {right}"
                ),
                Token::Label(right) => {
                    let right = self.context.address(right);
                    format!(
                        "\
{left}
add rax, {right}"
                    )
                }
                Token::Result(right) => format!(
                    "\
{left}
push rax
{right}
pop rdx
add rax, rdx"
                ),
                _ => panic!("Invalid operand"),
            },
            _ => panic!("Invalid operand"),
        };

        self.output.push(Token::Result(result));
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn it_compiles_addition_between_two_constants() {
        let code = "1 + 2";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov rax, 1 + 2"
        );
    }

    #[test]
    fn it_compiles_addition_between_constant_and_label() {
        let code = "some_label: usize = 1; 1 + some_label";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov QWORD[rbp - 8], 1
mov rax, QWORD[rbp - 8]
add rax, 1"
        );
    }

    #[test]
    fn it_compiles_addition_between_constant_and_result() {
        let code = "1 + (2 + 3)";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov rax, 2 + 3
add rax, 1"
        );
    }

    #[test]
    fn it_compiles_addition_between_label_and_result() {
        let code = "some_label: usize = 2; some_label + (2 + 3)";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov QWORD[rbp - 8], 2
mov rax, 2 + 3
add rax, QWORD[rbp - 8]"
        );
    }

    #[test]
    fn it_compiles_addition_between_two_labels() {
        let code = "label1: usize = 3; label2: usize = 2; label1 + label2";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov QWORD[rbp - 8], 3
mov QWORD[rbp - 16], 2
mov rax, QWORD[rbp - 8]
add rax, QWORD[rbp - 16]"
        )
    }

    #[test]
    fn it_compiles_addition_between_two_results() {
        let code = "(1 + 2) + (3 + 4)";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov rax, 1 + 2
push rax
mov rax, 3 + 4
pop rdx
add rax, rdx"
        )
    }
}

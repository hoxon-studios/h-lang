use crate::parser::{tokens::Token, Parser};

impl<'a> Parser<'a> {
    pub fn parse_bitwise_or(&mut self) {
        let Some(left) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(right) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let left = self.context.resolve(left);
        let right = self.context.resolve(right);

        let result = match left {
            Token::Constant(left) => match right {
                Token::Constant(right) => format!(
                    "\
mov rax, {left} | {right}"
                ),
                Token::Label(right) => {
                    let right = right.to_address();
                    format!(
                        "\
mov rax, {right}
or rax, {left}"
                    )
                }
                Token::Result(right) => format!(
                    "\
{right}
or rax, {left}"
                ),
                _ => panic!("Invalid operand"),
            },
            Token::Label(left) => match right {
                Token::Label(right) => {
                    let left = left.to_address();
                    let right = right.to_address();
                    format!(
                        "\
mov rax, {left}
or rax, {right}"
                    )
                }
                Token::Result(right) => {
                    let left = left.to_address();
                    format!(
                        "\
{right}
or rax, {left}"
                    )
                }
                Token::Constant(right) => {
                    let left = left.to_address();
                    format!(
                        "\
mov rax, {left}
or rax, {right}"
                    )
                }
                _ => panic!("Invalid operand"),
            },
            Token::Result(left) => match right {
                Token::Label(right) => {
                    let right = right.to_address();
                    format!(
                        "\
{left}
or rax, {right}"
                    )
                }
                Token::Result(right) => format!(
                    "\
{left}
push rax
{right}
pop rdx
or rax, rdx"
                ),
                Token::Constant(right) => format!(
                    "\
{left}
or rax, {right}"
                ),
                _ => panic!("Invalid operand"),
            },
            _ => panic!("Invalid operand"),
        };

        self.output.push(Token::Result(result));
    }
}

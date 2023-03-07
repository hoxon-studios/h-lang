use crate::parser::{tokens::Token, Parser};

impl<'a> Parser<'a> {
    pub fn parse_bitwise_or(&mut self) {
        let Some(left) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(right) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let result = match left {
            Token::Constant(left) => match right {
                Token::Constant(right) => format!(
                    "\
mov rax, {left} | {right}"
                ),
                Token::Label(right) => {
                    let right = self.context.address(right);
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
                Token::String(_)
                | Token::Set(_)
                | Token::Statement { .. }
                | Token::Unit
                | Token::Item { .. } => {
                    panic!("Invalid operand")
                }
            },
            Token::Label(left) => match right {
                Token::Label(right) => {
                    let left = self.context.address(left);
                    let right = self.context.address(right);
                    format!(
                        "\
mov rax, {left}
or rax, {right}"
                    )
                }
                Token::Result(right) => {
                    let left = self.context.address(left);
                    format!(
                        "\
{right}
or rax, {left}"
                    )
                }
                Token::Constant(right) => {
                    let left = self.context.address(left);
                    format!(
                        "\
mov rax, {left}
or rax, {right}"
                    )
                }
                Token::String(_)
                | Token::Set(_)
                | Token::Statement { .. }
                | Token::Unit
                | Token::Item { .. } => {
                    panic!("Invalid operand")
                }
            },
            Token::Result(left) => match right {
                Token::Label(right) => {
                    let right = self.context.address(right);
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
                Token::String(_)
                | Token::Set(_)
                | Token::Statement { .. }
                | Token::Unit
                | Token::Item { .. } => {
                    panic!("Invalid operand")
                }
            },
            Token::String(_)
            | Token::Set(_)
            | Token::Statement { .. }
            | Token::Unit
            | Token::Item { .. } => {
                panic!("Invalid operand")
            }
        };

        self.output.push(Token::Result(result));
    }
}

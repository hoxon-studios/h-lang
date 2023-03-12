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

        let result = match (left, right) {
            (Token::Constant(left), Token::Constant(right)) => format!(
                "\
mov rax, {left} | {right}"
            ),
            (Token::Constant(constant), Token::Label(label))
            | (Token::Label(label), Token::Constant(constant)) => {
                let label = label.to_address();
                format!(
                    "\
mov rax, {label}
or rax, {constant}"
                )
            }
            (Token::Constant(constant), Token::Result(result))
            | (Token::Result(result), Token::Constant(constant)) => format!(
                "\
{result}
or rax, {constant}"
            ),
            (Token::Label(left), Token::Label(right)) => {
                let left = left.to_address();
                let right = right.to_address();
                format!(
                    "\
mov rax, {left}
or rax, {right}"
                )
            }
            (Token::Label(label), Token::Result(result))
            | (Token::Result(result), Token::Label(label)) => {
                let label = label.to_address();
                format!(
                    "\
{result}
or rax, {label}"
                )
            }
            (Token::Result(left), Token::Result(right)) => format!(
                "\
{left}
push rax
{right}
pop rdx
or rax, rdx"
            ),
            _ => panic!("Invalid operands"),
        };

        self.output.push(Token::Result(result));
    }
}

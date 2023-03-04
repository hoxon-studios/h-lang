use crate::parser::{tokens::Token, Parser};

impl<'a> Parser<'a> {
    pub fn parse_block(&mut self) {
        let Some(right) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(Token::Instruction(left)) = self.output.pop() else {
            panic!("Invalid operand")
        };

        match right {
            Token::Instruction(right) => self.output.push(Token::Instruction(format!(
                "\
{left}
{right}"
            ))),
            Token::Result(right) => self.output.push(Token::Result(format!(
                "\
{left}
{right}"
            ))),
            Token::Constant(right) => self.output.push(Token::Result(format!(
                "\
{left}
mov rax, {right}"
            ))),
            Token::Label(right) => {
                let right = self.context.address(right);
                self.output.push(Token::Result(format!(
                    "\
{left}
mov rax, {right}"
                )))
            }
            Token::Unit => self.output.push(Token::Result(format!(
                "\
{left}
mov rax, 0"
            ))),
            _ => panic!("Invalid operand"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn it_compiles_block() {
        let code = "some_value: usize = 1; another: usize = 2; some_value + 2";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov QWORD[rbp - 8], 1
mov QWORD[rbp - 16], 2
mov rax, QWORD[rbp - 8]
add rax, 2"
        );
    }
}

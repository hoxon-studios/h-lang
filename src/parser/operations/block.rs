use crate::parser::{
    tokens::{Code, Constant, Statement, Token},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_block(&mut self) {
        let Some(right) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(left) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let left = self.context.resolve(left);
        let right = self.context.resolve(right);

        match left {
            Token::Result(Code(left)) => match right {
                Token::Statement(Statement {
                    body: Code(right), ..
                }) => self.output.push(Token::Statement(Statement {
                    body: Code(format!(
                        "\
{left}
{right}"
                    )),
                    exit_label: None,
                })),
                Token::Result(Code(right)) => self.output.push(Token::Result(Code(format!(
                    "\
{left}
{right}"
                )))),
                Token::Constant(Constant(right)) => self.output.push(Token::Result(Code(format!(
                    "\
{left}
mov rax, {right}"
                )))),
                Token::Label(right) => {
                    let right = right.to_address();
                    self.output.push(Token::Result(Code(format!(
                        "\
{left}
mov rax, {right}"
                    ))))
                }
                Token::Unit => self.output.push(Token::Result(Code(format!(
                    "\
{left}
mov rax, 0"
                )))),
                _ => panic!("Invalid operand"),
            },
            Token::Statement(Statement {
                body: Code(left), ..
            }) => match right {
                Token::Statement(Statement {
                    body: Code(right), ..
                }) => self.output.push(Token::Statement(Statement {
                    body: Code(format!(
                        "\
{left}
{right}"
                    )),
                    exit_label: None,
                })),
                Token::Result(Code(right)) => self.output.push(Token::Result(Code(format!(
                    "\
{left}
{right}"
                )))),
                Token::Constant(Constant(right)) => self.output.push(Token::Result(Code(format!(
                    "\
{left}
mov rax, {right}"
                )))),
                Token::Label(right) => {
                    let right = right.to_address();
                    self.output.push(Token::Result(Code(format!(
                        "\
{left}
mov rax, {right}"
                    ))))
                }
                Token::Unit => self.output.push(Token::Result(Code(format!(
                    "\
{left}
mov rax, 0"
                )))),
                _ => panic!("Invalid operand"),
            },
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

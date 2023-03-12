use crate::parser::{tokens::Token, Parser};

impl<'a> Parser<'a> {
    pub fn parse_assignment(&mut self) {
        let Some(value) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(address) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let id = match address {
            Token::Id(id) => self.context.label(id).to_address(),
            Token::Label(label) => label.to_address(),
            Token::Result(_) => format!("QWORD[rax]"),
            _ => panic!("Invalid operand"),
        };

        let value = self.context.resolve(value);
        let body = match value {
            Token::Constant(value) => format!(
                "\
mov {id}, {value}"
            ),
            Token::Label(value) => {
                let value = value.to_address();
                format!(
                    "\
mov rax, {value}
mov {id}, rax"
                )
            }
            Token::Result(value) => format!(
                "\
{value}
mov {id}, rax"
            ),
            _ => panic!("Invalid operand"),
        };

        self.output.push(Token::Statement {
            body,
            exit_label: None,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn it_compiles_assignment() {
        let code = "some_var: usize = 1";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov QWORD[rbp - 8], 1"
        );
    }
}

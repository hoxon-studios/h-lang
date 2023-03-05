use crate::parser::{tokens::Token, Parser};

impl<'a> Parser<'a> {
    pub fn parse_assignment(&mut self) {
        let Some(value) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(Token::Label(label)) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let label = self.context.address(label);
        let body = match value {
            Token::Constant(value) => format!(
                "\
mov {label}, {value}"
            ),
            Token::Label(value) => {
                let value = self.context.address(value);
                format!(
                    "\
mov rax, {value}
mov {label}, rax"
                )
            }
            Token::Result(value) => format!(
                "\
{value}
mov {label}, rax"
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
        let code = "some_var = 1";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov some_var, 1"
        );
    }
}

use crate::parser::{tokens::Token, Parser};

impl<'a> Parser<'a> {
    pub fn parse_loop(&mut self) {
        let Some(Token::Statement { body, .. }) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let start_label = self.context.take_label("L_START");
        let end_label = self.context.take_label("L_EXIT");
        let body = body.replace("{break}", &end_label);
        let result = format!(
            "\
{start_label}:
{body}
jmp {start_label}
{end_label}:"
        );

        self.output.push(Token::Statement {
            body: result,
            exit_label: None,
        });
    }

    pub fn parse_break(&mut self) {
        let result = format!("jmp {{break}}");
        self.output.push(Token::Statement {
            body: result,
            exit_label: None,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn it_parses_loop_statement() {
        let code = "\
            loop (
                x: usize = 1;
                if x ( break )
            )";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
.L_START_2:
mov QWORD[rbp - 8], 1
cmp QWORD[rbp - 8], 0
je .C_NEXT_1
jmp .L_EXIT_3
jmp .C_EXIT_0
.C_NEXT_1:
.C_EXIT_0:
jmp .L_START_2
.L_EXIT_3:"
        )
    }
}

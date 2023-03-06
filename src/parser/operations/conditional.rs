use crate::parser::{tokens::Token, Parser};

impl<'a> Parser<'a> {
    pub fn parse_if_conditional(&mut self) {
        let Some(body) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(condition) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let body = match body {
            Token::Statement { body, .. } => body,
            Token::Unit
            | Token::Constant(_)
            | Token::Label(_)
            | Token::Result(_)
            | Token::Set(_)
            | Token::Item { .. } => {
                panic!("Invalid operand")
            }
        };
        let exit_label = self.context.take_label("C_EXIT");
        let next_label = self.context.take_label("C_NEXT");
        let condition = match condition {
            Token::Constant(condition) => format!(
                "\
cmp {condition}, 0"
            ),
            Token::Label(condition) => {
                let condition = self.context.address(condition);
                format!(
                    "\
cmp {condition}, 0"
                )
            }
            Token::Result(condition) => format!(
                "\
{condition}
cmp rax, 0"
            ),
            Token::Statement { .. } | Token::Set(_) | Token::Item { .. } | Token::Unit => {
                panic!("Invalid operand")
            }
        };

        let body = format!(
            "\
{condition}
je {next_label}
{body}
jmp {exit_label}
{next_label}:
{exit_label}:"
        );

        self.output.push(Token::Statement {
            body,
            exit_label: Some(exit_label),
        });
    }

    pub fn parse_else_conditional(&mut self) {
        let Some(Token::Statement { body: right, exit_label: right_exit }) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(Token::Statement { body: left, exit_label: Some(left_exit) }) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let body = match &right_exit {
            Some(right_exit) => left
                .replace(&format!("{left_exit}:"), &right)
                .replace(&left_exit, &right_exit),
            None => {
                let body = left.replace(&format!("{left_exit}:"), &right);
                format!(
                    "\
{body}
{left_exit}:",
                )
            }
        };

        self.output.push(Token::Statement {
            body,
            exit_label: right_exit.clone(),
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn it_parses_conditional() {
        let code = "\
            fn check(value: usize)
                result: usize = 0;
                if 0 result = 1 else if 1 result = 2 else result = 3;
                result";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
check:
push rbp
mov rbp, rsp
sub rsp, 16
mov QWORD[rbp - 8], rdi
mov QWORD[rbp - 16], 0
cmp 0, 0
je .C_NEXT_1
mov QWORD[rbp - 16], 1
jmp .C_EXIT_2
.C_NEXT_1:
cmp 1, 0
je .C_NEXT_3
mov QWORD[rbp - 16], 2
jmp .C_EXIT_2
.C_NEXT_3:
mov QWORD[rbp - 16], 3
.C_EXIT_2:
mov rax, QWORD[rbp - 16]
add rsp, 16
pop rbp
ret"
        );
    }
}

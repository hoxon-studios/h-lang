use crate::parser::{
    tokens::{Label, Token},
    Parser,
};

use super::call::SYSTEM_V_AMD64_ABI_CONVENTION;

impl<'a> Parser<'a> {
    pub fn parse_function(&mut self) {
        let Some(body) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(parameters) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(Token::Id(id)) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let body = self.context.resolve(body);

        let body = match body {
            Token::Result(body) => body,
            Token::Statement { body, .. } => body,
            Token::Label(body) => {
                let body = body.to_address();
                format!(
                    "\
mov rax, {body}"
                )
            }
            Token::Constant(body) => format!(
                "\
mov rax, {body}"
            ),
            Token::Unit => format!(
                "\
mov rax, 0"
            ),
            _ => panic!("Invalid operand"),
        };

        let parameters = match parameters {
            Token::Set(parameters) => parameters
                .into_iter()
                .map(|p| match p {
                    Token::Label(label) => label,
                    _ => panic!("Invalid operand"),
                })
                .collect::<Vec<Label>>(),
            Token::Unit => vec![],
            Token::Label(label) => vec![label],
            _ => panic!("Invalid operand"),
        };

        let parameters = parameters
            .iter()
            .zip(SYSTEM_V_AMD64_ABI_CONVENTION)
            .map(|(p, reg)| {
                let address = p.to_address();
                format!("mov {address}, {reg}")
            })
            .collect::<Vec<String>>()
            .join("\n");

        let stack_size: Option<usize> = self
            .context
            .scopes
            .pop()
            .map(|scope| scope.symbols.iter().map(|s| s._type.size()).sum());

        let result = match body.is_empty() {
            true => format!(
                "\
segment .text
{id}:
ret"
            ),
            false => match stack_size {
                Some(stack_size) => match parameters.is_empty() {
                    true => format!(
                        "\
segment .text
{id}:
push rbp
mov rbp, rsp
sub rsp, {stack_size}
{body}
add rsp, {stack_size}
pop rbp
ret"
                    ),
                    false => format!(
                        "\
segment .text
{id}:
push rbp
mov rbp, rsp
sub rsp, {stack_size}
{parameters}
{body}
add rsp, {stack_size}
pop rbp
ret"
                    ),
                },
                None => match parameters.is_empty() {
                    true => format!(
                        "\
segment .text
{id}:
push rbp
mov rbp, rsp
{body}
pop rbp
ret"
                    ),
                    false => format!(
                        "\
segment .text
{id}:
push rbp
mov rbp, rsp
{parameters}
{body}
pop rbp
ret"
                    ),
                },
            },
        };

        self.output.push(Token::Item {
            name: id,
            definition: result,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn it_compiles_a_function_definition() {
        let code = "\
            fn some(x: usize, y: usize)
                a: usize = 3;
                a + x + y";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
segment .text
some:
push rbp
mov rbp, rsp
sub rsp, 24
mov QWORD[rbp - 8], rdi
mov QWORD[rbp - 16], rsi
mov QWORD[rbp - 24], 3
mov rax, QWORD[rbp - 24]
add rax, QWORD[rbp - 8]
add rax, QWORD[rbp - 16]
add rsp, 24
pop rbp
ret"
        )
    }

    #[test]
    fn it_compiles_two_function_definitions() {
        let code = "\
            public fn some(x: usize, y: usize)
                x + y
            private fn power(x: usize)
                x + x";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
global some
segment .text
some:
push rbp
mov rbp, rsp
sub rsp, 16
mov QWORD[rbp - 8], rdi
mov QWORD[rbp - 16], rsi
mov rax, QWORD[rbp - 8]
add rax, QWORD[rbp - 16]
add rsp, 16
pop rbp
ret

segment .text
power:
push rbp
mov rbp, rsp
sub rsp, 8
mov QWORD[rbp - 8], rdi
mov rax, QWORD[rbp - 8]
add rax, QWORD[rbp - 8]
add rsp, 8
pop rbp
ret"
        )
    }
}

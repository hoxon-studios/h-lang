use crate::parser::{tokens::Token, Parser};

pub const LINUX_SYSCALL_CONVENTION: &[&'static str] =
    &["rax", "rdi", "rsi", "rdx", "r10", "r8", "r9"];
pub const SYSTEM_V_AMD64_ABI_CONVENTION: &[&'static str] =
    &["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

impl<'a> Parser<'a> {
    pub fn parse_call(&mut self) {
        let Some(expression) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(Token::Label(label)) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let parameters = match expression {
            Token::Set(set) => set,
            Token::Unit => vec![],
            _ => vec![expression],
        };

        let convention = match label {
            "syscall" => LINUX_SYSCALL_CONVENTION,
            _ => SYSTEM_V_AMD64_ABI_CONVENTION,
        };

        let evaluations = parameters
            .iter()
            .filter_map(|p| match p {
                Token::Result(result) => Some(format!(
                    "\
{result}
push rax"
                )),
                _ => None,
            })
            .collect::<Vec<String>>()
            .join("\n");

        let parameters = parameters
            .iter()
            .zip(convention.iter())
            .rev()
            .map(|(p, reg)| match p {
                Token::Constant(value) => format!(
                    "\
mov {reg}, {value}"
                ),
                Token::Label(value) => {
                    let value = self.context.address(value);
                    format!(
                        "\
mov {reg}, {value}"
                    )
                }
                Token::Result(_) => format!(
                    "\
pop {reg}"
                ),
                _ => panic!("Invalid operand"),
            })
            .collect::<Vec<String>>()
            .join("\n");

        let function_call = match label {
            "syscall" => "syscall".to_string(),
            _ => format!("call {}", &label),
        };

        let result = [evaluations, parameters, function_call]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join("\n");

        self.output.push(Token::Result(result));
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn it_compiles_function_call() {
        let code = "some_function$(1 + 2, 3, 4 + 5)";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov rax, 1 + 2
push rax
mov rax, 4 + 5
push rax
pop rdx
mov rsi, 3
pop rdi
call some_function"
        );
    }

    #[test]
    fn it_compiles_system_call() {
        let code = "syscall$(0x01, 0, message, length)";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov rdx, length
mov rsi, message
mov rdi, 0
mov rax, 0x01
syscall"
        )
    }
}

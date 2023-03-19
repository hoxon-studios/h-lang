use crate::parser::{
    tokens::{Code, Constant, Token},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_addition(&mut self) {
        let Some(right) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(left) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let left = self.context.resolve(left);
        let right = self.context.resolve(right);

        let result = match (left, right) {
            (Token::Constant(Constant(left)), Token::Constant(Constant(right))) => {
                format!(
                    "\
mov rax, {left} + {right}"
                )
            }
            (Token::Constant(Constant(constant)), Token::Label(label))
            | (Token::Label(label), Token::Constant(Constant(constant))) => {
                let label = label.to_address();
                format!(
                    "\
mov rax, {label}
add rax, {constant}"
                )
            }
            (Token::Constant(Constant(constant)), Token::Result(Code(result)))
            | (Token::Result(Code(result)), Token::Constant(Constant(constant))) => {
                format!(
                    "\
{result}
add rax, {constant}"
                )
            }
            (Token::Label(left), Token::Label(right)) => {
                let left = left.to_address();
                let right = right.to_address();
                format!(
                    "\
mov rax, {left}
add rax, {right}"
                )
            }
            (Token::Label(label), Token::Result(Code(result)))
            | (Token::Result(Code(result)), Token::Label(label)) => {
                let label = label.to_address();
                format!(
                    "\
{result}
add rax, {label}"
                )
            }
            (Token::Result(Code(left)), Token::Result(Code(right))) => {
                format!(
                    "\
{left}
push rax
{right}
pop rdx
add rax, rdx"
                )
            }
            _ => panic!("Invalid operands"),
        };

        self.output.push(Token::Result(Code(result)));
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn it_compiles_addition_between_two_constants() {
        let code = "1 + 2";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov rax, 1 + 2"
        );
    }

    #[test]
    fn it_compiles_addition_between_constant_and_label() {
        let code = "some_label: usize = 1; 1 + some_label";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov QWORD[rbp - 8], 1
mov rax, QWORD[rbp - 8]
add rax, 1"
        );
    }

    #[test]
    fn it_compiles_addition_between_constant_and_result() {
        let code = "1 + (2 + 3)";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov rax, 2 + 3
add rax, 1"
        );
    }

    #[test]
    fn it_compiles_addition_between_label_and_result() {
        let code = "some_label: usize = 2; some_label + (2 + 3)";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov QWORD[rbp - 8], 2
mov rax, 2 + 3
add rax, QWORD[rbp - 8]"
        );
    }

    #[test]
    fn it_compiles_addition_between_two_labels() {
        let code = "label1: usize = 3; label2: usize = 2; label1 + label2";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov QWORD[rbp - 8], 3
mov QWORD[rbp - 16], 2
mov rax, QWORD[rbp - 8]
add rax, QWORD[rbp - 16]"
        )
    }

    #[test]
    fn it_compiles_addition_between_two_results() {
        let code = "(1 + 2) + (3 + 4)";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
mov rax, 1 + 2
push rax
mov rax, 3 + 4
pop rdx
add rax, rdx"
        )
    }
}

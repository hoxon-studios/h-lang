use crate::{
    backends::x86_64::X86_64,
    intermediate::expressions::{Addition, Expression},
};

impl X86_64 {
    pub fn addition(&self, addition: &Addition) -> String {
        match &addition.left {
            Expression::Constant(left) => match &addition.right {
                Expression::Constant(right) => {
                    format!(
                        "\
mov rax, {left} + {right}"
                    )
                }
                Expression::Label(right) => {
                    let right = self.label(right);
                    format!(
                        "\
mov rax, {right}
add rax, {left}"
                    )
                }
                Expression::Result(right) => {
                    let right = self.evaluation(right);
                    format!(
                        "\
{right}
add rax, {left}"
                    )
                }
                Expression::Set(_) => panic!("Invalid operand"),
                Expression::Statement(_) => panic!("Invalid operand"),
            },
            Expression::Label(_) => todo!(),
            Expression::Result(_) => todo!(),
            Expression::Set(_) => panic!("Invalid operand"),
            Expression::Statement(_) => panic!("Invalid operand"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{backends::x86_64::X86_64, frontend::tokenize, intermediate::parse};

    #[test]
    fn it_compiles_addition_between_two_constants() {
        let code = "1 + 2";
        let expression = parse(tokenize(code).unwrap()).unwrap();
        // ACT
        let result = X86_64::init().compile(&expression);
        // ASSERT
        assert_eq!(
            result,
            "\
mov rax, 1 + 2"
        );
    }

    #[test]
    fn it_compiles_addition_between_constant_and_label() {
        let code = "1 + some_label";
        let expression = parse(tokenize(code).unwrap()).unwrap();
        // ACT
        let result = X86_64::init().compile(&expression);
        // ASSERT
        assert_eq!(
            result,
            "\
mov rax, QWORD[rbp - 8]
add rax, 1"
        );
    }

    #[test]
    fn it_compiles_addition_between_constant_and_result() {
        let code = "1 + (2 + 3)";
        let expression = parse(tokenize(code).unwrap()).unwrap();
        // ACT
        let result = X86_64::init().compile(&expression);
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
        let code = "let some_label; some_label + (1 + 2)";
        let expression = parse(tokenize(code).unwrap()).unwrap();
        // ACT
        let result = X86_64::init().compile(&expression);
        // ASSERT
        assert_eq!(
            result,
            "\
mov rax, 2 + 3
add rax, QWORD[rbp - 8]"
        );
    }
}

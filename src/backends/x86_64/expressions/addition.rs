use crate::{
    backends::x86_64::X86_64,
    parser::tokens::{Addition, Value},
};

impl X86_64 {
    pub fn addition(&mut self, addition: &Addition) -> String {
        match &addition.left {
            Value::Constant(left) => match &addition.right {
                Value::Constant(right) => {
                    format!(
                        "\
mov rax, {left} + {right}"
                    )
                }
                Value::Label(right) => {
                    let right = self.label(right);
                    format!(
                        "\
mov rax, {right}
add rax, {left}"
                    )
                }
                Value::Result(right) => {
                    let right = self.expression(right);
                    format!(
                        "\
{right}
add rax, {left}"
                    )
                }
                Value::Reference(_) | Value::Unit => panic!("Invalid operand"),
            },
            Value::Label(left) => match &addition.right {
                Value::Constant(right) => {
                    let left = self.label(left);
                    format!(
                        "\
mov rax, {left}
add rax, {right}"
                    )
                }
                Value::Label(right) => {
                    let left = self.label(left);
                    let right = self.label(right);
                    format!(
                        "\
mov rax, {left}
add rax, {right}"
                    )
                }
                Value::Result(right) => {
                    let left = self.label(left);
                    let right = self.expression(right);
                    format!(
                        "\
{right}
add rax, {left}"
                    )
                }
                Value::Reference(_) | Value::Unit => panic!("Invalid operand"),
            },
            Value::Result(left) => match &addition.right {
                Value::Constant(right) => {
                    let left = self.expression(left);
                    format!(
                        "\
{left}
add rax, {right}"
                    )
                }
                Value::Label(right) => {
                    let left = self.expression(left);
                    let right = self.label(right);
                    format!(
                        "\
{left}
add rax, {right}"
                    )
                }
                Value::Result(right) => {
                    let left = self.expression(left);
                    let right = self.expression(right);
                    format!(
                        "\
{left}
push rax
{right}
pop rdx
add rax, rdx"
                    )
                }
                Value::Reference(_) | Value::Unit => panic!("Invalid operand"),
            },
            Value::Reference(_) | Value::Unit => panic!("Invalid operand"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{backends::x86_64::X86_64, parser::Parser};

    #[test]
    fn it_compiles_addition_between_two_constants() {
        let code = "1 + 2";
        let tokens = Parser::parse(code).unwrap();
        // ACT
        let result = X86_64::init().compile(tokens);
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
        let tokens = Parser::parse(code).unwrap();
        // ACT
        let result = X86_64::init().compile(tokens);
        // ASSERT
        assert_eq!(
            result,
            "\
sub rsp, 8
mov QWORD[rbp - 8], 1
mov rax, QWORD[rbp - 8]
add rax, 1
add rsp, 8"
        );
    }

    #[test]
    fn it_compiles_addition_between_constant_and_result() {
        let code = "1 + (2 + 3)";
        let tokens = Parser::parse(code).unwrap();
        // ACT
        let result = X86_64::init().compile(tokens);
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
        let tokens = Parser::parse(code).unwrap();
        // ACT
        let result = X86_64::init().compile(tokens);
        // ASSERT
        assert_eq!(
            result,
            "\
sub rsp, 8
mov QWORD[rbp - 8], 2
mov rax, 2 + 3
add rax, QWORD[rbp - 8]
add rsp, 8"
        );
    }

    #[test]
    fn it_compiles_addition_between_two_labels() {
        let code = "label1: usize = 3; label2: usize = 2; label1 + label2";
        let tokens = Parser::parse(code).unwrap();
        // ACT
        let result = X86_64::init().compile(tokens);
        // ASSERT
        assert_eq!(
            result,
            "\
sub rsp, 16
mov QWORD[rbp - 8], 3
mov QWORD[rbp - 16], 2
mov rax, QWORD[rbp - 8]
add rax, QWORD[rbp - 16]
add rsp, 16"
        )
    }

    #[test]
    fn it_compiles_addition_between_two_results() {
        let code = "(1 + 2) + (3 + 4)";
        let tokens = Parser::parse(code).unwrap();
        // ACT
        let result = X86_64::init().compile(tokens);
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

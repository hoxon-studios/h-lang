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
            },
            Expression::Label(_) => todo!(),
            Expression::Result(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        backends::x86_64::{Scope, Symbol, X86_64},
        intermediate::expressions::{Addition, Evaluation, Expression},
    };

    #[test]
    fn it_compiles_addition_between_two_constants() {
        let context = X86_64::init();
        let addition = Addition {
            left: Expression::Constant("1".to_string()),
            right: Expression::Constant("2".to_string()),
        };
        // ACT
        let result = context.addition(&addition);
        // ASSERT
        assert_eq!(
            result,
            "\
mov rax, 1 + 2"
        );
    }

    #[test]
    fn it_compiles_addition_between_constant_and_label() {
        let context = X86_64 {
            scopes: vec![Scope {
                stack: vec![Symbol {
                    name: "some_label".to_string(),
                    size: 8,
                }],
            }],
        };
        let addition = Addition {
            left: Expression::Constant("1".to_string()),
            right: Expression::Label("some_label".to_string()),
        };
        // ACT
        let result = context.addition(&addition);
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
        let context = X86_64::init();
        let addition = Addition {
            left: Expression::Constant("1".to_string()),
            right: Expression::Result(Box::new(Evaluation::Addition(Addition {
                left: Expression::Constant("2".to_string()),
                right: Expression::Constant("3".to_string()),
            }))),
        };
        // ACT
        let result = context.addition(&addition);
        // ASSERT
        assert_eq!(
            result,
            "\
mov rax, 2 + 3
add rax, 1"
        );
    }
}

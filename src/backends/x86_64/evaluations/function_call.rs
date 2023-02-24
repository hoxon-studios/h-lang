use crate::{
    backends::x86_64::X86_64,
    intermediate::expressions::{Expression, FunctionCall},
};

const LINUX_SYSCALL_CONVENTION: &[&'static str] = &["rax", "rdi", "rsi", "rdx", "r10", "r8", "r9"];
const SYSTEM_V_AMD64_ABI_CONVENTION: &[&'static str] = &["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

impl X86_64 {
    pub fn function_call(&mut self, function_call: &FunctionCall) -> String {
        let convention = match function_call.label.as_str() {
            "syscall" => LINUX_SYSCALL_CONVENTION,
            _ => SYSTEM_V_AMD64_ABI_CONVENTION,
        };

        let evaluations = function_call
            .parameters
            .0
            .iter()
            .filter_map(|p| match p {
                Expression::Result(evaluation) => {
                    let eval = self.evaluation(&evaluation);
                    Some(format!(
                        "\
{eval}
push rax"
                    ))
                }
                _ => None,
            })
            .collect::<Vec<String>>()
            .join("\n");

        let parameters = function_call
            .parameters
            .0
            .iter()
            .zip(convention.iter())
            .rev()
            .map(|(p, reg)| match p {
                Expression::Unit => format!(
                    "\
mov {reg}, 0"
                ),
                Expression::Constant(constant) => format!(
                    "\
mov {reg}, {constant}"
                ),
                Expression::Label(label) => {
                    let label = self.label(label);
                    format!(
                        "\
mov {reg}, {label}"
                    )
                }
                Expression::Result(_) => format!(
                    "\
pop {reg}"
                ),
                Expression::Set(_) | Expression::Statement(_) => panic!("Invalid parameter"),
            })
            .collect::<Vec<String>>()
            .join("\n");

        let function_name = &function_call.label;

        format!(
            "\
{evaluations}
{parameters}
call {function_name}"
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{backends::x86_64::X86_64, frontend::tokenize, intermediate::parse};

    #[test]
    fn it_compiles_function_call() {
        let code = "some_function(1 + 2, 3, 4 + 5)";
        let expression = parse(tokenize(code).unwrap()).unwrap();
        // ACT
        let result = X86_64::init().compile(&expression);
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
        )
    }
}

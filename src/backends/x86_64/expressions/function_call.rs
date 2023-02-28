use crate::{
    backends::x86_64::X86_64,
    parser::tokens::{FunctionCall, Value},
};

pub const LINUX_SYSCALL_CONVENTION: &[&'static str] =
    &["rax", "rdi", "rsi", "rdx", "r10", "r8", "r9"];
pub const SYSTEM_V_AMD64_ABI_CONVENTION: &[&'static str] =
    &["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

impl X86_64 {
    pub fn function_call(&mut self, function_call: &FunctionCall) -> String {
        let convention = match function_call.label {
            "syscall" => LINUX_SYSCALL_CONVENTION,
            _ => SYSTEM_V_AMD64_ABI_CONVENTION,
        };

        let evaluations = function_call
            .parameters
            .iter()
            .filter_map(|p| match p {
                Value::Result(evaluation) => {
                    let eval = self.expression(&evaluation);
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
            .iter()
            .zip(convention.iter())
            .rev()
            .map(|(p, reg)| match p {
                Value::Constant(constant) => format!(
                    "\
mov {reg}, {constant}"
                ),
                Value::Label(label) => {
                    let label = self.label(label);
                    format!(
                        "\
mov {reg}, {label}"
                    )
                }
                Value::Result(_) => format!(
                    "\
pop {reg}"
                ),
                Value::Unit => panic!("Invalid operand"),
                Value::Reference(label) => {
                    let label = self.label(label);
                    format!(
                        "\
lea {reg}, {label}"
                    )
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        let function_call = match function_call.label {
            "syscall" => "syscall".to_string(),
            _ => format!("call {}", &function_call.label),
        };

        format!(
            "\
{evaluations}
{parameters}
{function_call}"
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{backends::x86_64::X86_64, parser::parse};

    #[test]
    fn it_compiles_function_call() {
        let code = "some_function$(1 + 2, 3, 4 + 5)";
        let tokens = parse(code).unwrap();
        // ACT
        let result = X86_64::init().compile(tokens);
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
        let tokens = parse(code).unwrap();
        // ACT
        let result = X86_64::init().compile(tokens);
        // ASSERT
        assert_eq!(
            result,
            "
mov rdx, length
mov rsi, message
mov rdi, 0
mov rax, 0x01
syscall"
        )
    }
}

use super::X86_64;

impl X86_64 {
    pub fn label(&self, label: &str) -> String {
        let mut position = 0;
        for scope in self.scopes.iter() {
            for symbol in &scope.stack {
                position += symbol.size;
                if symbol.name == label {
                    return format!("QWORD[rbp - {position}]");
                }
            }
        }

        return label.to_string();
    }
}

#[cfg(test)]
mod tests {
    use crate::{backends::x86_64::X86_64, frontend::tokenize, intermediate::parse};

    #[test]
    fn it_compiles_label_stored_in_the_stack() {
        let code = "let some_label = 1; some_label";
        let expression = parse(tokenize(code).unwrap()).unwrap();
        // ACT
        let result = X86_64::init().compile(&expression);
        // ASSERT
        assert_eq!(
            result,
            "\
sub rsp, 8
mov QWORD[rbp - 8], 1
mov rax, QWORD[rbp - 8]
add rsp, 8"
        );
    }

    #[test]
    fn it_compiles_global_label() {
        let code = "some_label";
        let expression = parse(tokenize(code).unwrap()).unwrap();
        // ACT
        let result = X86_64::init().compile(&expression);
        // ASSERT
        assert_eq!(
            result,
            "\
mov rax, some_label"
        );
    }
}

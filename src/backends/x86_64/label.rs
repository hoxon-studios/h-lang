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
    use crate::backends::x86_64::{Scope, Symbol, X86_64};

    #[test]
    fn it_compiles_label_stored_in_the_stack() {
        let context = X86_64 {
            scopes: vec![
                Scope {
                    stack: vec![Symbol {
                        name: "some_label".to_string(),
                        size: 8,
                    }],
                },
                Scope {
                    stack: vec![Symbol {
                        name: "another".to_string(),
                        size: 8,
                    }],
                },
            ],
        };
        let label = "some_label";
        // ACT
        let result = context.label(label);
        // ASSERT
        assert_eq!(result, "QWORD[rbp - 8]");
    }
}

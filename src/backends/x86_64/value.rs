use crate::parser::tokens::Value;

use super::X86_64;

impl X86_64 {
    pub fn value(&mut self, value: &Value) -> String {
        match value {
            Value::Constant(value) => format!(
                "\
mov rax, {value}"
            ),
            Value::Label(label) => {
                let label = self.label(label);
                format!(
                    "\
mov rax, {label}"
                )
            }
            Value::Unit => format!(
                "\
mov rax, 0"
            ),
            Value::Result(result) => self.expression(&*result),
            Value::Reference(label) => {
                let label = self.label(label);
                format!(
                    "\
lea rax, {label}"
                )
            }
        }
    }
}

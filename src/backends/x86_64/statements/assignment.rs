use crate::{
    backends::x86_64::X86_64,
    parser::tokens::{Assignment, Token, Value},
};

impl X86_64 {
    pub fn assignment(&mut self, assignment: &Assignment) -> String {
        let label = match &*assignment.address {
            Token::Declaration(declaration) => self.declaration(&declaration),
            Token::Value(Value::Label(label)) => self.label(label),
            Token::Value(_) | Token::Set(_) | Token::Statement(_) | Token::Definition(_) => {
                panic!("Invalid operand")
            }
        };

        match &assignment.value {
            Value::Unit => format!(
                "\
mov {label}, 0"
            ),
            Value::Constant(value) => format!(
                "\
mov {label}, {value}"
            ),
            Value::Label(value) => {
                let value = self.label(value);
                format!(
                    "\
mov rax, {value}
mov {label}, rax"
                )
            }
            Value::Result(value) => {
                let value = self.expression(&value);
                format!(
                    "\
{value}
mov {label}, rax"
                )
            }
            Value::Reference(value) => {
                let value = self.label(value);
                format!(
                    "\
lea rax, {value}
mov {label}, rax"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{backends::x86_64::X86_64, parser::Parser};

    #[test]
    fn it_compiles_assignment() {
        let code = "some_var = 1";
        let tokens = Parser::parse(code).unwrap();
        // ACT
        let result = X86_64::init().compile(tokens);
        // ASSERT
        assert_eq!(
            result,
            "\
mov some_var, 1"
        );
    }
}

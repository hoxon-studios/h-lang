use crate::{
    backends::x86_64::X86_64,
    intermediate::expressions::{Assignment, Expression},
};

impl X86_64 {
    pub fn assignment(&mut self, assignment: &Assignment) -> String {
        let label = self.label(assignment.label.as_str());

        match &assignment.value {
            Expression::Unit => format!(
                "\
mov {label}, 0"
            ),
            Expression::Constant(value) => format!(
                "\
mov {label}, {value}"
            ),
            Expression::Label(value) => format!(
                "\
mov rax, {value}
mov {label}, rax"
            ),
            Expression::Result(value) => {
                let value = self.evaluation(&value);
                format!(
                    "\
{value}
mov {label}, rax"
                )
            }
            Expression::Statement(_) | Expression::Set(_) => panic!("Invalid operands"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{backends::x86_64::X86_64, frontend::tokenize, intermediate::parse};

    #[test]
    fn it_compiles_assignment() {
        let code = "some_var = 1";
        let expression = parse(tokenize(code).unwrap()).unwrap();
        // ACT
        let result = X86_64::init().compile(&expression);
        // ASSERT
        assert_eq!(
            result,
            "\
mov some_var, 1"
        );
    }
}

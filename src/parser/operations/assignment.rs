use crate::parser::{
    tokens::{Assignment, Statement, Token},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_assignment(&mut self) -> Result<(), String> {
        let Some(Token::Value(value)) = self.output.pop() else {
            return Err("Operand not found".to_string());
        };
        let Some(address) = self.output.pop() else {
            return Err("Operand not found".to_string());
        };

        self.output
            .push(Token::Statement(Statement::Assignment(Assignment {
                address: Box::new(address),
                value,
            })));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        tokens::{
            Addition, Assignment, Declaration, Expression, LabelType, Statement, Token, Value,
        },
        Parser,
    };

    #[test]
    fn it_parses_assignment() {
        let code = "variable: usize = 1 + 2";
        // ACT
        let result = Parser::parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![Token::Statement(Statement::Assignment(Assignment {
                address: Box::new(Token::Declaration(Declaration {
                    label: "variable",
                    pointer: false,
                    _type: LabelType::Usize
                })),
                value: Value::Result(Box::new(Expression::Addition(Addition {
                    left: Value::Constant("1"),
                    right: Value::Constant("2")
                })))
            }))]
        );
    }
}

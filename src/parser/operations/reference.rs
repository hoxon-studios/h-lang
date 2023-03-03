use crate::parser::{
    tokens::{Token, Value},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_reference(&mut self) -> Result<(), String> {
        let Some(Token::Value(Value::Label(label))) = self.output.pop() else {
            return Err("Invalid operand".to_string());
        };

        self.output.push(Token::Value(Value::Reference(label)));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        tokens::{Assignment, Block, Declaration, Expression, LabelType, Statement, Token, Value},
        Parser,
    };

    #[test]
    fn it_parses_reference() {
        let code = "some_var: usize = 1; &some_var";
        // ACT
        let result = Parser::parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![Token::Value(Value::Result(Box::new(Expression::Block(
                Block {
                    body: vec![Statement::Assignment(Assignment {
                        address: Box::new(Token::Declaration(Declaration {
                            label: "some_var",
                            pointer: false,
                            _type: LabelType::Usize
                        })),
                        value: Value::Constant("1")
                    })],
                    result: Value::Reference("some_var")
                }
            ))))]
        );
    }
}

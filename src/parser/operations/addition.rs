use crate::parser::{
    tokens::{Addition, Expression, Token, Value},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_addition(&mut self) -> Result<(), String> {
        let Some(Token::Value(right)) = self.output.pop() else {
            return Err("Operand not found".to_string());
        };
        let Some(Token::Value(left)) = self.output.pop() else {
            return Err("Operand not found".to_string());
        };

        self.output
            .push(Token::Value(Value::Result(Box::new(Expression::Addition(
                Addition { left, right },
            )))));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        tokens::{Addition, Expression, Token, Value},
        Parser,
    };

    #[test]
    fn it_parses_addition() {
        let code = "1 + 2";
        // ACT
        let result = Parser::parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![Token::Value(Value::Result(Box::new(Expression::Addition(
                Addition {
                    left: Value::Constant("1"),
                    right: Value::Constant("2")
                }
            ))))]
        );
    }
}

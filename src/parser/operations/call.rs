use crate::parser::{
    tokens::{Expression, FunctionCall, Token, Value},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_call(&mut self) -> Result<(), String> {
        let Some(expression) = self.output.pop() else {
            return Err("Operand not found".to_string());
        };
        let Some(Token::Value(Value::Label(label))) = self.output.pop() else {
            return Err("Operand not found".to_string());
        };
        let parameters = match expression {
            Token::Set(set) => set,
            _ => vec![expression],
        }
        .iter()
        .map(|p| match p {
            Token::Value(value) => value.clone(),
            _ => panic!("Invalid operand"),
        })
        .collect::<Vec<Value>>();

        self.output.push(Token::Value(Value::Result(Box::new(
            Expression::FunctionCall(FunctionCall { label, parameters }),
        ))));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        tokens::{Expression, FunctionCall, Token, Value},
        Parser,
    };

    #[test]
    fn it_parses_function() {
        let code = "some_func$(1, 2, 3)";
        // ACT
        let result = Parser::parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![Token::Value(Value::Result(Box::new(
                Expression::FunctionCall(FunctionCall {
                    label: "some_func",
                    parameters: vec![
                        Value::Constant("1"),
                        Value::Constant("2"),
                        Value::Constant("3")
                    ]
                })
            )))]
        );
    }
}

use crate::parser::{tokens::Token, Parser};

impl<'a> Parser<'a> {
    pub fn parse_group(&mut self) -> Result<(), String> {
        let Some(left) = self.output.pop() else {
            return Err("Operand not found".to_string());
        };
        let Some(right) = self.output.pop() else {
            return Err("Operand not found".to_string());
        };

        let mut left = if let Token::Set(left) = left {
            left
        } else {
            vec![left]
        };
        let mut right = if let Token::Set(right) = right {
            right
        } else {
            vec![right]
        };

        right.append(&mut left);
        self.output.push(Token::Set(right));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        tokens::{Token, Value},
        Parser,
    };

    #[test]
    fn it_parses_group() {
        let code = "(1, 2, 3)";
        // ACT
        let result = Parser::parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![Token::Set(vec![
                Token::Value(Value::Constant("1")),
                Token::Value(Value::Constant("2")),
                Token::Value(Value::Constant("3"))
            ])]
        );
    }

    #[test]
    fn it_parses_empty_group() {
        let code = "()";
        // ACT
        let result = Parser::parse(code).unwrap();
        // ASSERT
        assert_eq!(result, vec![Token::Value(Value::Unit)]);
    }
}

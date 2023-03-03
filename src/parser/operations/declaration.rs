use crate::parser::{
    tokens::{Declaration, LabelType, Token, Value},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_declaration(&mut self) -> Result<(), String> {
        let Some(Token::Value(_type)) = self.output.pop() else {
            return Err("Invalid operand".to_string());
        };
        let Some(Token::Value(Value::Label(label))) = self.output.pop() else {
            return Err("Invalid operand".to_string());
        };

        let (pointer, _type) = match _type {
            Value::Label("usize") => (false, LabelType::Usize),
            Value::Reference("usize") => (true, LabelType::Usize),
            _ => return Err("Invalid operand".to_string()),
        };

        self.output.push(Token::Declaration(Declaration {
            label,
            _type,
            pointer,
        }));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        tokens::{Declaration, LabelType, Token},
        Parser,
    };

    #[test]
    fn it_parses_declaration() {
        let code = "some_var: usize";
        // ACT
        let result = Parser::parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![Token::Declaration(Declaration {
                label: "some_var",
                pointer: false,
                _type: LabelType::Usize
            })],
        );
    }

    #[test]
    fn it_parses_pointer_declaration() {
        let code = "some_var: &usize";
        // ACT
        let result = Parser::parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![Token::Declaration(Declaration {
                label: "some_var",
                pointer: true,
                _type: LabelType::Usize
            })],
        );
    }
}

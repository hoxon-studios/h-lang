use crate::parser::{
    tokens::{Dereference, Expression, Token, Value},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_dereference(&mut self) -> Result<(), String> {
        let Some(Token::Value(index)) = self.output.pop() else {
            return Err("Invalid operand".to_string());
        };
        let Some(Token::Value(Value::Label(label))) = self.output.pop() else {
            return Err("Invalid operand".to_string())
        };

        self.output.push(Token::Value(Value::Result(Box::new(
            Expression::Dereference(Dereference {
                label,
                index: Box::new(index),
            }),
        ))));
        Ok(())
    }
}

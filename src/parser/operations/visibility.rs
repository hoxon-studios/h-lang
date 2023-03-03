use crate::parser::{
    tokens::{Definition, Token},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_visibility(&mut self, export: bool) -> Result<(), String> {
        let Some(mut token) = self.output.pop() else {
            return Err("Invalid operand".to_string());
        };

        match &mut token {
            Token::Definition(Definition::Function(function)) => function.export = export,
            _ => return Err("Invalid operand".to_string()),
        }

        self.output.push(token);
        Ok(())
    }
}

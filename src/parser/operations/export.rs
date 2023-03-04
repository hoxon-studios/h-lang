use crate::parser::{tokens::Token, Parser};

impl<'a> Parser<'a> {
    pub fn parse_export(&mut self) {
        let Some(Token::Item { name, definition }) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let definition = format!(
            "\
global {name}
{definition}"
        );

        self.output.push(Token::Item { name, definition });
    }
}

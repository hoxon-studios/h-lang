use crate::parser::{
    tokens::{Code, Definition, Token},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_export(&mut self) {
        let Some(Token::Definition(Definition { name, definition: Code(definition) })) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let definition = Code(format!(
            "\
global {name}
{definition}"
        ));

        self.output
            .push(Token::Definition(Definition { name, definition }));
    }
}

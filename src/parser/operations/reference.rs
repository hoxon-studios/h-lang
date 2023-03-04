use crate::parser::{tokens::Token, Parser};

impl<'a> Parser<'a> {
    pub fn parse_reference(&mut self) {
        let Some(Token::Label(label)) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let label = self.context.address(label);
        let result = format!(
            "\
lea rax, {label}"
        );

        self.output.push(Token::Result(result));
    }
}

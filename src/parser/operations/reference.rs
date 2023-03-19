use crate::parser::{
    tokens::{Code, Token},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_reference(&mut self) {
        let Some(label) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let label = self.context.resolve(label);

        if let Token::Label(label) = label {
            let address = label.to_address();
            let result = format!(
                "\
lea rax, {address}"
            );

            self.output.push(Token::Result(Code(result)));
        } else {
            panic!("Invalid operand")
        }
    }
}

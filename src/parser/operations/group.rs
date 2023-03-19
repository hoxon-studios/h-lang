use crate::parser::{
    tokens::{Token, TokenSet},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_group(&mut self) {
        let Some(left) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(right) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let mut left = if let Token::Set(TokenSet(left)) = left {
            left
        } else {
            vec![left]
        };
        let mut right = if let Token::Set(TokenSet(right)) = right {
            right
        } else {
            vec![right]
        };

        right.append(&mut left);
        self.output.push(Token::Set(TokenSet(right)));
    }
}

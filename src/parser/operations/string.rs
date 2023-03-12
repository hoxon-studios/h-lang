use crate::parser::{tokens::Token, Parser};

impl<'a> Parser<'a> {
    pub fn parse_string(&mut self) {
        let Some(Token::String(value)) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(Token::Id(id)) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let value = value.replace("\n", "\\n");
        let definition = format!(
            "\
segment .data
{id}: db `{value}`, 0"
        );

        self.output.push(Token::Item {
            name: id,
            definition,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn it_parses_string_literal() {
        let code = "string hello \"Hello World\"";
        // ACT
        let result = Parser::parse(code);
        // ASSERT
        assert_eq!(
            result,
            "\
segment .data
hello: db `Hello World`, 0"
        )
    }
}

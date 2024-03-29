use crate::parser::{
    context::{Symbol, SymbolType},
    tokens::{Code, Definition, Id, StringLiteral, Token},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_string(&mut self) {
        let Some(Token::String(StringLiteral(value))) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(Token::Id(Id(id))) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let value = value.replace("\n", "\\n");
        let definition = format!(
            "\
segment .data
{id}: db `{value}`, 0"
        );

        self.context.global.push(Symbol {
            id: id.to_string(),
            _type: SymbolType::String,
        });

        self.output.push(Token::Definition(Definition {
            name: id,
            definition: Code(definition),
        }));
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

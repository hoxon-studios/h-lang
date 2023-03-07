use crate::parser::{
    context::{Property, Struct},
    tokens::Token,
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_struct(&mut self) {
        let Some(_) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(Token::Label(name)) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let Some(scope) = self.context.scopes.pop() else {
            panic!("Scope not found")
        };

        let properties = scope
            .symbols
            .iter()
            .map(|s| Property {
                name: s.name.to_string(),
                _type: s._type.clone(),
            })
            .collect::<Vec<Property>>();
        let _struct = Struct {
            name: name.to_string(),
            properties,
        };

        self.context.structs.push(_struct);
    }
}

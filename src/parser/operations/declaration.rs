use crate::parser::{
    context::SymbolType,
    tokens::{Label, Token},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_declaration(&mut self) {
        let Some(Token::Id(_type)) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(left) = self.output.pop() else {
            panic!("Invalid operand")
        };

        if _type == "ptr" {
            match left {
                Token::Label(Label { id, _type, .. }) => {
                    self.context
                        .declare(id, &SymbolType::Pointer(Box::new(_type.clone())));
                    let label = self.context.label(id);
                    self.output.push(Token::Label(label.clone()));
                }
                _ => panic!("Invalid operand"),
            }
        } else {
            let _type = match _type {
                "usize" => SymbolType::Usize,
                _ => {
                    let structs = &self.context.structs;
                    let definition = structs
                        .iter()
                        .find(|s| s.name == _type)
                        .expect("Type not found");
                    SymbolType::Struct(definition.clone())
                }
            };
            let id = match left {
                Token::Id(id) => id,
                Token::Label(Label { id, .. }) => id,
                _ => panic!("Invalid operand"),
            };
            self.context.declare(id, &_type);
            let label = self.context.label(id);
            self.output.push(Token::Label(label.clone()));
        }
    }
}

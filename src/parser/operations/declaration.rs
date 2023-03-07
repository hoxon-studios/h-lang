use crate::parser::{
    context::{Symbol, SymbolType},
    tokens::Token,
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_declaration(&mut self) {
        let Some(Token::Label(_type)) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(Token::Label(label)) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let structs = &self.context.structs;

        if _type == "ptr" {
            let scope = self.context.take_scope();
            let symbol = scope
                .symbols
                .iter_mut()
                .find(|s| s.name == label)
                .expect("Symbol not found");
            symbol._type = SymbolType::Pointer(Box::new(symbol._type.clone()));
        } else {
            let _type = match _type {
                "usize" => SymbolType::Usize,
                _ => {
                    let definition = structs
                        .iter()
                        .find(|s| s.name == _type)
                        .expect("Type not found");
                    SymbolType::Struct(definition.clone())
                }
            };
            let scope = self.context.take_scope();
            let symbol = scope.symbols.iter_mut().find(|s| s.name == label);
            match symbol {
                Some(symbol) => symbol._type = _type,
                None => scope.symbols.push(Symbol {
                    name: label.to_string(),
                    _type,
                }),
            }
        }

        self.output.push(Token::Label(label));
    }
}

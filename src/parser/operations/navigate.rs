use crate::parser::{
    context::SymbolType,
    tokens::{Code, Id, Label, LabelAddress, Token},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_navigation(&mut self) {
        let Some(Token::Id(Id(property))) = self.output.pop() else {
            panic!("Invalid operand")
        };
        let Some(label) = self.output.pop() else {
            panic!("Invalid operand")
        };

        let label = self.context.resolve(label);

        match &label {
            Token::Label(label) => match label.address {
                LabelAddress::Stack { position, .. } => match &label._type {
                    SymbolType::Struct(_struct) => {
                        let offset = _struct.offset(property);
                        let _type = _struct.property_type(property);
                        self.output.push(Token::Label(Label {
                            id: label.id,
                            _type,
                            address: LabelAddress::Stack { position, offset },
                        }));
                    }
                    SymbolType::Pointer(pointer) => match pointer.as_ref() {
                        SymbolType::Struct(_struct) => {
                            let offset = _struct.offset(property);
                            let address = label.to_address();
                            let result = format!(
                                "\
mov rax, {address}
add rax, {offset}"
                            );
                            self.output.push(Token::Result(Code(result)));
                        }
                        _ => panic!("Invalid label type"),
                    },
                    _ => panic!("Invalid label type"),
                },
                _ => panic!("Invalid label type"),
            },
            _ => panic!("Invalid operand"),
        }
    }
}

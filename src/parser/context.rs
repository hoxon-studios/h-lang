use super::tokens::{Label, LabelAddress, Token};

const USIZE: usize = 8;
const BYTE: usize = 1;

#[derive(Debug)]
pub struct Context {
    pub structs: Vec<Struct>,
    pub global: Vec<Symbol>,
    pub scopes: Vec<ContextScope>,
}
impl Context {
    pub fn init() -> Context {
        Context {
            global: vec![],
            scopes: vec![],
            structs: vec![],
        }
    }
    fn symbol(&self, id: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            for symbol in &scope.symbols {
                if symbol.id == id {
                    return Some(symbol);
                }
            }
        }

        return None;
    }

    pub fn declare(&mut self, id: &str, _type: &SymbolType) {
        let scope = self.take_scope();
        let symbol = scope.symbols.iter_mut().find(|s| s.id == id);
        match symbol {
            Some(symbol) => symbol._type = _type.clone(),
            None => scope.symbols.push(Symbol {
                id: id.to_string(),
                _type: _type.clone(),
            }),
        }
    }

    pub fn resolve<'a>(&self, token: Token<'a>) -> Token<'a> {
        if let Token::Id(id) = token {
            Token::Label(self.label(id))
        } else {
            token
        }
    }

    pub fn label<'a>(&self, id: &'a str) -> Label<'a> {
        let mut position = 0;
        for scope in self.scopes.iter().rev() {
            for symbol in &scope.symbols {
                let size = symbol._type.size();
                position += size;
                if symbol.id == id {
                    return Label {
                        id,
                        _type: symbol._type.clone(),
                        address: LabelAddress::Stack {
                            position,
                            offset: 0,
                        },
                    };
                }
            }
        }

        for symbol in &self.global {
            if symbol.id == id {
                return Label {
                    id,
                    _type: symbol._type.clone(),
                    address: LabelAddress::Global { label: id },
                };
            }
        }

        panic!("Invalid identifier")
    }

    pub fn pointer_size(&self, id: &str) -> usize {
        let symbol = self.symbol(id).expect("Symbol not found");
        symbol._type.pointer_size()
    }

    pub fn take_scope(&mut self) -> &mut ContextScope {
        if self.scopes.len() > 0 {
            self.scopes.last_mut().unwrap()
        } else {
            let scope = ContextScope {
                symbols: vec![],
                labels: 0,
            };
            self.scopes.push(scope);

            self.scopes.last_mut().unwrap()
        }
    }

    pub fn take_label(&mut self, prefix: &str) -> String {
        let scope = self.take_scope();
        let label = format!(".{}_{}", prefix, scope.labels);
        scope.labels += 1;

        label
    }
}

#[derive(Debug)]
pub struct ContextScope {
    pub symbols: Vec<Symbol>,
    pub labels: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub properties: Vec<Property>,
}
#[derive(PartialEq, Debug, Clone)]
pub struct Property {
    pub name: String,
    pub _type: SymbolType,
}
impl Struct {
    pub fn size(&self) -> usize {
        self.properties.iter().map(|p| p._type.size()).sum()
    }
    pub fn offset(&self, property_name: &str) -> usize {
        let mut offset = 0;
        for property in &self.properties {
            if property.name == property_name {
                break;
            } else {
                offset += property._type.size()
            }
        }

        return offset;
    }
    pub fn property_type(&self, property_name: &str) -> SymbolType {
        self.properties
            .iter()
            .find(|p| p.name == property_name)
            .expect("Invalid property")
            ._type
            .clone()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Symbol {
    pub id: String,
    pub _type: SymbolType,
}

#[derive(PartialEq, Debug, Clone)]
pub enum SymbolType {
    Usize,
    Byte,
    Struct(Struct),
    Pointer(Box<SymbolType>),
    String,
}

impl SymbolType {
    pub fn size(&self) -> usize {
        match self {
            SymbolType::Usize => USIZE,
            SymbolType::Byte => BYTE,
            SymbolType::Pointer(_) => USIZE,
            SymbolType::Struct(_struct) => _struct.size(),
            SymbolType::String => panic!("String is not sized"),
        }
    }
    pub fn pointer_size(&self) -> usize {
        match self {
            SymbolType::Pointer(_type) => _type.as_ref().size(),
            _ => panic!("Invalid pointer"),
        }
    }
}

impl<'a> Label<'a> {
    pub fn to_address(&self) -> String {
        match self.address {
            LabelAddress::Global { label } => label.to_string(),
            LabelAddress::Stack { position, offset } => {
                if offset > 0 {
                    format!("QWORD[rbp - {position} + {offset}]")
                } else {
                    format!("QWORD[rbp - {position}]")
                }
            }
        }
    }
}

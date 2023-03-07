const USIZE: usize = 8;

#[derive(Debug)]
pub struct Context {
    pub structs: Vec<Struct>,
    pub scopes: Vec<ContextScope>,
}
impl Context {
    pub fn init() -> Context {
        Context {
            scopes: vec![],
            structs: vec![],
        }
    }
    pub fn symbol(&self, label: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            for symbol in &scope.symbols {
                if symbol.name == label {
                    return Some(symbol);
                }
            }
        }

        return None;
    }

    pub fn address(&self, label: &str) -> String {
        let parts: Vec<&str> = label.split('.').collect();
        let label = parts[0];
        let properties = &parts[1..];

        let mut position = 0;
        let mut current_symbol: Option<&Symbol> = None;
        'pos: for scope in self.scopes.iter().rev() {
            for symbol in &scope.symbols {
                let size = symbol._type.size();
                position += size;
                if symbol.name == label {
                    current_symbol = Some(symbol);
                    break 'pos;
                }
            }
        }

        if let Some(current_symbol) = current_symbol {
            let mut offset = 0;
            if !properties.is_empty() {
                let mut current_struct = match &current_symbol._type {
                    SymbolType::Struct(_struct) => _struct.name.clone(),
                    _ => panic!("Invalid symbol type"),
                };
                for &prop in properties {
                    let (off, child) = self.property_offset(&current_struct, prop);
                    if let Some(child) = child {
                        current_struct = child.name;
                    }
                    offset += off;
                }
            }

            if offset > 0 {
                return format!("QWORD[rbp - {position} + {offset}]");
            } else {
                return format!("QWORD[rbp - {position}]");
            }
        } else {
            return label.to_string();
        }
    }

    pub fn property_offset(&self, _struct: &str, property_name: &str) -> (usize, Option<Struct>) {
        let mut _struct = self
            .structs
            .iter()
            .find(|s| s.name == _struct)
            .expect("Invalid struct");

        let mut offset = 0;
        let mut found: bool = false;
        let mut child: Option<Struct> = None;
        for property in &_struct.properties {
            if property.name == property_name {
                found = true;
                if let SymbolType::Struct(_struct) = &property._type {
                    child = Some(_struct.clone())
                }
                break;
            } else {
                offset += property._type.size()
            }
        }

        if !found {
            panic!("Property not found")
        }

        return (offset, child);
    }

    pub fn pointer_size(&self, label: &str) -> usize {
        let symbol = self.symbol(label).expect("Symbol not found");
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
}

#[derive(PartialEq, Clone, Debug)]
pub struct Symbol {
    pub name: String,
    pub _type: SymbolType,
}

#[derive(PartialEq, Debug, Clone)]
pub enum SymbolType {
    Usize,
    Struct(Struct),
    Pointer(Box<SymbolType>),
}

impl SymbolType {
    pub fn size(&self) -> usize {
        match self {
            SymbolType::Usize => USIZE,
            SymbolType::Pointer(_) => USIZE,
            SymbolType::Struct(_struct) => _struct.size(),
        }
    }
    pub fn pointer_size(&self) -> usize {
        match self {
            SymbolType::Pointer(_type) => _type.as_ref().size(),
            _ => panic!("Invalid pointer"),
        }
    }
}

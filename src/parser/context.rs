const USIZE: usize = 8;

#[derive(Debug)]
pub struct Context {
    pub scopes: Vec<ContextScope>,
}
impl Context {
    pub fn init() -> Context {
        Context { scopes: vec![] }
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
        let mut position = 0;
        for scope in self.scopes.iter().rev() {
            for symbol in &scope.symbols {
                let size = symbol._type.size();
                position += size;
                if symbol.name == label {
                    return format!("QWORD[rbp - {position}]");
                }
            }
        }

        return label.to_string();
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

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub _type: SymbolType,
}

#[derive(Debug, Clone)]
pub enum SymbolType {
    Usize,
    Pointer(Box<SymbolType>),
}

impl SymbolType {
    pub fn size(&self) -> usize {
        match self {
            SymbolType::Usize => USIZE,
            SymbolType::Pointer(_) => USIZE,
        }
    }
    pub fn pointer_size(&self) -> usize {
        match self {
            SymbolType::Pointer(_type) => _type.as_ref().size(),
            _ => panic!("Invalid pointer"),
        }
    }
}

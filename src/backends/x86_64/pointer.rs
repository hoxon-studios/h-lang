use super::{SymbolType, X86_64};

impl X86_64 {
    pub fn pointer_size(&self, label: &str) -> usize {
        for scope in self.scopes.iter() {
            for symbol in &scope.stack {
                if symbol.name == label {
                    match symbol._type {
                        SymbolType::Pointer(size) => return size,
                        _ => panic!("Invalid pointer"),
                    }
                }
            }
        }

        panic!("Pointer not found");
    }
}

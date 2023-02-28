use crate::parser::tokens::{Declaration, LabelType};

use super::{Symbol, SymbolType, X86_64};

const USIZE: usize = 8;

impl X86_64 {
    pub fn declaration(&mut self, declaration: &Declaration) -> String {
        if let Some(scope) = self.scopes.last_mut() {
            let _type = match (declaration.pointer, &declaration._type) {
                (true, LabelType::Usize) => SymbolType::Pointer(USIZE),
                (false, LabelType::Usize) => SymbolType::Value(USIZE),
            };
            if let Some(symbol) = scope.stack.iter_mut().find(|s| s.name == declaration.label) {
                symbol._type = _type;
            } else {
                scope.stack.push(Symbol {
                    name: declaration.label.to_string(),
                    _type,
                });
            }
        }

        self.label(declaration.label)
    }

    pub fn to_symbol(declaration: &Declaration) -> Symbol {
        let _type = match declaration.pointer {
            true => match declaration._type {
                LabelType::Usize => SymbolType::Pointer(USIZE),
            },
            false => match declaration._type {
                LabelType::Usize => SymbolType::Pointer(USIZE),
            },
        };

        Symbol {
            name: declaration.label.to_string(),
            _type,
        }
    }
}

use crate::parser::tokens::Declaration;

use super::{Symbol, X86_64};

impl X86_64 {
    pub fn declaration(&mut self, declaration: &Declaration) -> String {
        if let Some(scope) = self.scopes.last_mut() {
            if let Some(symbol) = scope.stack.iter_mut().find(|s| s.name == declaration.label) {
                symbol.size = 8;
            } else {
                scope.stack.push(Symbol {
                    name: declaration.label.to_string(),
                    size: 8,
                });
            }
        }

        self.label(declaration.label)
    }
}

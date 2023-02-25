use crate::{
    backends::x86_64::{Symbol, X86_64},
    intermediate::expressions::LetStatement,
};

impl X86_64 {
    pub fn let_statement(&mut self, let_statement: &LetStatement) -> String {
        if let Some(scope) = self.scopes.last_mut() {
            if let Some(symbol) = scope
                .stack
                .iter_mut()
                .find(|s| s.name == let_statement.0.label)
            {
                symbol.size = 8;
            } else {
                scope.stack.push(Symbol {
                    name: let_statement.0.label.clone(),
                    size: 8,
                });
            }
        }

        self.assignment(&let_statement.0)
    }
}

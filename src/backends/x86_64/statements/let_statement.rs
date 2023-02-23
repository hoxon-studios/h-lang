use crate::{
    backends::x86_64::{Scope, Symbol, X86_64},
    intermediate::expressions::LetStatement,
};

impl X86_64 {
    pub fn let_statement(&mut self, let_statement: &LetStatement) -> String {
        let symbol = Symbol {
            name: let_statement.label.clone(),
            size: 8,
        };

        if let Some(scope) = self.scopes.last_mut() {
            scope.stack.push(symbol);
        } else {
            self.scopes.push(Scope {
                stack: vec![symbol],
            });
        }

        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{backends::x86_64::X86_64, frontend::tokenize, intermediate::parse};

    #[test]
    fn it_compiles_let_statement() {
        let code = "let some_var";
        let expression = parse(tokenize(code).unwrap()).unwrap();
        // ACT
        let result = X86_64::init().compile(&expression);
        // ASSERT
        assert_eq!(result, "")
    }
}
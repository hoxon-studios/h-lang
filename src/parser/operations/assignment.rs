use crate::parser::tokens::{Assignment, Statement, Token};

pub fn parse_assignment(stack: &mut Vec<Token>) -> Result<(), String> {
    let Some(Token::Value(value)) = stack.pop() else {
        return Err("Operand not found".to_string());
    };
    let Some(address) = stack.pop() else {
        return Err("Operand not found".to_string());
    };

    stack.push(Token::Statement(Statement::Assignment(Assignment {
        address: Box::new(address),
        value,
    })));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        parse,
        tokens::{Addition, Assignment, Declaration, Expression, Statement, Token, Value},
    };

    #[test]
    fn it_parses_assignment() {
        let code = "variable: usize = 1 + 2";
        // ACT
        let result = parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            Token::Statement(Statement::Assignment(Assignment {
                address: Box::new(Token::Declaration(Declaration {
                    label: "variable",
                    _type: "usize"
                })),
                value: Value::Result(Box::new(Expression::Addition(Addition {
                    left: Value::Constant("1"),
                    right: Value::Constant("2")
                })))
            }))
        );
    }
}

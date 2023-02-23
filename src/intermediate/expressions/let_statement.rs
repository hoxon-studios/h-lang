use super::{Expression, LetStatement, Statement};

pub fn parse_let_statement(stack: &mut Vec<Expression>) -> Result<(), String> {
    let Some(Expression::Label(label)) = stack.pop() else {
        return Err("Operand not found".to_string());
    };

    stack.push(Expression::Statement(Statement::Let(LetStatement {
        label,
    })));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        frontend::tokenize,
        intermediate::{
            expressions::{Expression, LetStatement, Statement},
            parse,
        },
    };

    #[test]
    fn it_parses_let_statement() {
        let code = "let some_var";
        let tokens = tokenize(code).unwrap();
        // ACT
        let result = parse(tokens).unwrap();
        // ASSERT
        assert_eq!(
            result,
            Expression::Statement(Statement::Let(LetStatement {
                label: "some_var".to_string()
            }))
        );
    }
}

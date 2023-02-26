use super::{Expression, LetStatement, Statement};

pub fn parse_let_statement(stack: &mut Vec<Expression>) -> Result<(), String> {
    let Some(Expression::Statement(label)) = stack.pop() else {
        return Err("Operand not found".to_string());
    };
    let Statement::Assignment(assignment) = *label else {
        return Err("Invalid operand".to_string());
    };

    stack.push(Expression::Statement(Box::new(Statement::Let(
        LetStatement(assignment),
    ))));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::{
            expressions::{Assignment, Expression, LetStatement, Statement},
            parse,
        },
        tokenizer::tokenize,
    };

    #[test]
    fn it_parses_let_statement() {
        let code = "let some_var = 1";
        let tokens = tokenize(code).unwrap();
        // ACT
        let result = parse(tokens).unwrap();
        // ASSERT
        assert_eq!(
            result,
            Expression::Statement(Box::new(Statement::Let(LetStatement(Assignment {
                label: "some_var".to_string(),
                value: Expression::Constant("1".to_string())
            }))))
        );
    }
}

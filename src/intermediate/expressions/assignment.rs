use super::{Assignment, Expression, Statement};

pub fn parse_assignment(stack: &mut Vec<Expression>) -> Result<(), String> {
    let Some(value) = stack.pop() else {
        return Err("Operand not found".to_string());
    };
    let Some(Expression::Label(label)) = stack.pop() else {
        return Err("Operand not found".to_string());
    };

    stack.push(Expression::Statement(Box::new(Statement::Assignment(
        Assignment { label, value },
    ))));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        frontend::tokenize,
        intermediate::{
            expressions::{Addition, Assignment, Evaluation, Expression, Statement},
            parse,
        },
    };

    #[test]
    fn it_parses_assignment() {
        let code = "variable = 1 + 2";
        // ACT
        let result = parse(tokenize(code).unwrap()).unwrap();
        // ASSERT
        assert_eq!(
            result,
            Expression::Statement(Box::new(Statement::Assignment(Assignment {
                label: "variable".to_string(),
                value: Expression::Result(Box::new(Evaluation::Addition(Addition {
                    left: Expression::Constant("1".to_string()),
                    right: Expression::Constant("2".to_string())
                })))
            })))
        );
    }
}

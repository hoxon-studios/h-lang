use super::{Addition, Evaluation, Expression};

pub fn parse_addition(stack: &mut Vec<Expression>) -> Result<(), String> {
    let Some(right) = stack.pop() else {
        return Err("Operand not found".to_string());
    };
    let Some(left) = stack.pop() else {
        return Err("Operand not found".to_string());
    };

    stack.push(Expression::Result(Box::new(Evaluation::Addition(
        Addition { left, right },
    ))));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        frontend::tokenize,
        intermediate::{
            expressions::{Addition, Evaluation, Expression},
            parse,
        },
    };

    #[test]
    fn it_parses_addition() {
        let code = "1 + 2)";
        let tokens = tokenize(code).unwrap();
        // ACT
        let result = parse(tokens).unwrap();
        // ASSERT
        assert_eq!(
            result,
            Expression::Result(Box::new(Evaluation::Addition(Addition {
                left: Expression::Constant("1".to_string()),
                right: Expression::Constant("2".to_string())
            })))
        );
    }
}

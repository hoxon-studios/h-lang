use super::{Evaluation, Expression, ExpressionSet, FunctionCall};

pub fn parse_function(stack: &mut Vec<Expression>, label: &str) -> Result<(), String> {
    let Some(expression) = stack.pop() else {
        return Err("Operand not found".to_string());
    };
    let parameters = match expression {
        Expression::Set(set) => set,
        _ => ExpressionSet(vec![expression]),
    };

    stack.push(Expression::Result(Box::new(Evaluation::FunctionCall(
        FunctionCall {
            label: label.to_string(),
            parameters,
        },
    ))));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        frontend::tokenize,
        intermediate::{
            expressions::{Evaluation, Expression, ExpressionSet, FunctionCall},
            parse,
        },
    };

    #[test]
    fn it_parses_function() {
        let code = "some_func(1, 2, 3)";
        let tokens = tokenize(code).unwrap();
        // ACT
        let result = parse(tokens).unwrap();
        // ASSERT
        assert_eq!(
            result,
            Expression::Result(Box::new(Evaluation::FunctionCall(FunctionCall {
                label: "some_func".to_string(),
                parameters: ExpressionSet(vec![
                    Expression::Constant("1".to_string()),
                    Expression::Constant("2".to_string()),
                    Expression::Constant("3".to_string())
                ])
            })))
        );
    }
}

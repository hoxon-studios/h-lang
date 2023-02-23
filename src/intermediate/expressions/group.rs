use super::{Expression, ExpressionSet};

pub fn parse_group(stack: &mut Vec<Expression>) -> Result<(), String> {
    let Some(left) = stack.pop() else {
        return Err("Operand not found".to_string());
    };
    let Some(right) = stack.pop() else {
        return Err("Operand not found".to_string());
    };

    let mut left = if let Expression::Set(ExpressionSet(left)) = left {
        left
    } else {
        vec![left]
    };
    let mut right = if let Expression::Set(ExpressionSet(right)) = right {
        right
    } else {
        vec![right]
    };

    right.append(&mut left);
    stack.push(Expression::Set(ExpressionSet(right)));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        frontend::tokenize,
        intermediate::{
            expressions::{Expression, ExpressionSet},
            parse,
        },
    };

    #[test]
    fn it_parses_group() {
        let code = "(1, 2, 3)";
        let tokens = tokenize(code).unwrap();
        // ACT
        let result = parse(tokens).unwrap();
        // ASSERT
        assert_eq!(
            result,
            Expression::Set(ExpressionSet(vec![
                Expression::Constant("1".to_string()),
                Expression::Constant("2".to_string()),
                Expression::Constant("3".to_string())
            ]))
        );
    }

    #[test]
    fn it_parses_empty_group() {
        let code = "()";
        let tokens = tokenize(code).unwrap();
        // ACT
        let result = parse(tokens).unwrap();
        // ASSERT
        assert_eq!(result, Expression::Set(ExpressionSet(vec![])));
    }
}

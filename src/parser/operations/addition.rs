use crate::parser::tokens::{Addition, Expression, Token, Value};

pub fn parse_addition(stack: &mut Vec<Token>) -> Result<(), String> {
    let Some(Token::Value(right)) = stack.pop() else {
        return Err("Operand not found".to_string());
    };
    let Some(Token::Value(left)) = stack.pop() else {
        return Err("Operand not found".to_string());
    };

    stack.push(Token::Value(Value::Result(Box::new(Expression::Addition(
        Addition { left, right },
    )))));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        parse,
        tokens::{Addition, Expression, Token, Value},
    };

    #[test]
    fn it_parses_addition() {
        let code = "1 + 2";
        // ACT
        let result = parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            Token::Value(Value::Result(Box::new(Expression::Addition(Addition {
                left: Value::Constant("1"),
                right: Value::Constant("2")
            }))))
        );
    }
}

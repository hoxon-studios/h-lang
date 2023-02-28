use crate::parser::tokens::{Expression, FunctionCall, Token, Value};

pub fn parse_call(stack: &mut Vec<Token>) -> Result<(), String> {
    let Some(expression) = stack.pop() else {
        return Err("Operand not found".to_string());
    };
    let Some(Token::Value(Value::Label(label))) = stack.pop() else {
        return Err("Operand not found".to_string());
    };
    let parameters = match expression {
        Token::Set(set) => set,
        _ => vec![expression],
    }
    .iter()
    .map(|p| match p {
        Token::Value(value) => value.clone(),
        _ => panic!("Invalid operand"),
    })
    .collect::<Vec<Value>>();

    stack.push(Token::Value(Value::Result(Box::new(
        Expression::FunctionCall(FunctionCall { label, parameters }),
    ))));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        parse,
        tokens::{Expression, FunctionCall, Token, Value},
    };

    #[test]
    fn it_parses_function() {
        let code = "some_func$(1, 2, 3)";
        // ACT
        let result = parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![Token::Value(Value::Result(Box::new(
                Expression::FunctionCall(FunctionCall {
                    label: "some_func",
                    parameters: vec![
                        Value::Constant("1"),
                        Value::Constant("2"),
                        Value::Constant("3")
                    ]
                })
            )))]
        );
    }
}

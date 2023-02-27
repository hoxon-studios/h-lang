use crate::parser::tokens::Token;

pub fn parse_group(stack: &mut Vec<Token>) -> Result<(), String> {
    let Some(left) = stack.pop() else {
        return Err("Operand not found".to_string());
    };
    let Some(right) = stack.pop() else {
        return Err("Operand not found".to_string());
    };

    let mut left = if let Token::Set(left) = left {
        left
    } else {
        vec![left]
    };
    let mut right = if let Token::Set(right) = right {
        right
    } else {
        vec![right]
    };

    right.append(&mut left);
    stack.push(Token::Set(right));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        parse,
        tokens::{Token, Value},
    };

    #[test]
    fn it_parses_group() {
        let code = "(1, 2, 3)";
        // ACT
        let result = parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            Token::Set(vec![
                Token::Value(Value::Constant("1")),
                Token::Value(Value::Constant("2")),
                Token::Value(Value::Constant("3"))
            ])
        );
    }

    #[test]
    fn it_parses_empty_group() {
        let code = "()";
        // ACT
        let result = parse(code).unwrap();
        // ASSERT
        assert_eq!(result, Token::Value(Value::Unit));
    }
}

use crate::parser::tokens::{Dereference, Expression, Token, Value};

pub fn parse_dereference(stack: &mut Vec<Token>) -> Result<(), String> {
    let Some(Token::Value(index)) = stack.pop() else {
        return Err("Invalid operand".to_string());
    };
    let Some(Token::Value(Value::Label(label))) = stack.pop() else {
        return Err("Invalid operand".to_string())
    };

    stack.push(Token::Value(Value::Result(Box::new(
        Expression::Dereference(Dereference {
            label,
            index: Box::new(index),
        }),
    ))));
    Ok(())
}

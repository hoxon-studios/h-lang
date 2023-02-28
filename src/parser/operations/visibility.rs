use crate::parser::tokens::{Definition, Token};

pub fn parse_visibility(stack: &mut Vec<Token>, export: bool) -> Result<(), String> {
    let Some(mut token) = stack.pop() else {
        return Err("Invalid operand".to_string());
    };

    match &mut token {
        Token::Definition(Definition::Function(function)) => function.export = export,
        _ => return Err("Invalid operand".to_string()),
    }

    stack.push(token);
    Ok(())
}

use crate::parser::tokens::{Declaration, LabelType, Token, Value};

pub fn parse_declaration(stack: &mut Vec<Token>) -> Result<(), String> {
    let Some(Token::Value(_type)) = stack.pop() else {
        return Err("Invalid operand".to_string());
    };
    let Some(Token::Value(Value::Label(label))) = stack.pop() else {
        return Err("Invalid operand".to_string());
    };

    let (pointer, _type) = match _type {
        Value::Label("usize") => (false, LabelType::Usize),
        Value::Reference("usize") => (true, LabelType::Usize),
        _ => return Err("Invalid operand".to_string()),
    };

    stack.push(Token::Declaration(Declaration {
        label,
        _type,
        pointer,
    }));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        parse,
        tokens::{Declaration, LabelType, Token},
    };

    #[test]
    fn it_parses_declaration() {
        let code = "some_var: usize";
        // ACT
        let result = parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![Token::Declaration(Declaration {
                label: "some_var",
                pointer: false,
                _type: LabelType::Usize
            })],
        );
    }

    #[test]
    fn it_parses_pointer_declaration() {
        let code = "some_var: &usize";
        // ACT
        let result = parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![Token::Declaration(Declaration {
                label: "some_var",
                pointer: true,
                _type: LabelType::Usize
            })],
        );
    }
}

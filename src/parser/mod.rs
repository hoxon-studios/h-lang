use self::{cursor::*, operations::*, tokens::*};

pub mod cursor;
pub mod operations;
pub mod tokens;

// This uses Shunting yard algorithm to parse the code
pub fn parse(code: &str) -> Result<Vec<Token>, String> {
    let mut output: Vec<Token> = vec![];
    let mut operators: Vec<Operator> = vec![];

    let mut cursor = code.clone();
    'outer: loop {
        cursor = skip_space(cursor);
        if let Some(code) = eat_empty(cursor) {
            output.push(Token::Value(Value::Unit));
            cursor = code;
        } else if let Some((code, operator)) = eat_operator(cursor) {
            match &operator {
                Operator::LeftParenthesis => {
                    operators.push(operator);
                }
                Operator::RightParenthesis => {
                    while let Some(operator) = operators.pop() {
                        match operator {
                            Operator::LeftParenthesis => break,
                            Operator::Operation(operation) => {
                                operation.apply(&mut output)?;
                            }
                            Operator::RightParenthesis => {
                                return Err("Open parenthesis missing".to_string())
                            }
                        }
                    }
                }
                Operator::Operation(operation) => loop {
                    let pop = if let Some(stack) = operators.last() {
                        match stack {
                            Operator::LeftParenthesis => false,
                            Operator::RightParenthesis => panic!("Invalid operator"),
                            Operator::Operation(stack) => {
                                let stack_precedence = stack.precedence();
                                let current_precedence = operation.precedence();
                                let left_associated = operation.left_associated();

                                (stack_precedence > current_precedence)
                                    || (stack_precedence == current_precedence && left_associated)
                            }
                        }
                    } else {
                        false
                    };

                    if pop {
                        match operators.pop().expect("Operator not found") {
                            Operator::LeftParenthesis => panic!("Invalid operator"),
                            Operator::RightParenthesis => panic!("Invalid operator"),
                            Operator::Operation(operation) => {
                                operation.apply(&mut output)?;
                            }
                        }
                    } else {
                        operators.push(operator.clone());
                        break;
                    }
                },
            }

            cursor = code;
        } else if let Some((code, number)) = eat_number(cursor) {
            output.push(Token::Value(Value::Constant(number)));
            cursor = code;
        } else if let Some((code, label)) = eat_label(cursor) {
            output.push(Token::Value(Value::Label(label)));
            cursor = code;
        } else {
            break 'outer;
        }
    }

    while let Some(operator) = operators.pop() {
        match operator {
            Operator::LeftParenthesis => return Err("Closing parenthesis missing".to_string()),
            Operator::RightParenthesis => panic!("Invalid operator"),
            Operator::Operation(operation) => {
                operation.apply(&mut output)?;
            }
        }
    }

    return Ok(output);
}

pub fn eat_empty(code: &str) -> Option<&str> {
    if let Some(code) = eat_token(skip_space(code), "(") {
        if let Some(code) = eat_token(skip_space(code), ")") {
            Some(code)
        } else {
            None
        }
    } else {
        None
    }
}

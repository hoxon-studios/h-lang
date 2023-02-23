use self::{
    cursor::{eat_label, eat_number, eat_token, skip_space},
    tokens::{Operation, Operator, Token},
};

mod cursor;
pub mod tokens;

// This uses Shunting yard algorithm to parse the code
pub fn tokenize(code: &str) -> Result<Vec<Token>, String> {
    let mut output: Vec<Token> = vec![];
    let mut operators: Vec<Operator> = vec![];

    let mut cursor = code.clone();
    'outer: loop {
        cursor = skip_space(cursor);
        if let Some(code) = eat_empty(cursor) {
            output.push(Token::Empty);
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
                            Operator::RightParenthesis => {
                                return Err("Open parenthesis missing".to_string())
                            }
                            Operator::Operation(operation) => {
                                output.push(Token::Operation(operation));
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
                                output.push(Token::Operation(operation));
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
            output.push(Token::Number(number));
            cursor = code;
        } else if let Some((code, label)) = eat_label(cursor) {
            output.push(Token::Label(label));
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
                output.push(Token::Operation(operation));
            }
        }
    }

    return Ok(output);
}

fn eat_empty(code: &str) -> Option<&str> {
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

fn eat_operator(code: &str) -> Option<(&str, Operator)> {
    if let Some(code) = eat_token(code, "(") {
        Some((code, Operator::LeftParenthesis))
    } else if let Some(code) = eat_token(code, ")") {
        Some((code, Operator::RightParenthesis))
    } else if let Some(code) = eat_token(code, "+") {
        Some((code, Operator::Operation(Operation::Addition)))
    } else if let Some(code) = eat_token(code, ",") {
        Some((code, Operator::Operation(Operation::Group)))
    } else if let Some(code) = eat_token(code, ";") {
        Some((code, Operator::Operation(Operation::Sequence)))
    } else if let Some(code) = eat_token(code, "let ") {
        Some((code, Operator::Operation(Operation::Let)))
    } else if let Some((code, label)) = eat_label(code) {
        if let Some(_) = eat_token(skip_space(code), "(") {
            Some((code, Operator::Operation(Operation::FunctionCall(label))))
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::frontend::{
        tokenize,
        tokens::{Operation, Token},
    };

    #[test]
    fn it_tokenize_addition() {
        let code = "1 + 2";
        // ACT
        let result = tokenize(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![
                Token::Number("1"),
                Token::Number("2"),
                Token::Operation(Operation::Addition)
            ]
        );
    }

    #[test]
    fn it_tokenize_function() {
        let code = "some_fn(1, 2)";
        // ACT
        let result = tokenize(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![
                Token::Number("1"),
                Token::Number("2"),
                Token::Operation(Operation::Group),
                Token::Operation(Operation::FunctionCall("some_fn"))
            ]
        );
    }

    #[test]
    fn it_tokenize_let_statement() {
        let code = "let some_var";
        // ACT
        let result = tokenize(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![Token::Label("some_var"), Token::Operation(Operation::Let)]
        );
    }

    #[test]
    fn it_tokenize_block() {
        let code = "let some_var; let another_var";
        // ACT
        let result = tokenize(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![
                Token::Label("some_var"),
                Token::Operation(Operation::Let),
                Token::Label("another_var"),
                Token::Operation(Operation::Let),
                Token::Operation(Operation::Sequence)
            ]
        );
    }
}

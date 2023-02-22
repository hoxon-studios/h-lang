use crate::cursor::{eat_label, eat_number, eat_token, skip_space};

use self::tokens::{Operation, Operator, Token};

pub mod tokens;

pub fn parse(code: &str) -> Result<Vec<Token>, String> {
    let mut output: Vec<Token> = vec![];
    let mut operators: Vec<Operator> = vec![];

    let mut cursor = code.clone();
    'outer: loop {
        cursor = skip_space(cursor);
        if let Some((code, operator)) = eat_operator(cursor) {
            match &operator {
                Operator::LeftParenthesis => {
                    operators.push(operator);
                }
                Operator::RightParenthesis => {
                    while let Some(operator) = operators.pop() {
                        if operator == Operator::LeftParenthesis {
                            break;
                        } else {
                            output.push(Token::Operator(operator));
                        }
                    }
                }
                Operator::Operation(operation) => loop {
                    let pop = if let Some(stack) = operators.last() {
                        match stack {
                            Operator::LeftParenthesis => false,
                            Operator::RightParenthesis => {
                                return Err("Invalid operator".to_string())
                            }
                            Operator::Operation(stack) => {
                                let stack_precedence = stack.precedence();
                                let current_precedence = operation.precedence();
                                let left_associated = operation.left_associated();

                                (stack_precedence > current_precedence)
                                    || (stack_precedence == current_precedence && left_associated)
                            }
                        }
                    } else {
                        operators.push(operator.clone());
                        false
                    };

                    if pop {
                        output.push(Token::Operator(operators.pop().expect("Token not found")));
                    } else {
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
        output.push(Token::Operator(operator));
    }

    return Ok(output);
}

fn eat_operator(code: &str) -> Option<(&str, Operator)> {
    if let Some(code) = eat_token(code, "(") {
        Some((code, Operator::LeftParenthesis))
    } else if let Some(code) = eat_token(code, ")") {
        Some((code, Operator::RightParenthesis))
    } else if let Some(code) = eat_token(code, "+") {
        Some((code, Operator::Operation(Operation::Addition)))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::frontend::{
        parse,
        tokens::{Operation, Operator, Token},
    };

    #[test]
    fn it_parses_tokens() {
        let code = "1 + 2";
        // ACT
        let result = parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![
                Token::Number("1"),
                Token::Number("2"),
                Token::Operator(Operator::Operation(Operation::Addition))
            ]
        );
    }
}

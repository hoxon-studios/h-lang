use self::{cursor::*, operations::*, tokens::*};

pub mod cursor;
pub mod operations;
pub mod tokens;

pub struct Parser<'a> {
    output: Vec<Token<'a>>,
    operators: Vec<Operator>,
}
impl<'a> Parser<'a> {
    pub fn parse(code: &str) -> Result<Vec<Token>, String> {
        let mut parser = Parser {
            output: vec![],
            operators: vec![],
        };
        let mut cursor = code.clone();
        'outer: loop {
            cursor = skip_space(cursor);
            if let Some(code) = eat_empty(cursor) {
                parser.output.push(Token::Value(Value::Unit));
                cursor = code;
            } else if let Some((code, operator)) = eat_operator(cursor) {
                match &operator {
                    Operator::LeftParenthesis => {
                        parser.operators.push(operator);
                    }
                    Operator::RightParenthesis => {
                        while let Some(operator) = parser.operators.pop() {
                            match operator {
                                Operator::LeftParenthesis => break,
                                Operator::Operation(operation) => {
                                    parser.apply(operation)?;
                                }
                                Operator::RightParenthesis => {
                                    return Err("Open parenthesis missing".to_string())
                                }
                            }
                        }
                    }
                    Operator::Operation(operation) => loop {
                        let pop = if let Some(stack) = parser.operators.last() {
                            match stack {
                                Operator::LeftParenthesis => false,
                                Operator::RightParenthesis => panic!("Invalid operator"),
                                Operator::Operation(stack) => {
                                    let stack_precedence = stack.precedence();
                                    let current_precedence = operation.precedence();
                                    let left_associated = operation.left_associated();

                                    (stack_precedence > current_precedence)
                                        || (stack_precedence == current_precedence
                                            && left_associated)
                                }
                            }
                        } else {
                            false
                        };

                        if pop {
                            match parser.operators.pop().expect("Operator not found") {
                                Operator::LeftParenthesis => panic!("Invalid operator"),
                                Operator::RightParenthesis => panic!("Invalid operator"),
                                Operator::Operation(operation) => {
                                    parser.apply(operation)?;
                                }
                            }
                        } else {
                            parser.operators.push(operator.clone());
                            break;
                        }
                    },
                }

                cursor = code;
            } else if let Some((code, number)) = eat_number(cursor) {
                parser.output.push(Token::Value(Value::Constant(number)));
                cursor = code;
            } else if let Some((code, label)) = eat_label(cursor) {
                parser.output.push(Token::Value(Value::Label(label)));
                cursor = code;
            } else {
                break 'outer;
            }
        }

        while let Some(operator) = parser.operators.pop() {
            match operator {
                Operator::LeftParenthesis => return Err("Closing parenthesis missing".to_string()),
                Operator::RightParenthesis => panic!("Invalid operator"),
                Operator::Operation(operation) => {
                    parser.apply(operation)?;
                }
            }
        }

        return Ok(parser.output);
    }

    pub fn apply(&mut self, operation: Operation) -> Result<(), String> {
        match operation {
            Operation::Reference => self.parse_reference()?,
            Operation::Visibility { export } => self.parse_visibility(export)?,
            Operation::Let => self.parse_declaration()?,
            Operation::Group => self.parse_group()?,
            Operation::Sequence => self.parse_block()?,
            Operation::Assign => self.parse_assignment()?,
            Operation::Addition => self.parse_addition()?,
            Operation::Call => self.parse_call()?,
            Operation::Function => self.parse_function()?,
            Operation::Dereference => self.parse_dereference()?,
        }

        Ok(())
    }
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

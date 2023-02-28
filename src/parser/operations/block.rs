use crate::parser::tokens::{Block, Expression, Token, Value};

pub fn parse_block(stack: &mut Vec<Token>) -> Result<(), String> {
    let Some(right) = stack.pop() else {
        return Err("Operand not found".to_string());
    };
    let Some(left) = stack.pop() else {
        return Err("Operand not found".to_string());
    };

    match (left, right) {
        (Token::Statement(left), Token::Statement(right)) => stack.push(Token::Value(
            Value::Result(Box::new(Expression::Block(Block {
                body: vec![left, right],
                result: Value::Unit,
            }))),
        )),
        (Token::Statement(left), Token::Value(right)) => stack.push(Token::Value(Value::Result(
            Box::new(Expression::Block(Block {
                body: vec![left],
                result: right,
            })),
        ))),
        (Token::Value(Value::Result(result)), Token::Statement(right)) => match *result {
            Expression::Block(mut block) => {
                block.body.push(right);
                stack.push(Token::Value(Value::Result(Box::new(Expression::Block(
                    block,
                )))));
            }
            _ => return Err("Invalid operand".to_string()),
        },
        (Token::Value(Value::Result(result)), Token::Value(right)) => match *result {
            Expression::Block(mut block) => {
                block.result = right;
                stack.push(Token::Value(Value::Result(Box::new(Expression::Block(
                    block,
                )))));
            }
            _ => return Err("Invalid operand".to_string()),
        },
        _ => return Err("Invalid operand".to_string()),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        parse,
        tokens::{
            Addition, Assignment, Block, Declaration, Expression, LabelType, Statement, Token,
            Value,
        },
    };

    #[test]
    fn it_parses_block() {
        let code = "some_var: usize = 1; another_var: usize = 2";
        // ACT
        let result = parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![Token::Value(Value::Result(Box::new(Expression::Block(
                Block {
                    body: vec![
                        Statement::Assignment(Assignment {
                            address: Box::new(Token::Declaration(Declaration {
                                label: "some_var",
                                pointer: false,
                                _type: LabelType::Usize
                            })),
                            value: Value::Constant("1")
                        }),
                        Statement::Assignment(Assignment {
                            address: Box::new(Token::Declaration(Declaration {
                                label: "another_var",
                                pointer: false,
                                _type: LabelType::Usize
                            })),
                            value: Value::Constant("2")
                        })
                    ],
                    result: Value::Unit
                }
            ))))]
        );
    }

    #[test]
    fn it_parses_block_with_result() {
        let code = "some_var: usize = 1; another: usize = 2; some_var + another";
        // ACT
        let result = parse(code).unwrap();
        // ASSERT
        assert_eq!(
            result,
            vec![Token::Value(Value::Result(Box::new(Expression::Block(
                Block {
                    body: vec![
                        Statement::Assignment(Assignment {
                            address: Box::new(Token::Declaration(Declaration {
                                label: "some_var",
                                pointer: false,
                                _type: LabelType::Usize
                            })),
                            value: Value::Constant("1")
                        }),
                        Statement::Assignment(Assignment {
                            address: Box::new(Token::Declaration(Declaration {
                                label: "another",
                                pointer: false,
                                _type: LabelType::Usize
                            })),
                            value: Value::Constant("2")
                        })
                    ],
                    result: Value::Result(Box::new(Expression::Addition(Addition {
                        left: Value::Label("some_var"),
                        right: Value::Label("another")
                    })))
                }
            ))))]
        );
    }
}

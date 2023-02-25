use super::{Block, Evaluation, Expression};

pub fn parse_block(stack: &mut Vec<Expression>) -> Result<(), String> {
    let Some(right) = stack.pop() else {
        return Err("Operand not found".to_string());
    };
    let Some(left) = stack.pop() else {
        return Err("Operand not found".to_string());
    };

    match (left, right) {
        (Expression::Statement(left), Expression::Statement(right)) => {
            stack.push(Expression::Result(Box::new(Evaluation::Block(Block {
                body: vec![*left, *right],
                result: Expression::Unit,
            }))))
        }
        (Expression::Statement(left), right) => {
            stack.push(Expression::Result(Box::new(Evaluation::Block(Block {
                body: vec![*left],
                result: right,
            }))));
        }
        (Expression::Result(evaluation), Expression::Statement(right)) => match *evaluation {
            Evaluation::Block(mut block) => {
                block.body.push(*right);
                stack.push(Expression::Result(Box::new(Evaluation::Block(block))));
            }
            _ => return Err("Invalid operand".to_string()),
        },
        (Expression::Result(evaluation), right) => match *evaluation {
            Evaluation::Block(mut block) => {
                block.result = right;
                stack.push(Expression::Result(Box::new(Evaluation::Block(block))));
            }
            _ => return Err("Invalid operand".to_string()),
        },
        _ => return Err("Invalid operand".to_string()),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        frontend::tokenize,
        intermediate::{
            expressions::{
                Addition, Assignment, Block, Evaluation, Expression, LetStatement, Statement,
            },
            parse,
        },
    };

    #[test]
    fn it_parses_block() {
        let code = "let some_var = 1; let another_var = 2";
        let tokens = tokenize(code).unwrap();
        // ACT
        let result = parse(tokens).unwrap();
        // ASSERT
        assert_eq!(
            result,
            Expression::Result(Box::new(Evaluation::Block(Block {
                body: vec![
                    Statement::Let(LetStatement(Assignment {
                        label: "some_var".to_string(),
                        value: Expression::Constant("1".to_string())
                    })),
                    Statement::Let(LetStatement(Assignment {
                        label: "another_var".to_string(),
                        value: Expression::Constant("2".to_string())
                    })),
                ],
                result: Expression::Unit
            })))
        );
    }

    #[test]
    fn it_parses_block_with_result() {
        let code = "let some_var = 1; let another = 2; some_var + another";
        let tokens = tokenize(code).unwrap();
        // ACT
        let result = parse(tokens).unwrap();
        // ASSERT
        assert_eq!(
            result,
            Expression::Result(Box::new(Evaluation::Block(Block {
                body: vec![
                    Statement::Let(LetStatement(Assignment {
                        label: "some_var".to_string(),
                        value: Expression::Constant("1".to_string())
                    })),
                    Statement::Let(LetStatement(Assignment {
                        label: "another".to_string(),
                        value: Expression::Constant("2".to_string())
                    }))
                ],
                result: Expression::Result(Box::new(Evaluation::Addition(Addition {
                    left: Expression::Label("some_var".to_string()),
                    right: Expression::Label("another".to_string())
                })))
            })))
        );
    }
}

use super::{Block, Expression, Statement};

pub fn parse_block(stack: &mut Vec<Expression>) -> Result<(), String> {
    let Some(right) = stack.pop() else {
        return Err("Operand not found".to_string());
    };

    match right {
        Expression::Statement(Statement::Block(_)) => return Err("Invalid operand".to_string()),
        Expression::Statement(right) => {
            let Some(left) = stack.pop() else {
                return Err("Operand not found".to_string());
            };
            match left {
                Expression::Statement(Statement::Block(mut block)) => {
                    block.0.push(right);
                    stack.push(Expression::Statement(Statement::Block(block)));
                }
                Expression::Statement(left) => {
                    stack.push(Expression::Statement(Statement::Block(Block(vec![
                        left, right,
                    ]))))
                }
                _ => return Err("Invalid operand".to_string()),
            }
        }
        _ => return Err("Invalid operand".to_string()),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        frontend::tokenize,
        intermediate::{
            expressions::{Block, Expression, LetStatement, Statement},
            parse,
        },
    };

    #[test]
    fn it_parses_block() {
        let code = "let some_var; let another_var";
        let tokens = tokenize(code).unwrap();
        // ACT
        let result = parse(tokens).unwrap();
        // ASSERT
        assert_eq!(
            result,
            Expression::Statement(Statement::Block(Block(vec![
                Statement::Let(LetStatement {
                    label: "some_var".to_string()
                }),
                Statement::Let(LetStatement {
                    label: "another_var".to_string()
                })
            ])))
        );
    }
}

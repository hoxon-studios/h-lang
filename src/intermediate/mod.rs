use crate::frontend::tokens::{Operation, Operator, Token};

use self::expressions::{Addition, Evaluation, Expression};

pub mod expressions;

pub fn parse(tokens: Vec<Token>) -> Result<Expression, String> {
    let mut expressions: Vec<Expression> = vec![];

    for token in &tokens {
        match token {
            &Token::Number(value) => expressions.push(Expression::Constant(value.to_string())),
            &Token::Label(value) => expressions.push(Expression::Label(value.to_string())),
            Token::Operator(operator) => match operator {
                Operator::LeftParenthesis => {}
                Operator::RightParenthesis => {}
                Operator::Operation(operation) => match operation {
                    Operation::Addition => parse_addition(&mut expressions)?,
                },
            },
            Token::Keyword(_) => todo!(),
        }
    }

    if expressions.len() > 1 {
        Err("Failed to parse expressions".to_string())
    } else {
        Ok(expressions.pop().expect("Failed to parse expressions"))
    }
}

fn parse_addition(stack: &mut Vec<Expression>) -> Result<(), String> {
    let Some(right) = stack.pop() else {
        return Err("Operand not found".to_string());
    };
    let Some(left) = stack.pop() else {
        return Err("Operand not found".to_string());
    };

    if let Expression::Constant(_) | Expression::Label(_) | Expression::Result(_) = left {
        if let Expression::Constant(_) | Expression::Label(_) | Expression::Result(_) = right {
            stack.push(Expression::Result(Box::new(Evaluation::Addition(
                Addition { left, right },
            ))))
        } else {
            return Err("Invalid right operand".to_string());
        }
    } else {
        return Err("Invalid left operand".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        frontend::tokens::{Operation, Operator, Token},
        intermediate::expressions::Addition,
    };

    use super::{
        expressions::{Evaluation, Expression},
        parse,
    };

    #[test]
    fn it_parses_addition() {
        let tokens: Vec<Token> = vec![
            Token::Number("1"),
            Token::Number("2"),
            Token::Operator(Operator::Operation(Operation::Addition)),
        ];
        // ACT
        let result = parse(tokens).unwrap();
        // ASSERT
        assert_eq!(
            result,
            Expression::Result(Box::new(Evaluation::Addition(Addition {
                left: Expression::Constant("1".to_string()),
                right: Expression::Constant("2".to_string())
            })))
        );
    }
}

use crate::frontend::tokens::{Operation, Token};

use self::expressions::{
    addition::parse_addition, assignment::parse_assignment, block::parse_block,
    function_call::parse_function_call, group::parse_group, let_statement::parse_let_statement,
    Expression, ExpressionSet,
};

pub mod expressions;

pub fn parse(tokens: Vec<Token>) -> Result<Expression, String> {
    let mut stack: Vec<Expression> = vec![];

    for token in &tokens {
        match token {
            &Token::Number(value) => stack.push(Expression::Constant(value.to_string())),
            &Token::Label(value) => stack.push(Expression::Label(value.to_string())),
            Token::Empty => stack.push(Expression::Set(ExpressionSet(vec![]))),
            Token::Operation(operation) => match operation {
                Operation::Group => parse_group(&mut stack)?,
                Operation::Addition => parse_addition(&mut stack)?,
                Operation::FunctionCall(label) => parse_function_call(&mut stack, label)?,
                Operation::Let => parse_let_statement(&mut stack)?,
                Operation::Assign => parse_assignment(&mut stack)?,
                Operation::Sequence => parse_block(&mut stack)?,
            },
        }
    }

    if stack.len() > 1 {
        Err("Failed to parse expressions".to_string())
    } else {
        Ok(stack.pop().expect("Failed to parse expressions"))
    }
}

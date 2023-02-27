use self::{
    addition::parse_addition, assignment::parse_assignment, block::parse_block, call::parse_call,
    declaration::parse_declaration, function::parse_function, group::parse_group,
};

use super::{cursor::eat_token, tokens::Token};

pub mod addition;
pub mod assignment;
pub mod block;
pub mod call;
pub mod declaration;
pub mod function;
pub mod group;

#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    LeftParenthesis,
    RightParenthesis,
    Operation(Operation),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operation {
    Let,
    Group,
    Sequence,
    Assign,
    Addition,
    Call,
    Function,
}

impl Operation {
    pub fn apply(&self, stack: &mut Vec<Token>) -> Result<(), String> {
        match self {
            Operation::Let => parse_declaration(stack)?,
            Operation::Group => parse_group(stack)?,
            Operation::Sequence => parse_block(stack)?,
            Operation::Assign => parse_assignment(stack)?,
            Operation::Addition => parse_addition(stack)?,
            Operation::Call => parse_call(stack)?,
            Operation::Function => parse_function(stack)?,
        }

        Ok(())
    }

    pub fn precedence(&self) -> usize {
        match self {
            Operation::Function => 0,
            Operation::Sequence => 1,
            Operation::Assign => 2,
            Operation::Call => 3,
            Operation::Group => 4,
            Operation::Let => 5,
            Operation::Addition => 6,
        }
    }
    pub fn left_associated(&self) -> bool {
        match self {
            Operation::Function => true,
            Operation::Sequence => true,
            Operation::Let => true,
            Operation::Assign => false,
            Operation::Group => true,
            Operation::Addition => true,
            Operation::Call => true,
        }
    }
}

pub fn eat_operator(code: &str) -> Option<(&str, Operator)> {
    if let Some(code) = eat_token(code, "(") {
        Some((code, Operator::LeftParenthesis))
    } else if let Some(code) = eat_token(code, ")") {
        Some((code, Operator::RightParenthesis))
    } else if let Some(code) = eat_token(code, "=") {
        Some((code, Operator::Operation(Operation::Assign)))
    } else if let Some(code) = eat_token(code, "+") {
        Some((code, Operator::Operation(Operation::Addition)))
    } else if let Some(code) = eat_token(code, ",") {
        Some((code, Operator::Operation(Operation::Group)))
    } else if let Some(code) = eat_token(code, ";") {
        Some((code, Operator::Operation(Operation::Sequence)))
    } else if let Some(code) = eat_token(code, ":") {
        Some((code, Operator::Operation(Operation::Let)))
    } else if let Some(code) = eat_token(code, "$") {
        Some((code, Operator::Operation(Operation::Call)))
    } else if let Some(code) = eat_token(code, "fn ") {
        Some((code, Operator::Operation(Operation::Function)))
    } else {
        None
    }
}

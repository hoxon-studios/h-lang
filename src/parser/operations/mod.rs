use self::{
    addition::parse_addition, assignment::parse_assignment, block::parse_block, call::parse_call,
    declaration::parse_declaration, dereference::parse_dereference, function::parse_function,
    group::parse_group, reference::parse_reference, visibility::parse_visibility,
};

use super::{cursor::eat_token, tokens::Token};

pub mod addition;
pub mod assignment;
pub mod block;
pub mod call;
pub mod declaration;
pub mod dereference;
pub mod function;
pub mod group;
pub mod reference;
pub mod visibility;

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
    Reference,
    Dereference,
    Visibility { export: bool },
}

impl Operation {
    pub fn apply(&self, stack: &mut Vec<Token>) -> Result<(), String> {
        match self {
            Operation::Reference => parse_reference(stack)?,
            Operation::Visibility { export } => parse_visibility(stack, *export)?,
            Operation::Let => parse_declaration(stack)?,
            Operation::Group => parse_group(stack)?,
            Operation::Sequence => parse_block(stack)?,
            Operation::Assign => parse_assignment(stack)?,
            Operation::Addition => parse_addition(stack)?,
            Operation::Call => parse_call(stack)?,
            Operation::Function => parse_function(stack)?,
            Operation::Dereference => parse_dereference(stack)?,
        }

        Ok(())
    }

    pub fn precedence(&self) -> usize {
        match self {
            Operation::Visibility { .. } => 0,
            Operation::Function => 1,
            Operation::Sequence => 2,
            Operation::Assign => 3,
            Operation::Call => 4,
            Operation::Group => 5,
            Operation::Let => 6,
            Operation::Addition => 7,
            Operation::Reference => 8,
            Operation::Dereference => 8,
        }
    }
    pub fn left_associated(&self) -> bool {
        match self {
            Operation::Visibility { .. } => true,
            Operation::Function => true,
            Operation::Sequence => true,
            Operation::Let => true,
            Operation::Assign => false,
            Operation::Group => true,
            Operation::Addition => true,
            Operation::Call => true,
            Operation::Reference => true,
            Operation::Dereference => true,
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
    } else if let Some(code) = eat_token(code, "&") {
        Some((code, Operator::Operation(Operation::Reference)))
    } else if let Some(code) = eat_token(code, "#") {
        Some((code, Operator::Operation(Operation::Dereference)))
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
    } else if let Some(code) = eat_token(code, "public ") {
        Some((
            code,
            Operator::Operation(Operation::Visibility { export: true }),
        ))
    } else if let Some(code) = eat_token(code, "private ") {
        Some((
            code,
            Operator::Operation(Operation::Visibility { export: false }),
        ))
    } else {
        None
    }
}

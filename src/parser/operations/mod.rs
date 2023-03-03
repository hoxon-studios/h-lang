use super::cursor::eat_token;

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

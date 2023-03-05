use super::cursor::eat_token;

pub mod addition;
pub mod assignment;
pub mod block;
pub mod call;
pub mod conditional;
pub mod declaration;
pub mod dereference;
pub mod export;
pub mod function;
pub mod group;
pub mod reference;

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
    If,
    Else,
    Visibility { export: bool },
}

impl Operation {
    pub fn precedence(&self) -> usize {
        match self {
            Operation::Visibility { .. } => 0,
            Operation::Function => 1,
            Operation::Sequence => 2,
            Operation::Else => 3,
            Operation::If => 4,
            Operation::Assign => 5,
            Operation::Call => 6,
            Operation::Group => 7,
            Operation::Let => 8,
            Operation::Addition => 9,
            Operation::Reference => 10,
            Operation::Dereference => 11,
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
            Operation::If => true,
            Operation::Else => true,
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
    } else if let Some(code) = eat_token(code, "if ") {
        Some((code, Operator::Operation(Operation::If)))
    } else if let Some(code) = eat_token(code, "else ") {
        Some((code, Operator::Operation(Operation::Else)))
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

use super::cursor::eat_token;

pub mod addition;
pub mod assignment;
pub mod bitwise_or;
pub mod block;
pub mod call;
pub mod conditional;
pub mod declaration;
pub mod dereference;
pub mod export;
pub mod function;
pub mod group;
pub mod r#loop;
pub mod reference;
pub mod r#string;
pub mod r#struct;

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
    Struct,
    String,
    Reference,
    Dereference,
    If,
    Else,
    Loop,
    Break,
    BitwiseOr,
    Visibility { export: bool },
}

impl Operation {
    pub fn precedence(&self) -> usize {
        match self {
            Operation::Visibility { .. } => 0,
            Operation::Struct => 1,
            Operation::Function => 1,
            Operation::String => 1,
            Operation::Sequence => 2,
            Operation::Loop => 3,
            Operation::Else => 4,
            Operation::If => 5,
            Operation::Assign => 6,
            Operation::Call => 7,
            Operation::Group => 8,
            Operation::Let => 9,
            Operation::Addition => 10,
            Operation::BitwiseOr => 11,
            Operation::Reference => 12,
            Operation::Dereference => 13,
            Operation::Break => 14,
        }
    }
    pub fn left_associated(&self) -> bool {
        match self {
            Operation::Visibility { .. } => true,
            Operation::Function => true,
            Operation::String => true,
            Operation::Struct => true,
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
            Operation::Loop => true,
            Operation::Break => true,
            Operation::BitwiseOr => true,
        }
    }
}

pub fn eat_operator(code: &str) -> Option<(&str, Operator)> {
    if let Some(code) = eat_token(code, "(") {
        Some((code, Operator::LeftParenthesis))
    } else if let Some(code) = eat_token(code, ")") {
        Some((code, Operator::RightParenthesis))
    } else if let Some(code) = eat_token(code, "|") {
        Some((code, Operator::Operation(Operation::BitwiseOr)))
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
    } else if let Some(code) = eat_token(code, "struct ") {
        Some((code, Operator::Operation(Operation::Struct)))
    } else if let Some(code) = eat_token(code, "fn ") {
        Some((code, Operator::Operation(Operation::Function)))
    } else if let Some(code) = eat_token(code, "string ") {
        Some((code, Operator::Operation(Operation::String)))
    } else if let Some(code) = eat_token(code, "loop ") {
        Some((code, Operator::Operation(Operation::Loop)))
    } else if let Some(code) = eat_token(code, "break ") {
        Some((code, Operator::Operation(Operation::Break)))
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

#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    Number(&'a str),
    Label(&'a str),
    Empty,
    Operation(Operation),
}

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
    Product,
    Typify,
    Call,
    FunctionDefinition,
    Export,
}

impl Operation {
    pub fn precedence(&self) -> usize {
        match self {
            Operation::Export => 0,
            Operation::FunctionDefinition => 1,
            Operation::Sequence => 2,
            Operation::Let => 3,
            Operation::Assign => 4,
            Operation::Call => 5,
            Operation::Group => 6,
            Operation::Typify => 7,
            Operation::Addition => 8,
            Operation::Product => 9,
        }
    }
    pub fn left_associated(&self) -> bool {
        match self {
            Operation::Sequence => true,
            Operation::Let => true,
            Operation::Assign => false,
            Operation::Group => true,
            Operation::Addition => true,
            Operation::Product => true,
            Operation::Typify => true,
            Operation::FunctionDefinition => true,
            Operation::Export => true,
            Operation::Call => true,
        }
    }
}

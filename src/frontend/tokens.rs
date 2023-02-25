#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    Number(&'a str),
    Label(&'a str),
    Empty,
    Operation(Operation<'a>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operator<'a> {
    LeftParenthesis,
    RightParenthesis,
    Operation(Operation<'a>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operation<'a> {
    Let,
    Group,
    Sequence,
    FunctionCall(&'a str),
    Assign,
    Addition,
}

impl<'a> Operation<'a> {
    pub fn precedence(&self) -> usize {
        match self {
            Operation::Sequence => 0,
            Operation::Let => 1,
            Operation::Assign => 2,
            Operation::FunctionCall(_) => 3,
            Operation::Group => 4,
            Operation::Addition => 5,
        }
    }
    pub fn left_associated(&self) -> bool {
        match self {
            Operation::Sequence => true,
            Operation::Let => true,
            Operation::Assign => false,
            Operation::Addition => true,
            Operation::Group => true,
            Operation::FunctionCall(_) => true,
        }
    }
}

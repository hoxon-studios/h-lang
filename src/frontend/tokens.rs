#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    Number(&'a str),
    Label(&'a str),
    Keyword(&'static str),
    Operator(Operator),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    LeftParenthesis,
    RightParenthesis,
    Operation(Operation),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operation {
    Addition,
}

impl Operation {
    pub fn precedence(&self) -> usize {
        match self {
            Operation::Addition => 0,
        }
    }
    pub fn left_associated(&self) -> bool {
        match self {
            Operation::Addition => true,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Constant(String),
    Label(String),
    Result(Box<Evaluation>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Evaluation {
    Addition(Addition),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Addition {
    pub left: Expression,
    pub right: Expression,
}

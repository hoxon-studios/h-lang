pub mod addition;
pub mod assignment;
pub mod block;
pub mod call;
pub mod group;
pub mod let_statement;

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Unit,
    Constant(String),
    Label(String),
    Set(ExpressionSet),
    Result(Box<Evaluation>),
    Statement(Box<Statement>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    Assignment(Assignment),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Assignment {
    pub label: String,
    pub value: Expression,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Block {
    pub body: Vec<Statement>,
    pub result: Expression,
}

#[derive(PartialEq, Debug, Clone)]
pub struct LetStatement(pub Assignment);

#[derive(PartialEq, Debug, Clone)]
pub struct ExpressionSet(pub Vec<Expression>);

#[derive(PartialEq, Debug, Clone)]
pub enum Evaluation {
    Block(Block),
    Addition(Addition),
    FunctionCall(FunctionCall),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Addition {
    pub left: Expression,
    pub right: Expression,
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionCall {
    pub parameters: ExpressionSet,
    pub label: String,
}

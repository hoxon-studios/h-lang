pub mod addition;
pub mod block;
pub mod function;
pub mod group;
pub mod let_statement;

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Constant(String),
    Label(String),
    Set(ExpressionSet),
    Result(Box<Evaluation>),
    Statement(Statement),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Block {
    pub body: Vec<Statement>,
    pub result: Option<Expression>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct LetStatement {
    pub label: String,
}

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

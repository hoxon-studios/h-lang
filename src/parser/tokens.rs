#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    Value(Value<'a>),
    Set(Vec<Token<'a>>),
    Definition(Definition<'a>),
    Declaration(Declaration<'a>),
    Statement(Statement<'a>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Value<'a> {
    Constant(&'a str),
    Label(&'a str),
    Unit,
    Result(Box<Expression<'a>>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Declaration<'a> {
    pub label: &'a str,
    pub _type: &'a str,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement<'a> {
    Assignment(Assignment<'a>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression<'a> {
    Block(Block<'a>),
    Addition(Addition<'a>),
    FunctionCall(FunctionCall<'a>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionCall<'a> {
    pub parameters: Vec<Value<'a>>,
    pub label: &'a str,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Addition<'a> {
    pub left: Value<'a>,
    pub right: Value<'a>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Block<'a> {
    pub body: Vec<Statement<'a>>,
    pub result: Value<'a>,
}
#[derive(PartialEq, Debug, Clone)]
pub struct Assignment<'a> {
    pub address: Box<Token<'a>>,
    pub value: Value<'a>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Definition<'a> {
    Function(Function<'a>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Function<'a> {
    pub label: &'a str,
    pub parameters: Vec<Declaration<'a>>,
    pub body: Block<'a>,
}

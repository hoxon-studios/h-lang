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
    Reference(&'a str),
    Unit,
    Result(Box<Expression<'a>>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Dereference<'a> {
    pub label: &'a str,
    pub index: Box<Value<'a>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Declaration<'a> {
    pub label: &'a str,
    pub pointer: bool,
    pub _type: LabelType,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LabelType {
    Usize,
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
    Dereference(Dereference<'a>),
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
    pub export: bool,
    pub label: &'a str,
    pub parameters: Vec<Declaration<'a>>,
    pub body: Block<'a>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Export<'a> {
    pub definition: Definition<'a>,
}

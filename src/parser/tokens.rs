use super::context::SymbolType;

#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    Unit,
    Constant(Constant<'a>),
    Id(Id<'a>),
    String(StringLiteral<'a>),
    Set(TokenSet<'a>),
    Statement(Statement),
    Label(Label<'a>),
    Result(Code),
    Definition(Definition<'a>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Constant<'a>(pub &'a str);

#[derive(PartialEq, Debug, Clone)]
pub struct Id<'a>(pub &'a str);

#[derive(PartialEq, Debug, Clone)]
pub struct StringLiteral<'a>(pub &'a str);

#[derive(PartialEq, Debug, Clone)]
pub struct TokenSet<'a>(pub Vec<Token<'a>>);

#[derive(PartialEq, Debug, Clone)]
pub struct Statement {
    pub body: Code,
    pub exit_label: Option<String>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Definition<'a> {
    pub name: &'a str,
    pub definition: Code,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Code(pub String);

#[derive(PartialEq, Debug, Clone)]
pub struct Label<'a> {
    pub id: &'a str,
    pub _type: SymbolType,
    pub address: LabelAddress<'a>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LabelAddress<'a> {
    Global { label: &'a str },
    Stack { position: usize, offset: usize },
}

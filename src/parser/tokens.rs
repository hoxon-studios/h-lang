use super::context::SymbolType;

#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    Unit,
    Constant(&'a str),
    Id(&'a str),
    String(&'a str),
    Set(Vec<Token<'a>>),
    Statement {
        body: String,
        exit_label: Option<String>,
    },
    Label(Label<'a>),
    Result(String),
    Item {
        name: &'a str,
        definition: String,
    },
}

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

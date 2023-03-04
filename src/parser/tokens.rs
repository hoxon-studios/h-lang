#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    Unit,
    Constant(&'a str),
    Label(&'a str),
    Set(Vec<Token<'a>>),
    Instruction(String),
    Result(String),
    Item { name: &'a str, definition: String },
}

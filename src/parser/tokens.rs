#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    Unit,
    Constant(&'a str),
    Label(&'a str),
    Set(Vec<Token<'a>>),
    Statement {
        body: String,
        exit_label: Option<String>,
    },
    Result(String),
    Item {
        name: &'a str,
        definition: String,
    },
}

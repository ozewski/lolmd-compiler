#[derive(Debug, Clone)]
pub enum Node {
    DocOpen, DocClose,
    Comment(String),
    HeadOpen, HeadClose,
    Title(String),
    ParagraphOpen, ParagraphClose,
    ListOpen, ListClose,
    ItemOpen, ItemClose,
    BoldOpen, BoldClose,
    Link(String),
    Newline,
    Text(String),
    VarDef { name: String, value: String },
    VarUse(String),
}

#[derive(Debug, Clone)]
pub enum Node {
    // Open/close pairs (containers)
    DocOpen, DocClose,
    HeadOpen, HeadClose,
    TitleOpen, TitleClose,
    CommentOpen, CommentClose,
    ParagraphOpen, ParagraphClose,
    ListOpen, ListClose,
    ItemOpen, ItemClose,
    BoldOpen, BoldClose,
    LinkOpen, LinkClose,

    // Single nodes (leaves)
    Newline,
    Text(String),
    VarDef { name: String, value: String },
    VarUse(String),
}
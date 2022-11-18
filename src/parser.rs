use std::path::Path;

pub enum Rule {
    Indent(u32),
    Include(String),
    Doctype(String),
    Tag, // todo
    Comment,
    Text,
    EndOfFile
}

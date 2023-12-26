#[derive(Debug, PartialEq, Clone)]
pub enum GitObject {
    Blob,
    Tree,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileTypes {
    Utf8,
    NonUTF8,
}

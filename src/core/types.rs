#[derive(Debug)]
pub enum GitObject {
    Blob,
    Tree,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileTypes {
    Utf8,
    NonUTF8,
}

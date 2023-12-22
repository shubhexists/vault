#[derive(Debug)]
pub enum GitObject {
    Blob,
    Tree,
}

#[derive(Debug, Clone)]
pub enum FileTypes {
    Utf8,
    NonUTF8,
}

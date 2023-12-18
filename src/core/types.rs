use super::blob::Blob;
use super::tree::Tree;

#[derive(Debug)]
pub enum GitObject {
    Blob(Blob),
    Tree(Tree),
}

#[derive(Debug, Clone)]
pub enum FileTypes {
    Utf8,
    NonUTF8,
}

use super::blob::Blob;
use super::tree::Tree;

#[derive(Debug)]
pub enum GitObject {
    Blob(Blob),
    Tree(Tree),
}

#[derive(Debug, Clone)]
pub struct FileType {
    pub ftype: Option<String>,
}

#[derive(Debug, Clone)]
pub enum FileTypes {
    Utf8(FileType),
    NonUTF8(FileType),
}

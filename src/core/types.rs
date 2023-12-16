use super::blob::Blob;
use super::tree::Tree;

#[derive(Debug)]
pub enum GitObject {
    Blob(Blob),
    Tree(Tree)
}

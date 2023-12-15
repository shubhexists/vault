use std::io;

use super::types::GitObject;

/* SAMPLE TREE FILE

blob 103\0fileA.txt\0123456789abcdef0123456789abcdef01234567
tree 41\0folderA\0ab89abcdef0123456789abcdef0123456789ab
 */

#[derive(Debug)]
pub struct Tree {
    pub entries: Vec<TreeEntry>,
    pub content: String,
    content_size: i32,
}

#[derive(Debug)]
pub struct TreeEntry {
    pub name: String,
    pub object: GitObject,
    pub hashed_path: String,
}

impl Tree {
    pub fn make_tree(entries: Vec<TreeEntry>) -> io::Result<Tree> {
        let mut tree_content: String = String::new();
        for tree_element in &entries {
            let tree_entry_content: String = Tree::get_contents_of_tree_element(tree_element);
            tree_content.push_str(&tree_entry_content);
            tree_content.push('\n');
        }
        let content_size = String::from(tree_content.clone()).chars().count() as i32;
        Ok(Tree {
            entries,
            content: tree_content,
            content_size,
        })
    }

    fn get_contents_of_tree_element(entry: &TreeEntry) -> String {
        let mut file_content: String = String::new();
        match &entry.object {
            GitObject::Blob(blob) => {
                file_content.push_str("blob ");
                file_content.push_str(&blob.content_size.to_string());
                file_content.push('\0');
            }
            GitObject::Tree(tree) => {
                file_content.push_str("tree ");
                file_content.push_str(&tree.content_size.to_string());
                file_content.push('\0');
            }
            GitObject::Commit(_commit) => {
                panic!("Element 'Commit' can not be inside 'Tree'")
            }
        }
        file_content.push_str(&entry.name);
        file_content.push('\0');
        file_content.push_str(&entry.hashed_path);
        file_content
    }
}

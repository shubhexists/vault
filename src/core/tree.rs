use super::types::GitObject;

/* SAMPLE TREE FILE

blob \0fileA.txt\0123456789abcdef0123456789abcdef01234567
tree \0folderA\0ab89abcdef0123456789abcdef0123456789ab
 */

#[derive(Debug)]
pub struct Tree {
    pub entries: Vec<TreeEntry>,
    pub content: String,
    pub content_size: i32,
}

#[derive(Debug)]
pub struct TreeEntry {
    pub name: String,
    pub object: GitObject,
    pub hashed_path: String,
}

impl Tree {
    pub fn make_tree(entries: Vec<TreeEntry>) -> Tree {
        let mut tree_content: String = String::new();
        for tree_element in &entries {
            let tree_entry_content: String = Tree::get_contents_of_tree_element(tree_element);
            tree_content.push_str(&tree_entry_content);
            tree_content.push('\n');
        }
        let content_size: i32 = String::from(tree_content.clone()).chars().count() as i32;
        Tree {
            entries,
            content: tree_content,
            content_size,
        }
    }

    fn get_contents_of_tree_element(entry: &TreeEntry) -> String {
        let mut file_content: String = String::new();
        match &entry.object {
            GitObject::Blob => {
                file_content.push_str("blob ");
            }
            GitObject::Tree => {
                file_content.push_str("tree ");
            }
        }
        file_content.push('\0');
        file_content.push_str(&entry.name);
        file_content.push('\0');
        file_content.push_str(&entry.hashed_path);
        file_content
    }

    pub fn get_tree_from_content(tree_content: &String) -> Tree {
        let content_size: i32 = tree_content.chars().count() as i32;
        let break_: Vec<&str> = tree_content.split("\n").collect();
        let break_by_new_line: &[&str] = &break_[..break_.len() - 1];
        let mut tree_entry_contents: Vec<TreeEntry> = Vec::new();
        for item in break_by_new_line {
            let tree_entry_content: Vec<&str> = item.split("\0").collect();
            //REMOVE THIS
            println!("{:?}", tree_entry_content);
            let is_valid_tree_entry: bool = TreeEntry::check_valid_tree_entry(&tree_entry_content);
            if is_valid_tree_entry {
                let tree_entry_object: TreeEntry =
                    TreeEntry::parse_tree_entry_contents(&tree_entry_content);
                tree_entry_contents.push(tree_entry_object);
            } else {
                panic!("Unable to get tree object!")
            }
        }
        Tree {
            entries: tree_entry_contents,
            content: tree_content.to_string(),
            content_size,
        }
    }
}

impl TreeEntry {
    pub fn check_valid_tree_entry(tree_entry_content: &Vec<&str>) -> bool {
        if tree_entry_content.len() == 3 {
            if tree_entry_content[0] == "blob " || tree_entry_content[0] == "tree " {
                return true;
            }
        }
        false
    }

    pub fn parse_tree_entry_contents(tree_entry_content: &Vec<&str>) -> TreeEntry {
        let filetype = || {
            if tree_entry_content[0] == "blob " {
                GitObject::Blob
            } else {
                GitObject::Tree
            }
        };
        let filename: &str = tree_entry_content[1];
        let hash_string: &str = tree_entry_content[2];
        TreeEntry {
            name: filename.to_string(),
            object: filetype(),
            hashed_path: hash_string.to_string(),
        }
    }
}

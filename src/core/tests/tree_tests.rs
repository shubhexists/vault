use crate::core::{
    tree::{Tree, TreeEntry},
    types::GitObject,
};

#[test]
fn test_make_tree_with_blob_entry() {
    let blob_entry: TreeEntry = TreeEntry {
        name: String::from("fileA.txt"),
        object: GitObject::Blob,
        hashed_path: String::from("0123456789abcdef0123456789abcdef01234567"),
    };
    let tree: Tree = Tree::make_tree(vec![blob_entry.clone()]);
    let expected_content: &str = "blob \0fileA.txt\00123456789abcdef0123456789abcdef01234567\n";
    assert_eq!(tree.content, expected_content);
    assert_eq!(tree.content_size, expected_content.len() as i32);
    assert_eq!(tree.entries.len(), 1);
    assert_eq!(tree.entries[0], blob_entry);
}

#[test]
fn test_make_tree_with_tree_entry() {
    let tree_entry: TreeEntry = TreeEntry {
        name: String::from("folderA"),
        object: GitObject::Tree,
        hashed_path: String::from("ab89abcdef0123456789abcdef0123456789ab"),
    };
    let tree: Tree = Tree::make_tree(vec![tree_entry.clone()]);
    let expected_content: &str = "tree \0folderA\0ab89abcdef0123456789abcdef0123456789ab\n";
    assert_eq!(tree.content, expected_content);
    assert_eq!(tree.content_size, expected_content.len() as i32);
    assert_eq!(tree.entries.len(), 1);
    assert_eq!(tree.entries[0], tree_entry);
}

#[test]
fn test_get_tree_from_content_with_blob_entry() {
    let blob_entry_content: &str = "blob \0fileA.txt\00123456789abcdef0123456789abcdef01234567";
    let tree_content: String = format!("{}\n", blob_entry_content);

    let expected_tree_entry: TreeEntry = TreeEntry {
        name: String::from("fileA.txt"),
        object: GitObject::Blob,
        hashed_path: String::from("0123456789abcdef0123456789abcdef01234567"),
    };
    let expected_tree: Tree = Tree {
        entries: vec![expected_tree_entry.clone()],
        content: tree_content.clone(),
        content_size: tree_content.len() as i32,
    };
    assert_eq!(Tree::get_tree_from_content(&tree_content), expected_tree);
}

#[test]
fn test_get_tree_from_content_with_tree_entry() {
    let tree_entry_content: &str = "tree \0folderA\0ab89abcdef0123456789abcdef0123456789ab";
    let tree_content: String = format!("{}\n", tree_entry_content);
    let expected_tree_entry: TreeEntry = TreeEntry {
        name: String::from("folderA"),
        object: GitObject::Tree,
        hashed_path: String::from("ab89abcdef0123456789abcdef0123456789ab"),
    };
    let expected_tree: Tree = Tree {
        entries: vec![expected_tree_entry.clone()],
        content: tree_content.clone(),
        content_size: tree_content.len() as i32,
    };
    assert_eq!(Tree::get_tree_from_content(&tree_content), expected_tree);
}

#[test]
#[should_panic]
fn test_get_tree_from_content_invalid_content() {
    let invalid_content: &str = "invalid_content";
    println!(
        "{:?}",
        Tree::get_tree_from_content(&invalid_content.to_string())
    );
}

#[test]
fn test_check_valid_tree_entry_with_valid_blob() {
    let valid_blob_entry: Vec<&str> = vec![
        "blob ",
        "fileA.txt",
        "0123456789abcdef0123456789abcdef01234567",
    ];
    assert_eq!(TreeEntry::check_valid_tree_entry(&valid_blob_entry), true);
}

// #[test]
// fn test_check_valid_tree_entry_invalid_content() {
//     let valid_blob_entry: Vec<&str> = vec!["invalid content"];
//     assert_eq!(TreeEntry::check_valid_tree_entry(&valid_blob_entry), false);
// }

#[test]
fn test_check_valid_tree_entry_with_valid_tree() {
    let valid_tree_entry: Vec<&str> =
        vec!["tree ", "folderA", "ab89abcdef0123456789abcdef0123456789ab"];
    assert_eq!(TreeEntry::check_valid_tree_entry(&valid_tree_entry), true);
}

#[test]
fn test_check_valid_tree_entry_with_invalid_entry() {
    let invalid_entry: Vec<&str> = vec!["invalid_entry"];
    assert_eq!(TreeEntry::check_valid_tree_entry(&invalid_entry), false);
}

#[test]
fn test_parse_tree_entry_contents_with_blob() {
    let blob_entry_content: Vec<&str> = vec![
        "blob ",
        "fileA.txt",
        "0123456789abcdef0123456789abcdef01234567",
    ];
    let expected_blob_entry: TreeEntry = TreeEntry {
        name: String::from("fileA.txt"),
        object: GitObject::Blob,
        hashed_path: String::from("0123456789abcdef0123456789abcdef01234567"),
    };

    assert_eq!(
        TreeEntry::parse_tree_entry_contents(&blob_entry_content),
        expected_blob_entry
    );
}

#[test]
fn test_parse_tree_entry_contents_with_tree() {
    let tree_entry_content: Vec<&str> =
        vec!["tree ", "folderA", "ab89abcdef0123456789abcdef0123456789ab"];
    let expected_tree_entry: TreeEntry = TreeEntry {
        name: String::from("folderA"),
        object: GitObject::Tree,
        hashed_path: String::from("ab89abcdef0123456789abcdef0123456789ab"),
    };
    assert_eq!(
        TreeEntry::parse_tree_entry_contents(&tree_entry_content),
        expected_tree_entry
    );
}

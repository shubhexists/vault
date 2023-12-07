//VAULT COMMIT
// Loop around the directory and make the commit
// No add . apparently : )
use crate::core::tree::TreeEntry;
use crate::core::types::GitObject;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn read_directory_recursive(dir_path: &Path) -> Vec<TreeEntry> {
    let mut entries: Vec<TreeEntry> = Vec::new();
    if let Ok(entries_result) = fs::read_dir(dir_path) {
        for entry_result in entries_result {
            if let Ok(entry) = entry_result {
                if entry
                    .file_type()
                    .map_or(false, |ft: fs::FileType| ft.is_dir())
                {
                    let subdirectory_entries: Vec<TreeEntry> =
                        read_directory_recursive(&entry.path());
                    // let sub_dir_tree: crate::core::tree::Tree = make_tree(subdirectory_entries);
                    //Push entry of type GitObject::Tree
                    entries.push(
                        //     TreeEntry{
                        //     name: "".to_string(),
                        //     object: GitObject::Tree(sub_dir_tree)
                        // }
                        todo!(),
                    );
                } else {
                    // Read file content and create a blob object
                    if let Ok(content) = fs::read(entry.path()) {
                        //Make a blob
                        // let blob = make_blob(&content);
                        //Push entry of type GitObject::Blob
                        entries.push(todo!());
                    }
                }
            }
        }
    }

    entries
}

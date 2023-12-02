//VAULT COMMIT
// Loop around the directory and make the commit
// No add . apparently : )
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use crate::core::types::GitObject;

fn read_directory_recursive(dir_path: &Path) -> Vec<HashMap<String, GitObject>> {
    let mut entries:Vec<HashMap<String, GitObject>> = Vec::new();
    if let Ok(entries_result) = fs::read_dir(dir_path) {
        for entry_result in entries_result {
            if let Ok(entry) = entry_result {
                if entry.file_type().map_or(false, |ft: fs::FileType| ft.is_dir()) {
                    let subdirectory_entries: Vec<HashMap<String, GitObject>> = read_directory_recursive(&entry.path());
                    //Push entry of type GitObject::Tree
                    entries.push(todo!());
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
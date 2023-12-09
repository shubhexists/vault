use crate::core::blob::Blob;
//VAULT COMMIT
// Loop around the directory and make the commit
// No add . apparently : )
use crate::core::tree::TreeEntry;
use crate::core::types::GitObject;
use crate::hash::hash_in_sha256;
use crate::utils::read_files::read_string;
use flate2::Compression;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

pub fn commit(dir_path: &Path) -> io::Result<()> {
    let mut entries: Vec<TreeEntry> = Vec::new();
    if let Ok(entries_result) = fs::read_dir(dir_path) {
        for entry_result in entries_result {
            if let Ok(entry) = entry_result {
                if entry
                    .file_type()
                    .map_or(false, |ft: fs::FileType| ft.is_dir())
                {
                    let subdirectory_entries: Result<(), io::Error> = commit(&entry.path());
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
                        //Using Read string as I don't down if I can read binary from a text file
                        let file_contents: String = read_string(entry.path()).unwrap();
                        let file_blob: Result<Blob, std::io::Error> = Blob::new_blob(file_contents);
                        match file_blob {
                            Ok(file_blob) => {
                                let string_to_be_hashed: &String =
                                    &file_blob.clone().get_content_of_blob();
                                let hashed_blob_string: String =
                                    hash_in_sha256(&string_to_be_hashed);
                                entries.push(TreeEntry {
                                    // Enter file Name here
                                    name: "".to_string(),
                                    object: GitObject::Blob(file_blob),
                                    hashedPath: hashed_blob_string,
                                });
                            }
                            //Think what could be the error actually shown to the user?
                            Err(e) => panic!("Some Error Occurred"),
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

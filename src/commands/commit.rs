use crate::compress_zlib::compress_zlib;
use crate::core::blob::Blob;
//VAULT COMMIT
// Loop around the directory and make the commit
// No add . apparently : )
use crate::core::tree::TreeEntry;
use crate::core::types::GitObject;
use crate::hash::hash_in_sha256;
use crate::utils::get_current_branch::get_current_branch;
use crate::utils::read_files::read_string;
use flate2::Compression;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

pub fn commit(dir_path: &Path) -> io::Result<()> {
    let mut entries: Vec<TreeEntry> = Vec::new();
    let current_branch: Result<String, io::Error> = get_current_branch();
    match current_branch {
        Ok(current_branch) => {
            let path: &Path = Path::new(".vault");
            let branch_path: PathBuf = path.join(current_branch);
            let vault_path: PathBuf = branch_path.join("objects");
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
                                let file_blob: Result<Blob, std::io::Error> =
                                    Blob::new_blob(file_contents);
                                match file_blob {
                                    Ok(file_blob) => {
                                        let string_to_be_hashed: &String =
                                            &file_blob.clone().get_content_of_blob();
                                        let serialized_content: String = format!(
                                            "blob {}\0{}",
                                            string_to_be_hashed.len(),
                                            string_to_be_hashed
                                        );
                                        let compressed_content: Vec<u8> =
                                            compress_zlib(&serialized_content)?;
                                        let hashed_blob_string: String =
                                            hash_in_sha256(&string_to_be_hashed);
                                        // Make a directory with the 1st two letters of the hash
                                        let dir_name: &str = &hashed_blob_string[0..2];
                                        let dir_path: std::path::PathBuf =
                                            vault_path.join(dir_name);
                                        let file_name: &str = &hashed_blob_string[2..];
                                        let file_path: std::path::PathBuf =
                                            dir_path.join(file_name);
                                        fs::create_dir(dir_path).expect("Some error occurred");
                                        File::create(file_path).expect("Some Error occurred");
                                        entries.push(TreeEntry {
                                            // Enter file Name here
                                            name: "".to_string(),
                                            object: GitObject::Blob(file_blob),
                                            hashedPath: hashed_blob_string,
                                        });
                                    }
                                    //Think what could be the error actually shown to the user?
                                    Err(_e) => panic!("Some Error Occurred"),
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => panic!("Failed to read YAML"),
    }

    Ok(())
}

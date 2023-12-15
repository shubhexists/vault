use crate::compress_zlib::compress_zlib;
use crate::core::blob::Blob;
//VAULT COMMIT
// Loop around the directory and make the commit
// No add . apparently : )
use crate::core::tree::{Tree, TreeEntry};
use crate::core::types::GitObject;
use crate::hash::hash_in_sha256;
use crate::utils::get_current_branch::get_current_branch;
use crate::utils::read_files::read_string;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn handle_commit(dir_path: &Path) -> io::Result<Vec<TreeEntry>> {
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
                            let subdirectory_entries: Result<Vec<TreeEntry>, io::Error> =
                                handle_commit(&entry.path());
                            match subdirectory_entries {
                                Ok(subdirectory_entries) => {
                                    let sub_dir_tree: Result<Tree, io::Error> =
                                        Tree::make_tree(subdirectory_entries);
                                    match sub_dir_tree {
                                        Ok(sub_dir_tree) => {
                                            let string_to_be_hashed: &String =
                                                &sub_dir_tree.content;
                                            let compressed_content: Result<Vec<u8>, io::Error> =
                                                compress_zlib(&string_to_be_hashed);
                                            match compressed_content {
                                                Ok(compressed_content) => {
                                                    let hashed_tree_string: String =
                                                        hash_in_sha256(&string_to_be_hashed);
                                                    // Make a directory with the 1st two letters of the hash
                                                    let dir_name: &str = &hashed_tree_string[0..2];
                                                    let dir_path: std::path::PathBuf =
                                                        vault_path.join(dir_name);
                                                    let file_name: &str = &hashed_tree_string[2..];
                                                    let file_path: std::path::PathBuf =
                                                        dir_path.join(file_name);
                                                    // BAD LOGIC HERE !
                                                    if let Err(_) = fs::metadata(dir_path.clone()) {
                                                        let _is_already_created =
                                                            fs::create_dir(dir_path)
                                                                .expect("Some error occurred");
                                                    }
                                                    let mut file: File = File::create(file_path)?;
                                                    let _ = file.write_all(&compressed_content);
                                                    entries.push(TreeEntry {
                                                        name: "".to_string(),
                                                        object: GitObject::Tree(sub_dir_tree),
                                                        hashed_path: hashed_tree_string,
                                                    });
                                                }
                                                Err(e) => {
                                                    panic!("Error in compressing the file: {e}")
                                                }
                                            }
                                        }
                                        Err(e) => panic!("Some error occurred : {e}"),
                                    }
                                }
                                Err(e) => panic!("Some Error Occurred! {e}"),
                            }
                        } else {
                            // Read file content and create a blob object
                            if let Ok(_content) = fs::read(entry.path()) {
                                //Using Read string as I don't down if I can read binary from a text file
                                let file_contents: String = read_string(entry.path()).unwrap();
                                let file_blob: Result<Blob, std::io::Error> =
                                    Blob::new_blob(file_contents);
                                match file_blob {
                                    Ok(file_blob) => {
                                        let string_to_be_hashed: &String =
                                            &file_blob.clone().get_content_of_blob();
                                        let compressed_content: Vec<u8> =
                                            compress_zlib(&string_to_be_hashed)?;
                                        let hashed_blob_string: String =
                                            hash_in_sha256(&string_to_be_hashed);
                                        // Make a directory with the 1st two letters of the hash
                                        let dir_name: &str = &hashed_blob_string[0..2];
                                        let dir_path: std::path::PathBuf =
                                            vault_path.join(dir_name);
                                        let file_name: &str = &hashed_blob_string[2..];
                                        let file_path: std::path::PathBuf =
                                            dir_path.join(file_name);
                                            // BAD LOGIC HERE !
                                        if let Err(_) = fs::metadata(dir_path.clone()) {
                                            let _is_already_created = fs::create_dir(dir_path)
                                                .expect("Some error occurred");
                                        }
                                        let mut file: File = File::create(file_path)?;
                                        let _ = file.write_all(&compressed_content);
                                        entries.push(TreeEntry {
                                            // Enter file Name here
                                            name: "".to_string(),
                                            object: GitObject::Blob(file_blob),
                                            hashed_path: hashed_blob_string,
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
        Err(e) => panic!("Failed to read YAML: {e}"),
    }

    Ok(entries)
}

pub fn commit(dir_path: &Path) -> io::Result<()> {
    let commit = handle_commit(dir_path);
    match commit {
        Ok(_) => Ok(()),
        Err(e) => panic!("Failed to Commit: {e}"),
    }
}

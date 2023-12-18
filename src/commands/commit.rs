//VAULT COMMIT
// Loop around the directory and make the commit
// No add . apparently : )
use crate::core::blob::Blob;
use crate::core::commit::Commit;
use crate::core::tree::{Tree, TreeEntry};
use crate::core::types::GitObject;
use crate::utils::compress_zlib::compress_zlib;
use crate::utils::get_current_branch::get_current_branch;
use crate::utils::hash::hash_in_sha256;
use crate::utils::read_files::read_bytes;
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
                        let entry_name: std::ffi::OsString = entry.file_name();
                        if entry_name != ".vault" {
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
                                                        let dir_name: &str =
                                                            &hashed_tree_string[0..2];
                                                        let dir_path: std::path::PathBuf =
                                                            vault_path.join(dir_name);
                                                        let file_name: &str =
                                                            &hashed_tree_string[2..];
                                                        let file_path: std::path::PathBuf =
                                                            dir_path.join(file_name);
                                                        // BAD LOGIC HERE !
                                                        if let Err(_) =
                                                            fs::metadata(dir_path.clone())
                                                        {
                                                            let _is_already_created =
                                                                fs::create_dir(dir_path)
                                                                    .expect("Some error occurred");
                                                        }
                                                        let mut file: File =
                                                            File::create(file_path)?;
                                                        let _ = file.write_all(&compressed_content);
                                                        entries.push(TreeEntry {
                                                            name: entry_name
                                                                .to_str()
                                                                .unwrap()
                                                                .to_string(),
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
                                let file_dir_path: PathBuf = entry.path();
                                let file_extension: &Option<&str> = &file_dir_path
                                    .extension()
                                    .and_then(|ext: &std::ffi::OsStr| ext.to_str());
                                let file_contents: Vec<u8> =
                                    read_bytes(file_dir_path.clone()).unwrap();
                                let file_blob: Result<Blob, io::Error> = match file_extension {
                                    Some(file_ext) => {
                                        Blob::new_blob(file_contents, Some(file_ext.to_string()))
                                    }
                                    None => Blob::new_blob(file_contents, None),
                                };
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
                                            name: entry_name.to_str().unwrap().to_string(),
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

//@TODO - Sync this function to config.yaml
pub fn commit(dir_path: &Path, message: &str) -> io::Result<()> {
    let commit: Result<Vec<TreeEntry>, io::Error> = handle_commit(dir_path);
    let vault = Path::new(".vault");
    let current_branch: Result<String, io::Error> = get_current_branch();
    match current_branch {
        Ok(current_branch) => {
            let vault_branch_path: PathBuf = vault.join(current_branch);
            let vault_path = vault_branch_path.join("objects");
            match commit {
                Ok(vec_tree) => {
                    let main_repo_tree: Result<Tree, io::Error> = Tree::make_tree(vec_tree);
                    match main_repo_tree {
                        Ok(dir_tree) => {
                            let string_to_be_hashed: &String = &dir_tree.content;
                            let compressed_content: Result<Vec<u8>, io::Error> =
                                compress_zlib(&string_to_be_hashed);
                            match compressed_content {
                                Ok(compressed_content) => {
                                    let hash_main_dir_in_sha256: String =
                                        hash_in_sha256(&string_to_be_hashed);
                                    let dir_name: &str = &&hash_main_dir_in_sha256[0..2];
                                    let dir_path: std::path::PathBuf = vault_path.join(dir_name);
                                    let file_name: &str = &&hash_main_dir_in_sha256[2..];
                                    let file_path: std::path::PathBuf = dir_path.join(file_name);
                                    // Not Good Logic ig?
                                    if let Err(_) = fs::metadata(dir_path.clone()) {
                                        let _is_already_created =
                                            fs::create_dir(dir_path).expect("Some error occurred");
                                    }
                                    let mut file: File = File::create(file_path)?;
                                    let _ = file.write_all(&compressed_content);
                                    let current_commit: Result<Commit, io::Error> =
                                        Commit::new_commit(message, hash_main_dir_in_sha256);
                                    match current_commit {
                                        Ok(current_commit) => {
                                            let commit_content: String =
                                                Commit::get_content_of_commit(current_commit);
                                            let compressed_commit_content: Result<
                                                Vec<u8>,
                                                io::Error,
                                            > = compress_zlib(&commit_content);
                                            match compressed_commit_content {
                                                Ok(compressed_commit_content) => {
                                                    let hash_commit_content_in_sha256: String =
                                                        hash_in_sha256(&commit_content);
                                                    let dir_name: &str =
                                                        &hash_commit_content_in_sha256[..2];
                                                    let file_name: &str =
                                                        &hash_commit_content_in_sha256[2..];
                                                    let dir_path: std::path::PathBuf =
                                                        vault_path.join(dir_name);
                                                    let file_path: PathBuf =
                                                        dir_path.join(file_name);
                                                    // Not Good Logic ig?
                                                    if let Err(_) = fs::metadata(dir_path.clone()) {
                                                        let _is_already_created =
                                                            fs::create_dir(dir_path)
                                                                .expect("Some error occurred");
                                                    }
                                                    let mut file: File = File::create(file_path)?;
                                                    let _ =
                                                        file.write_all(&compressed_commit_content);
                                                }
                                                Err(e) => {
                                                    panic!("Some Error Occurred: {e}");
                                                }
                                            }
                                        }
                                        Err(e) => panic!("Some error Occurred : {e}"),
                                    }
                                    Ok(())
                                }
                                Err(e) => panic!("Some error Occurred: {e}"),
                            }
                        }
                        Err(e) => panic!("Some Error Occurred: {e}"),
                    }
                }
                Err(e) => panic!("Failed to Commit: {e}"),
            }
        }
        Err(e) => panic!("Some error occurred : {e}"),
    }
}

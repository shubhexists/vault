use super::compress_zlib::decompress_zlib;
use crate::core::{commit::Commit, tree::Tree};
use std::env::temp_dir;
use std::io::Write;
use std::{fs::File, io, path::Path};

pub fn create_fs(commit_hash: &String, path: &String, branch: &String) -> io::Result<()> {
    let dir_name: &str = &commit_hash[..2];
    let file_name: &str = &commit_hash[2..];
    let vault_path: &Path = Path::new(".vault");
    let current_branch_path: std::path::PathBuf = vault_path.join(branch);
    let object_dir_path: std::path::PathBuf = current_branch_path.join("objects");
    let hash_dir: std::path::PathBuf = object_dir_path.join(dir_name);
    let hash_file_path: std::path::PathBuf = hash_dir.join(file_name);
    let file: File = File::open(hash_file_path)?;
    let commit_string: String = decompress_zlib(file).unwrap();
    let commit_object: Commit = Commit::get_commit_from_content(&commit_string);
    let root_dir_hash: String = commit_object.commit_hash;
    let _ = navigate_through_root_dir_hash(&root_dir_hash, branch, path);
    Ok(())
}

fn navigate_through_root_dir_hash(
    root_dir_hash: &String,
    branch: &String,
    path: &String,
) -> io::Result<()> {
    let dir_name: &str = &root_dir_hash[..2];
    let file_name: &str = &&root_dir_hash[2..];
    let vault_path: &Path = Path::new(".vault");
    let current_branch_path: std::path::PathBuf = vault_path.join(branch);
    let object_dir_path: std::path::PathBuf = current_branch_path.join("objects");
    let hash_dir: std::path::PathBuf = object_dir_path.join(dir_name);
    let hash_file_path: std::path::PathBuf = hash_dir.join(file_name);
    let file: File = File::open(hash_file_path)?;
    let commit_string: String = decompress_zlib(file).unwrap();
    let tree_object: Tree = Tree::get_tree_from_content(&commit_string);
    let temp_dir: std::path::PathBuf = temp_dir();
    
    Ok(())
}

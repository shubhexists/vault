use std::{fs::File, io, path::Path};

use crate::core::commit::Commit;

use super::compress_zlib::decompress_zlib;

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
    let commit_object: Option<Commit> = Commit::get_commit_from_content(&commit_string);
    println!("{:?}", commit_object);
    Ok(())
}

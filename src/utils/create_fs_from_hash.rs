use super::compress_zlib::decompress_zlib;
use super::read_files::copy_directory_contents;
use crate::core::blob::Blob;
use crate::core::types::GitObject;
use crate::core::{commit::Commit, tree::Tree};
use std::path::PathBuf;
use std::{env, fs};
use std::{fs::File, io, path::Path};

pub fn create_fs(commit_hash: &String, path: &String, branch: &String) -> io::Result<()> {
    let dir_name: &str = &commit_hash[..2];
    let file_name: &str = &commit_hash[2..];
    let vault_path: &Path = Path::new(".vault");
    let current_branch_path: PathBuf = vault_path.join(branch);
    let temp_dir_path: PathBuf = current_branch_path.join("tmp");
    let object_dir_path: PathBuf = current_branch_path.join("objects");
    let hash_dir: PathBuf = object_dir_path.join(dir_name);
    let hash_file_path: PathBuf = hash_dir.join(file_name);
    let file: File = File::open(hash_file_path)?;
    let commit_string: String = decompress_zlib(file).unwrap();
    let commit_object: Commit = Commit::get_commit_from_content(&commit_string);
    let root_dir_hash: String = commit_object.commit_hash;
    let _ = fs::create_dir(&temp_dir_path);
    let _ = navigate_through_dir_hash(&root_dir_hash, &current_branch_path, &temp_dir_path);
    if path == "./" || path == "." {
        // To figure out, which are the files that are to be protected
    } else {
        let current_dir: PathBuf = env::current_dir().unwrap();
        let new_dir: PathBuf = current_dir.join(path);
        let _ = copy_directory_contents(&temp_dir_path, &new_dir);
        let _ = fs::remove_dir_all(temp_dir_path);
    }
    Ok(())
}

//@TODO- THis function is not error handled at all
fn navigate_through_dir_hash(
    dir_hash: &String,
    path: &PathBuf,
    temp_dir_path: &PathBuf,
) -> io::Result<()> {
    let dir_name: &str = &dir_hash[..2];
    let file_name: &str = &dir_hash[2..];
    let object_dir_path: PathBuf = path.join("objects");
    let hash_dir: PathBuf = object_dir_path.join(dir_name);
    let hash_file_path: PathBuf = hash_dir.join(file_name);
    let file: File = File::open(hash_file_path)?;
    let commit_string: String = decompress_zlib(file).unwrap();
    let tree_object: Tree = Tree::get_tree_from_content(&commit_string);
    for entry in tree_object.entries {
        match entry.object {
            GitObject::Blob => {
                let blob_hash: String = entry.hashed_path;
                let blob_content_dir: &str = &blob_hash[..2];
                let blob_content_file: &str = &blob_hash[2..];
                let blob_hash_dir: &PathBuf = &object_dir_path.join(&blob_content_dir);
                let blob_hash_file_path: &PathBuf = &blob_hash_dir.join(&blob_content_file);
                let file: File = File::open(blob_hash_file_path)?;
                let blob_string: String = decompress_zlib(file).unwrap();
                let blob_object: Blob = Blob::get_blob_from_content(&blob_string);
                let final_blob_content: &String = &blob_object.content;
                let file_path: PathBuf = temp_dir_path.join(entry.name);
                let _ = fs::write(file_path, final_blob_content);
            }
            GitObject::Tree => {
                let dir_name: String = entry.name;
                let dir_path: PathBuf = temp_dir_path.join(dir_name);
                if let Err(_) = fs::metadata(&dir_path) {
                    let _is_already_created =
                        fs::create_dir(&dir_path).expect("Some error occurred");
                }
                let _ = navigate_through_dir_hash(&entry.hashed_path, path, &dir_path);
            }
        }
    }
    Ok(())
}

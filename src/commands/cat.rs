use crate::{utils::compress_zlib::decompress_zlib, utils::get_current_branch::get_current_branch};
use std::{fs::File, io, path::Path};

pub fn cat(hash_string: &str) -> io::Result<()> {
    let path: &Path = Path::new(".vault");
    let dir_name: &str = &hash_string[..2];
    let file_path: &str = &hash_string[2..];
    // Handle Error here
    let current_branch_name: String = get_current_branch().unwrap();
    let branch_path: std::path::PathBuf = path.join(current_branch_name);
    let branch_object_path: std::path::PathBuf = branch_path.join("objects");
    let hash_dir_path: std::path::PathBuf = branch_object_path.join(dir_name);
    let file_path: std::path::PathBuf = hash_dir_path.join(file_path);
    // Since this would read from the current directory only,  If the file is not found, prompt the user to check if his directory is correct
    let file: File = File::open(file_path)?;
    let decompressed = decompress_zlib(file);
    match decompressed {
        Ok(decompressed) => {
            println!("{decompressed}")
        }
        Err(e) => panic!("Unable to read: {e}"),
    }
    Ok(())
}

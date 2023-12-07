//This will have a utility function to split the SHA-1 where the 1st two letters will represent the directory and the rest wouyld be the file name
//THis might BE WRONG
use std::io;
use std::fs;
use std::fs::File;
use std::path::Path;

pub fn save_with_hash(hashed_file_name: &str) -> io::Result<()> {
    let vault_path: &Path = Path::new(".vault/objects");
    let dir_name: &str = &hashed_file_name[0..2];
    let file_name: &str = &hashed_file_name[2..];
    //Now make a directory with dir name and a file with file name
    //TO think if this error message is correct
    let dir_path: std::path::PathBuf = vault_path.join(dir_name);
    fs::create_dir(dir_path.clone()).expect("Some error occurred");
    let file_path = dir_path.join(file_name);
    File::create(file_path).expect("Some Error occurred");
    Ok(())
}
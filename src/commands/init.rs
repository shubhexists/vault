//This will have an init function which will currently create a .vault folder in the current directory
// VAULT INIT
use std::fs::File;
use std::path::Path;
use std::{fs, io};

//@TODO - make a utils module with reusable function like create file and create dir...

pub fn init() -> io::Result<()> {
    let current_dir: std::path::PathBuf =
        std::env::current_dir().expect("Unable to get current directory");
    let current_dir_str: String = current_dir.to_string_lossy().into_owned();
    let path: &Path = Path::new(".vault");
    let file_path: std::path::PathBuf = path.join("CurrentDir");
    let objects: std::path::PathBuf = path.join("objects");
    let logs: std::path::PathBuf = path.join("logs");
    let log_head: std::path::PathBuf = logs.join("HEAD");
    let ignore_file_path: &Path = Path::new(".vaultignore");
    if path.exists() {
        panic!("This directory already is a in a vault. Cannot init!");
    } else {
        //@TODO - Think of better error messages
        //@TODO - If any step fails, reverse all the previous steps...,
        // We can do this by making a temp directory and only copy the temp dir if all the steps pass
        fs::create_dir(path).expect("Unable to create .vault folder");
        fs::create_dir(objects).expect("Unable to create a vault! \n Some error occured");
        fs::create_dir(logs).expect("Unable to create a vault! \n Some error occured");
        fs::File::create(file_path.clone())
            .expect("Unable to create a vault! \n Some error occured");
        fs::File::create(log_head).expect("Unable to create a vault! \n Some error occured");
        let _ = fs::write(file_path, current_dir_str);
        let _ = File::create(ignore_file_path);
        println!("Vault created successfully");
        Ok(())
    }
}

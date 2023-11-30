//This will have an init function which will currently create a .vault folder in the current directory

// use crate::branches::create;
use std::fs;
use std::fs::File;
use std::path::Path;

pub fn init() {
    let path: &Path = Path::new(".vault");
    let file_path: std::path::PathBuf = path.join("CurrentDir");
    let objects: std::path::PathBuf = path.join("objects");
    let ignore_file_path: &Path = Path::new(".vaultignore");
    if path.exists() {
        println!("This directory already is a in a vault. Cannot init!");
    } else {
        //@TODO - Think of better error messages
        //@TODO - If any step fails, reverse all the previous steps...,
        // We can do this by making a temp directory and only copy the temp dir if all the steps pass
        fs::create_dir(path).expect("Unable to create .vault folder");
        fs::create_dir(objects).expect("Unable to create a vault! \n Some error occured");
        fs::File::create(file_path).expect("Unable to create a vault! \n Some error occured");
        let _ = File::create(ignore_file_path);
        println!("Vault created successfully");
    }
}

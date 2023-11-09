//This will have an init function which will currently create a .vault folder in the current directory

use crate::branches::create;
use std::fs;
use std::fs::File;
use std::path::Path;

pub fn init() {
    let path = Path::new(".vault");
    let file_path = path.join("CurrentDir");
    let ignore_file_path = Path::new(".vaultignore");
    if path.exists() {
        println!("This directory already is a in a vault. Cannot init! ");
    } else {
        fs::create_dir(".vault").expect("Unable to create .vault folder");
        create("main");
        let _ = File::create(ignore_file_path);
        println!("Vault created successfully");
        //To see how to handle errors here
        let _ = fs::write(file_path, "main");
    }
}

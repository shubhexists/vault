//This will have an init function which will currently create a .vault folder in the current directory

use std::fs;
use std::path::Path;

pub fn init() {
    let path = Path::new(".vault");
    if path.exists() {
        println!("This directory already is a in a vault. Cannot init! ");
    } else {
        fs::create_dir(".vault").expect("Unable to create .vault folder");
        println!("Vault created successfully");
    }
}
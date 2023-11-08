//This file will handles all the logic to create a new branch
// Path: src/branches.rs

use std::fs;
use std::path::Path;

pub fn create(branch_name: &str) {
    let path = Path::new(".vault");
    if path.exists() {
        let branch_path = path.join(branch_name);
        if branch_path.exists() {
            println!("Branch already exists");
        } else {
            fs::create_dir(branch_path).expect("Unable to create branch");
            println!("Branch created successfully");
        }
    } else {
        println!("This directory is not a vault. Cannot create branch");
    }
}

pub fn delete(branch_name: &str) {
    let path = Path::new(".vault");
    if path.exists(){
        let branch_path = path.join(branch_name);
        if branch_path.exists(){
            //Debug Print ??
            println!("Deleting Branch {}",branch_name);
        } else {
            println!("ERROR: Branch does not exists! ");
        }
    } else {
        println!("This directory is not a vault. Cannot delete !")
    }
}
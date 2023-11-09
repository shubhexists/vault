//This file will handles all the logic to create a new branch

use std::fs;
use std::path::Path;

//Creates a new branch :)
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

// Deletes a branch !
//@TODO - A person can not delete an active branch! He has to first switch to another branch to delete any particular branch
pub fn delete(branch_name: &str) {
    let path = Path::new(".vault");
    if path.exists() {
        let branch_path = path.join(branch_name);
        if branch_path.exists() {
            println!("Deleting Branch {}", branch_name);
            //How to handle errors here?
            let _ = fs::remove_dir_all(branch_path);
        } else {
            println!("ERROR: Branch does not exists! ");
        }
    } else {
        println!("This directory is not a vault. Cannot delete !")
    }
}

//Switches between branches !
pub fn switch(branch_name: &str) {
    let path = Path::new(".vault");
    let file_path = path.join("CurrentDir");
    let branch_path = path.join(branch_name);
    if path.exists() {
        if branch_path.exists() {
            let _ = fs::write(file_path, branch_name);
        } else {
            println!("Branch {} doesn't exists!", branch_name);
        }
    } else {
        println!("First create a vault to create branches!")
    }
}

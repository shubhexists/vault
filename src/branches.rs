//This file will handles all the logic to create a new branch


use std::fs;
use std::io::{self};
use std::path::Path;
use crate::utils::utils:: read_vault_ignore;
use crate::utils::sync_dir::sync_current_dir;


//Creates a new branch :)
//@TODO - Add all current components to the new branch
pub fn create(branch_name: &str) {
    let path: &Path = Path::new(".vault");
    if path.exists() {
        let branch_path: std::path::PathBuf = path.join(branch_name);
        if branch_path.exists() {
            println!("Branch already exists");
        } else {
            fs::create_dir(branch_path).expect("Unable to create branch");
            //For the first time in main branch, user has to do vault add
            //@TODO - Promt the user in vault add to remove binaries and build type of files to be memory efficient?

            let current_dir: std::path::PathBuf =
                std::env::current_dir().expect("Unable to get current directory");
            let _ = sync_current_dir(&current_dir, &branch_name);

            println!("Branch created successfully");
        }
    } else {
        println!("This directory is not a vault. Cannot create branch");
    }
}

// Deletes a branch !
//@TODO - A person can not delete an active branch! He has to first switch to another branch to delete any particular branch
pub fn delete(branch_name: &str) {
    let path: &Path = Path::new(".vault");
    if path.exists() {
        let branch_path: std::path::PathBuf = path.join(branch_name);
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
pub fn switch(branch_name: &str) -> io::Result<()> {
    // Make a nested array of hashmap of all the files in .vault/branch_name
    let path: &Path = Path::new(".vault");
    let file_path: std::path::PathBuf = path.join("CurrentDir");
    let branch_path: std::path::PathBuf = path.join(branch_name);
    let ignored_files: Vec<String> = read_vault_ignore();
    if path.exists() {
        if branch_path.exists() {
            let current_dir: std::path::PathBuf =
                std::env::current_dir().expect("Unable to get current directory");
            let entries: fs::ReadDir = fs::read_dir(&current_dir)?;
            for entry in entries {
                let entry: fs::DirEntry = entry?;
                let path: std::path::PathBuf = entry.path();
                //According to ChatGPT, unwrap_or_default will ensure that an empty string is returned if filename extraction failed.
                //to_string_lossy will handles characters that are not UTF8 encoded in the file name.
                let file_name: std::borrow::Cow<'_, str> =
                    path.file_name().unwrap_or_default().to_string_lossy();
                // println!("{}", file_name);
                if path.is_dir() {
                    if !file_name.starts_with('.') {
                        if ignored_files.contains(&file_name.to_string()) {
                        } else {
                        }
                    }
                } else if path.is_file() {
                    if !file_name.starts_with('.') {
                        if ignored_files.contains(&file_name.to_string()) {
                        } else {
                            let branch_file_path: std::path::PathBuf =
                                branch_path.join(&file_name.to_string());
                        }
                    }
                }
            }
            let _ = fs::write(file_path, branch_name);
            println!("Switched to branch: {}", branch_name);
        } else {
            println!("Branch {} doesn't exists!", branch_name);
        }
    } else {
        println!("First create a vault to create branches!")
    }
    Ok(())
}

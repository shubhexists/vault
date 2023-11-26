use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Read;
use std::io::{self};
use std::path::Path;

pub fn sync_current_dir(current_dir: &Path, current_branch: &str) -> io::Result<()> {
    let ignored_files: Vec<String> = read_vault_ignore();
    let vault_path: &Path = Path::new(".vault");
    let entries: fs::ReadDir = fs::read_dir(&current_dir)?;
    for entry in entries {
        let entry: fs::DirEntry = entry?;
        let path: std::path::PathBuf = entry.path();
        //According to ChatGPT, unwrap_or_default will ensure that an empty string is returned if filename extraction failed.
        //to_string_lossy will handles characters that are not UTF8 encoded in the file name.
        let file_name: std::borrow::Cow<'_, str> =
            path.file_name().unwrap_or_default().to_string_lossy();
        if path.is_dir() {
            //Ignore hidden directory
            if !file_name.starts_with('.') {
                if ignored_files.contains(&file_name.to_string()) {
                } else {
                    let branch_path: std::path::PathBuf = vault_path.join(&current_branch);
                    //If it's a directory, create a directory with the same name in current branch?
                    // println!("Directory: {}", file_name);
                    let branch_dir_path: std::path::PathBuf =
                        branch_path.join(&file_name.to_string());
                    // println!("{}",branch_dir_path.display());
                    fs::create_dir(branch_dir_path).expect("Unable to create branch");
                    let sub_dir_path: std::path::PathBuf = current_dir.join(&path);
                    let temp_branch_dir: String =
                        current_branch.to_owned() + "/" + &file_name.to_string();
                    let _ = sync_current_dir(&sub_dir_path, &temp_branch_dir);
                }
            }
        } else if path.is_file() {
            //Ignore hidden files
            if !file_name.starts_with('.') {
                if ignored_files.contains(&file_name.to_string()) {
                } else {
                    let branch_path: std::path::PathBuf = vault_path.join(&current_branch);
                    let file_destination_path: std::path::PathBuf =
                        branch_path.join(&file_name.to_string());
                    let mut file: File = File::open(path).expect("Unable to Open");
                    let mut contents: String = String::new();
                    let _ = file.read_to_string(&mut contents);
                    let _ = File::create(&file_destination_path);
                    let _ = fs::write(file_destination_path, contents);
                }
            }
        }
    }
    Ok(())
}

pub fn get_current_branch() -> String {
    let file_path: &str = ".vault/CurrentDir";
let mut file: File = File::open(file_path).expect("Unable to Open");
let mut contents: String = String::new();
let _ = file.read_to_string(&mut contents);
    println!("File contents: {}", contents);
    contents
}

pub fn read_vault_ignore() -> Vec<String> {
    let filename: &str = ".vaultignore";
    let path: &Path = Path::new(filename);
    let mut result: Vec<String> = Vec::new();
    if path.exists() {
        for line in read_to_string(filename).unwrap().lines() {
            result.push(line.to_string())
        }
    }
    result
}

pub fn sync_files_from_branch(){
    let current_dir: std::path::PathBuf = std::env::current_dir().expect("Unable to get current directory!");
    let ignored_files: Vec<String> = read_vault_ignore();
    let entries: Result<fs::ReadDir, io::Error> = fs::read_dir(&current_dir);
}
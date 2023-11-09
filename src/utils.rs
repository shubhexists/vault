use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

//Change the name of the function as it won't just read the contents
pub fn read_current_dir(current_dir: &Path) -> io::Result<()> {
    let ignored_files: Vec<String> = read_vault_ignore();
    let entries = fs::read_dir(&current_dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        //According to ChatGPT, unwrap_or_default will ensure that an empty string is returned if filename extraction failed.
        //to_string_lossy will handles characters that are not UTF8 encoded in the file name.
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
        if path.is_dir() {
            //Ignore hidden directory
            if !file_name.starts_with('.') {
                if ignored_files.contains(&file_name.to_string()) {
                } else {
                    //If it's a directory, create a directory with the same name in current branch?
                    println!("Directory: {}", file_name);
                    let sub_dir_path = current_dir.join(path);
                    let _ = read_current_dir(&sub_dir_path);
                }
            }
        } else if path.is_file() {
            //Ignore hidden files
            if !file_name.starts_with('.') {
                if ignored_files.contains(&file_name.to_string()) {
                } else {
                    println!("File: {} in directory {}", file_name, current_dir.display());
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

fn read_vault_ignore() -> Vec<String> {
    let filename: &str = ".vaultignore";
    let path = Path::new(filename);
    let mut result: Vec<String> = Vec::new();
    if path.exists() {
        for line in read_to_string(filename).unwrap().lines() {
            result.push(line.to_string())
        }
    }
    result
}

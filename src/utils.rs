use std::fs;
use std::fs::File;
use std::io::{self, Read};

//Change the name of the function as it won't just read the contents
pub fn read_current_dir() -> io::Result<()> {
    // let current_branch = get_current_branch();
    // println!("{}",current_branch);
    let current_dir = std::env::current_dir()?;
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
                //If it's a directory, create a directory with the same name in current branch?
                println!("Directory: {}", file_name);
            }
        } else if path.is_file() {
            //Ignore hidden files
            if !file_name.starts_with('.') {
                println!("File: {}", file_name);
            }
        }
    }
    Ok(())
}


//I dont know exactly what to do but it returns Ok("main") instead of a string :/
fn get_current_branch() -> io::Result<String> {
    let file_path = ".vault/CurrentDir";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File contents:\n{}", contents);
    Ok(contents)
}
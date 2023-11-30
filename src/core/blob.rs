use crate::utils::read_files::{read_bytes, read_string};
use std::io;
use std::path::Path;

/* SAMPLE BLOB

blob 23\0This is the content of the file.
 */

#[derive(Debug)]
pub struct Blob {
    content_size: i32,
    content: String,
}

impl Blob {
    pub fn new_blob(file_path: &str) -> io::Result<Blob> {
        let path: &Path = Path::new(file_path);
        if path.is_file() {
            //Determine binary or not :/ (we can add certain extension for which we are sure that the file is definitely text)
            let contents: Result<String, io::Error> = read_string(file_path);
            match contents {
                Ok(string_contents) => {
                    let content_size: i32 = string_contents.as_bytes().len().try_into().unwrap();
                    Ok(Blob {
                        content_size,
                        content: string_contents,
                    })
                }
                Err(err) => {
                    panic!("Error reading file: {}", err);
                }
            }
        } else {
            //This error should not be shown in release (Make it debug only maybe?)
            //Also this should not be panic here 
            panic!("Blob can only have files")
        }
    }
}

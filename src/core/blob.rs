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
    pub fn new_blob(content: String) -> io::Result<Blob> {
        let size: i32 = String::from(content.clone()).chars().count() as i32;
        Ok(Blob {
            content_size: size,
            content,
        })
    }
    pub fn get_content_of_blob(blob: Blob) -> String {
        let mut file_content: String = String::new();
        file_content.push_str("blob ");
        file_content.push_str(&blob.content_size.to_string());
        file_content.push('\0');
        file_content.push_str(&blob.content);
        file_content
    }
}

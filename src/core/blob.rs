use std::io;

use super::types::{FileType, FileTypes};

/* SAMPLE BLOB - for UTF-8 files...

blob 23\0This is the content of the file.
 */

#[derive(Debug, Clone)]
pub struct Blob {
    isUtf8: FileTypes,
    pub content_size: i32,
    content: String,
}

impl Blob {
    pub fn new_blob(content: Vec<u8>, filetype: Option<String>) -> io::Result<Blob> {
        let str_content: Result<String, std::string::FromUtf8Error> = String::from_utf8(content);
        match str_content {
            Ok(content) => {
                let size: i32 = String::from(content.clone()).chars().count() as i32;
                Ok(Blob {
                    isUtf8: FileTypes::Utf8(FileType { ftype: filetype }),
                    content_size: size,
                    content,
                })
            }
            Err(_) => {
                let size: i32 = content.len() as i32;
                Ok(Blob {
                    isUtf8: FileTypes::NonUTF8(FileType { ftype: filetype }),
                    content_size: size,
                    content: todo!()
                })
            }
        }
    }

    pub fn get_content_of_blob(self) -> String {
        let mut file_content: String = String::new();
        file_content.push_str("blob ");
        file_content.push_str(&self.content_size.to_string());
        file_content.push('\0');
        file_content.push_str(&self.content);
        file_content
    }
}

use super::types::FileTypes;
use std::io;

/* SAMPLE BLOB - for UTF-8 files...

blob \0UTF\023\0This is the content of the file.
 */

/* SAMPLE BLOB - for NonUTF-8 files.... (Did not know the advantage of using hex (if any?))

blob \0NonUTF\036\0123,345,4,6,7,7,43,142,32,44
 */

#[derive(Debug, Clone)]
pub struct Blob {
    is_utf8: FileTypes,
    pub content_size: i32,
    content: String,
}

impl Blob {
    pub fn new_blob(content: Vec<u8>) -> io::Result<Blob> {
        let str_content: Result<String, std::string::FromUtf8Error> =
            String::from_utf8(content.clone());
        match str_content {
            Ok(content) => {
                let size: i32 = String::from(content.clone()).chars().count() as i32;
                Ok(Blob {
                    is_utf8: FileTypes::Utf8,
                    content_size: size,
                    content,
                })
            }
            Err(_) => {
                let size: i32 = content.len() as i32;
                let content_vec_to_str = || -> String {
                    let mut file_content: String = String::new();
                    for byte in content {
                        file_content.push_str(&byte.to_string());
                        file_content.push(',');
                    }
                    file_content
                };
                Ok(Blob {
                    is_utf8: FileTypes::NonUTF8,
                    content_size: size,
                    content: content_vec_to_str(),
                })
            }
        }
    }

    pub fn get_content_of_blob(self) -> String {
        let mut file_content: String = String::new();
        file_content.push_str("blob ");
        file_content.push('\0');
        match self.is_utf8 {
            FileTypes::Utf8 => {
                file_content.push_str("UTF");
            }
            FileTypes::NonUTF8 => {
                file_content.push_str("NonUTF");
            }
        }
        file_content.push('\0');
        file_content.push_str(&self.content_size.to_string());
        file_content.push('\0');
        file_content.push_str(&self.content);
        file_content
    }
}

use std::io;

/* SAMPLE BLOB

blob 23\0This is the content of the file.
 */

#[derive(Debug, Clone)]
pub struct Blob {
    pub content_size: i32,
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
    pub fn get_content_of_blob(self) -> String {
        let mut file_content: String = String::new();
        file_content.push_str("blob ");
        file_content.push_str(&self.content_size.to_string());
        file_content.push('\0');
        file_content.push_str(&self.content);
        file_content
    }
}

/* SAMPLE BLOB

blob 23\0This is the content of the file.
 */

#[derive(Debug)]
pub struct Blob {
    content_size: i32,
    content: String,
}

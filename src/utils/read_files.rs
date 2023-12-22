use std::fs::{self, OpenOptions};
use std::io::{Read, Result};
use std::path::Path;

pub fn read_bytes<P: AsRef<Path>>(file_path: P) -> Result<Vec<u8>> {
    let mut file: fs::File = OpenOptions::new().read(true).open(file_path)?;
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

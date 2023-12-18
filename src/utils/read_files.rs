use std::path::Path;
use std::fs::OpenOptions;
use std::io::{ Read, Result};

pub fn read_bytes<P: AsRef<Path>>(file_path: P) -> Result<Vec<u8>> {
    let mut file: std::fs::File = OpenOptions::new().read(true).open(file_path)?;
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

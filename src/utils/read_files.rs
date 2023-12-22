use std::fs::{self, OpenOptions};
use std::io::{self, Read, Result};
use std::path::{Path, PathBuf};

pub fn read_bytes<P: AsRef<Path>>(file_path: P) -> Result<Vec<u8>> {
    let mut file: fs::File = OpenOptions::new().read(true).open(file_path)?;
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

pub fn copy_directory_contents(src: &Path, dest: &Path) -> io::Result<()> {
    fs::create_dir_all(dest)?;
    let entries: fs::ReadDir = fs::read_dir(src)?;
    for entry in entries {
        let entry: fs::DirEntry = entry?;
        let entry_path: PathBuf = entry.path();
        let dest_path: PathBuf = dest.join(entry_path.file_name().unwrap());
        if entry_path.is_dir() {
            copy_directory_contents(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}

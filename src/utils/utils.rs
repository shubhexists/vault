use std::fs::read_to_string;
use std::path::Path;


pub fn read_vault_ignore() -> Vec<String> {
    let filename: &str = ".vaultignore";
    let path: &Path = Path::new(filename);
    let mut result: Vec<String> = Vec::new();
    if path.exists() {
        for line in read_to_string(filename).unwrap().lines() {
            result.push(line.to_string())
        }
    }
    result
}

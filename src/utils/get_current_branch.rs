use std::fs::File;
use std::io::{self, BufReader};

use super::yaml_layouts::InitLayout;

pub fn get_current_branch() -> io::Result<String> {
    let file: File = File::open(".vault/init.yaml").expect("Failed to open the details file");
    let reader: BufReader<File> = BufReader::new(file);
    let config: InitLayout = serde_yaml::from_reader(reader).expect("Failed to read YAML");
    let current_branch: String = config.current_branch;
    Ok(current_branch)
}

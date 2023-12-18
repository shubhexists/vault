use std::fs::File;
use std::io::{self, BufReader};

use super::yaml_layouts::InitLayout;

pub fn get_current_branch() -> io::Result<String> {
    let file: Result<File, io::Error> = File::open(".vault/init.yaml");
    match file {
        Ok(file) => {
            let reader: BufReader<File> = BufReader::new(file);
            let config: InitLayout = serde_yaml::from_reader(reader).expect("Failed to read YAML");
            let current_branch: String = config.current_branch;
            Ok(current_branch)
        }
        Err(_e) => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Couldn't detect directory as a vault. Kindly Run `vault init` to make a vault",
        )),
    }
}

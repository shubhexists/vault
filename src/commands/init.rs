//This will have an init function which will currently create a .vault folder in the current directory
// VAULT INIT
use crate::commands::{create, switch};
use crate::utils::yaml_layouts::InitLayout;
use std::fs::{self, create_dir};
use std::path::Path;

pub fn init() {
    let vault_folder: &Path = Path::new(".vault");
    let current_dir: std::path::PathBuf =
        std::env::current_dir().expect("Unable to get current directory");
    let ignore_file_path: std::path::PathBuf = current_dir.join(".vaultignore");
    match create_dir(vault_folder) {
        Ok(_) => {
            let init_file: std::path::PathBuf = vault_folder.join("init.yaml");
            let content: String = get_init_data();
            fs::File::create(ignore_file_path).unwrap();
            fs::write(init_file, content).unwrap();
            // @TODO: Add call to branch function with default(master) parameter.
            create(&"master".to_string());
            // @TODO: Add call to switch function with default(master) parameter.
            switch(&"master".to_string());
        }
        Err(_) => println!("Vault already initialized"),
    }
}

fn get_init_data() -> String {
    let yaml_struct: InitLayout = InitLayout {
        current_branch: String::new(),
        branches: Vec::new(),
    };

    let yaml_string: String = serde_yaml::to_string(&yaml_struct).unwrap();
    return yaml_string;
}

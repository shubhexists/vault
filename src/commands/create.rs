use crate::utils::{ConfigLayout, InitLayout};
use std::fs;
use std::path::{Path, PathBuf};

pub fn create(branch_name: &String) {
    let vault_folder: &Path = Path::new(".vault");
    let init_file: PathBuf = vault_folder.join("init.yaml");
    let content_bytes: Vec<u8> = fs::read(&init_file).unwrap();
    let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&content_bytes);
    let mut init_content: InitLayout = serde_yaml::from_str(&content).unwrap();

    init_content.branches.push(branch_name.clone());
    let branch_folder: PathBuf = vault_folder.join(branch_name.clone());
    let branch_object_folder: PathBuf = branch_folder.join("objects");
    // @TODO Handle Error here with match statements
    fs::create_dir(&branch_folder).unwrap();
    fs::create_dir(branch_object_folder).unwrap();
    create_config_yaml(&branch_folder);
    println!("content of init.yaml file \n{:?}", init_content);
    let yaml_string: String = serde_yaml::to_string(&init_content).unwrap();
    fs::write(init_file, yaml_string).unwrap();
}

fn create_config_yaml(folder_path: &PathBuf) {
    let config_path: PathBuf = folder_path.join("config.yaml");
    let yaml_struct: ConfigLayout = ConfigLayout::default();
    let yaml_string: String = serde_yaml::to_string(&yaml_struct).unwrap();
    fs::write(config_path, yaml_string).unwrap();
}

use crate::utils::{InitLayout, ConfigLayout};
use std::fs;
use std::path::{Path, PathBuf};

pub fn create(branch_name: &String) {
    let vault_folder = Path::new(".vault");
    let init_file = vault_folder.join("init.yaml");
    let content_bytes = fs::read(init_file).unwrap();
    let content = String::from_utf8_lossy(&content_bytes);
    let mut init_content: InitLayout = serde_yaml::from_str(&content).unwrap();

    init_content.branches.push(branch_name.clone());
    let branch_folder: PathBuf = vault_folder.join(branch_name.clone());
    fs::create_dir(&branch_folder).unwrap();
    create_config_yaml(&branch_folder);
    println!("content of init.yaml file \n{:?}", init_content);
}


fn create_config_yaml(folder_path: &PathBuf){
    let config_path = folder_path.join("config.yaml");
    let yaml_struct = ConfigLayout::default();
    let yaml_string = serde_yaml::to_string(&yaml_struct).unwrap();
    fs::write(config_path, yaml_string).unwrap();
}

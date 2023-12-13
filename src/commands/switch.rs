use crate::utils::InitLayout;
use std::fs;
use std::path::Path;


pub fn switch(branch_name: &String) {
    let vault_folder = Path::new(".vault");
    let init_file = vault_folder.join("init.yaml");
    let content_bytes = fs::read(&init_file).unwrap();
    let content = String::from_utf8_lossy(&content_bytes);
    let mut init_content: InitLayout = serde_yaml::from_str(&content).unwrap();
    if init_content.branches.contains(branch_name){
        init_content.current_branch = branch_name.to_string();
        println!("Branch switched to: {}",branch_name);
        let yaml_string = serde_yaml::to_string(&init_content).unwrap();
        fs::write(init_file, yaml_string).unwrap();
    } else {
        println!("Invalid branch name: {} Please check for spell error",branch_name);
    };
}

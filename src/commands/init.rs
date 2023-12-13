//This will have an init function which will currently create a .vault folder in the current directory
// VAULT INIT
use crate::commands::{create, switch};
use crate::utils::InitLayout;
use std::fs::{self, create_dir};
use std::path::Path;


pub fn init() {
    let vault_folder = Path::new(".vault");
//     if vault_folder.exists() {
//         println!("Vault already initialized")
//     }
   match create_dir(vault_folder) {
       Ok(_) => {
           let init_file = vault_folder.join("init.yaml");
           let content: String = get_init_data();
           fs::write(init_file, content).unwrap();
        // @TODO: Add call to branch function with default(master) parameter.
            create(&"master".to_string());
        // @TODO: Add call to switch function with default(master) parameter.
            switch(&"master".to_string());
       },
       Err(_) => println!("Vault already initialized")
   }
}

fn get_init_data() -> String {
   let yaml_struct = InitLayout {
       current_branch: String::new(),
       branches: Vec::new(),
   };

    let yaml_string = serde_yaml::to_string(&yaml_struct).unwrap();
    return yaml_string;
}

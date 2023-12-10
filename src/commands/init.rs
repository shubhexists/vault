//This will have an init function which will currently create a .vault folder in the current directory
// VAULT INIT
use std::fs::create_dir;
use std::path::Path;

pub fn init() {
    let vault_folder = Path::new(".vault");
    if vault_folder.exists() {
        println!("Vault already initialized");
    } else {
        create_dir(vault_folder).unwrap();
    }
    // @TODO: Add call to branch function with default(master) parameter.
    // @TODO: Add call to switch function with default(master) parameter.
}

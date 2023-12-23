use crate::utils::get_current_branch::get_current_branch;
use crate::utils::yaml_layouts::InitLayout;
use std::fs;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path,PathBuf};

pub fn delete(branch_to_delete: &str) -> io::Result<()> {
    if branch_to_delete == "." {
        let _ = fs::remove_dir_all(".vault");
        let _ = fs::remove_file(".vaultignore");
        Ok(())
    } else {
        let init_file: &Path = Path::new(".vault/init.yaml");
        let file: Result<File, io::Error> = File::open(&init_file);
        match file {
            Ok(file) => {
                let reader: BufReader<File> = BufReader::new(file);
                let mut parsed: InitLayout =
                    serde_yaml::from_reader(reader).expect("Failed to read YAML");
                let current_branch: Result<String, io::Error> = get_current_branch();
                match current_branch {
                    Ok(current_branch) => {
                        if current_branch == branch_to_delete {
                            panic!("You can not delete an Active Branch!Kindly switch to another branch first.")
                        } else {
                            let exists: bool = does_branch_exists(&branch_to_delete, &parsed);
                            if exists {
                                let vault_path: &Path = Path::new(".vault");
                                let branch_to_delete_path: PathBuf = vault_path.join(branch_to_delete);
                                let _ = fs::remove_dir_all(branch_to_delete_path);
                                parsed.branches.retain(|x| x != branch_to_delete);
                                let yaml_string: String = serde_yaml::to_string(&parsed).unwrap();
                                fs::write(init_file,yaml_string).unwrap();
                                Ok(())
                            } else {
                                panic!("Branch not found")
                            }
                        }
                    }
                    Err(e) => {
                        panic!("Some Error Occurred: {e}")
                    }
                }
            }
            Err(_e) => {
                panic!(
                    "Couldn't detect directory as a vault. Kindly Run `vault init` to make a vault"
                )
            }
        }
    }
}

fn does_branch_exists(branch_to_check: &str, parsed: &InitLayout) -> bool {
    let branch_array: &Vec<String> = &parsed.branches;
    branch_array.contains(&branch_to_check.to_string())
}

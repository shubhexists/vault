use std::{
    borrow::Cow,
    fs, io,
    path::{Path, PathBuf},
};

use crate::utils::{
    get_current_branch::get_current_branch,
    yaml_layouts::{Commit, ConfigLayout},
};

pub fn log() -> io::Result<()> {
    let current_branch: Result<String, std::io::Error> = get_current_branch();
    match current_branch {
        Ok(current_branch) => {
            let vault_path: &Path = Path::new(".vault");
            let branch_path: PathBuf = vault_path.join(current_branch);
            let config_path: PathBuf = branch_path.join("config.yaml");
            let content_bytes: Vec<u8> = fs::read(config_path).expect("Unable to read config.yaml");
            let content: Cow<'_, str> = String::from_utf8_lossy(&content_bytes);
            let config_content: ConfigLayout = serde_yaml::from_str(&content).unwrap();
            let commits: Vec<Commit> = config_content.commits;
            for commit in commits {
                println!("\u{001b}[33mcommit {} \u{001b}[0m", commit.hash);
                println!("Author: {}", commit.author);
                println!("Date:   {}", commit.date);
                println!();
                if commit.message == "" {
                    println!("   \u{001b}[34m[No Commit Message] \u{001b}[0m");
                } else {
                    println!("   {}", commit.message);
                }
                println!();
            }
            Ok(())
        }
        Err(e) => panic!("Some error occurred: {e}"),
    }
}

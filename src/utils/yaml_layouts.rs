use std::path::Path;

use serde::{Deserialize, Serialize};

use super::get_current_branch::{self, get_current_branch};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct InitLayout {
    pub current_branch: String,
    pub branches: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ConfigLayout {
    head: String,
    commits: Vec<Commit>,
    hashes: Vec<Hash>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Commit {
    hash: String,
    message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Hash {
    hash: String,
}

impl Default for ConfigLayout {
    fn default() -> Self {
        ConfigLayout {
            head: String::new(),
            commits: Vec::new(),
            hashes: Vec::new(),
        }
    }
}

impl ConfigLayout {
    fn add_commit(commit_data: &Commit) {
        let current_branch: Result<String, std::io::Error> = get_current_branch();
        match current_branch {
            Ok(current_branch) => {
                let vault_path: &Path = Path::new(".vault");
                let branch_path: std::path::PathBuf = vault_path.join(current_branch);
                let config_path: std::path::PathBuf = branch_path.join("config.yaml");
                todo!()
            }
            Err(e) => panic!("Some error occurred: {e}"),
        }
    }
}

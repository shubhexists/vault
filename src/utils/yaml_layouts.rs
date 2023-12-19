use super::get_current_branch::get_current_branch;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct InitLayout {
    pub created_at: String,
    pub current_branch: String,
    pub branches: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ConfigLayout {
    created_at: String,
    head: String,
    commits: Vec<Commit>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Hash {
    hash: String,
}

impl Default for ConfigLayout {
    fn default() -> Self {
        ConfigLayout {
            created_at: Utc::now().to_string(),
            head: String::new(),
            commits: Vec::new(),
        }
    }
}

impl ConfigLayout {
    pub fn add_commit(commit_data: Commit) -> io::Result<()> {
        let current_branch: Result<String, std::io::Error> = get_current_branch();
        match current_branch {
            Ok(current_branch) => {
                let vault_path: &Path = Path::new(".vault");
                let branch_path: std::path::PathBuf = vault_path.join(current_branch);
                let config_path: std::path::PathBuf = branch_path.join("config.yaml");
                let content_bytes: Vec<u8> =
                    fs::read(&config_path).expect("Unable to read config.yaml");
                let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&content_bytes);
                let mut config_content: ConfigLayout = serde_yaml::from_str(&content).unwrap();
                config_content.head = commit_data.hash.to_string();
                config_content.commits.push(commit_data);
                let yaml_string: String = serde_yaml::to_string(&config_content).unwrap();
                fs::write(config_path, yaml_string).unwrap();
                Ok(())
            }
            Err(e) => panic!("Some error occurred: {e}"),
        }
    }
    pub fn get_last_commit() -> Option<Commit> {
        let current_branch: Result<String, std::io::Error> = get_current_branch();
        match current_branch {
            Ok(current_branch) => {
                let vault_path: &Path = Path::new(".vault");
                let branch_path: std::path::PathBuf = vault_path.join(current_branch);
                let config_path: std::path::PathBuf = branch_path.join("config.yaml");
                let content_bytes: Vec<u8> =
                    fs::read(config_path).expect("Unable to read config.yaml");
                let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&content_bytes);
                let config_content: ConfigLayout = serde_yaml::from_str(&content).unwrap();
                let last_commit: Option<Commit> = config_content.commits.last().cloned();
                last_commit
            }
            Err(e) => panic!("Some error occurred: {e}"),
        }
    }
}

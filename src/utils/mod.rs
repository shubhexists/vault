pub mod read_files;
use serde::{Serialize, Deserialize};

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
    short: String,
    long: String,
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
    fn add_commit()
    {
        // @TODO: Add login to update commit in struct.
    }
}

pub mod read_files;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct InitLayout { 
    pub current_branch: String, 
    pub branches: Vec<String>,
}

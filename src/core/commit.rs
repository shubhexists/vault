use super::types::GitObject;

#[derive(Debug)]
pub struct Commit {
    //Add timestamp and Timezone
    message: String,
    author: String,
    timestamp: u64,
    changes: Vec<(String, GitObject)>,
    parent: Option<Vec<Commit>>
}


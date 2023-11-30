use std::collections::HashMap;
use super::{types::GitObject, commit::Commit};

struct Repository {
    objects: HashMap<String, GitObject>,
    //These won't be implemented initially in my opinion
    branches: HashMap<String, Commit>,
    current_branch: String,
}
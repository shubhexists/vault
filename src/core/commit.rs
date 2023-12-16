use super::tree::Tree;
use chrono::{DateTime, Utc};
use std::io;

/* SAMPLE COMMIT

commit 257\0
tree 41\0blob 41\0fileA.txt\0123456789abcdef0123456789abcdef01234567
parent abcdef0123456789abcdef0123456789abcdef01
author John Doe 

Commit message goes here.
 */

#[derive(Debug)]
pub struct Commit {
    date_time: DateTime<Utc>,
    message: String,
    author: String,
    commit_hash: String,
    parent: Option<Tree>,
}

impl Commit {
    //This function is not error handled
    pub fn new_commit(commit_message: &str, root_repository_tree_hash: String) -> io::Result<Self> {
        let date_time: DateTime<Utc> = Utc::now();
        let username: String = whoami::realname();
        let parent: Option<Tree> = Option::None;
        let commit_hash: String = root_repository_tree_hash;
        Ok(Commit {
            date_time,
            message: commit_message.to_string(),
            author: username,
            commit_hash,
            parent
        })
    }
}

use std::io;
use chrono::{DateTime, Utc};
use super::types::GitObject;

/* SAMPLE COMMIT 

commit 257\0
tree 41\0blob 41\0fileA.txt\0123456789abcdef0123456789abcdef01234567
parent abcdef0123456789abcdef0123456789abcdef01
author John Doe <john@example.com> 1638286773 -0500
committer John Doe <john@example.com> 1638286773 -0500

Commit message goes here.
 */


#[derive(Debug)]
pub struct Commit {
    date: DateTime<Utc>,
    message: String,
    author: String,
    timestamp: u64,
    changes: Vec<(String, GitObject)>,
    parent: Option<Vec<Commit>>
}

impl Commit {
    pub fn new_commit() -> io::Result<Self>{
        let date = Utc::now();
        todo!()
    }
}
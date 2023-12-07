use super::tree::Tree;
use chrono::{DateTime, Utc};
use std::io;

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
    date_time: DateTime<Utc>,
    message: String,
    author: String, // Maybe make this a enum of username and email maybe other details?
    tree: Tree,
    parent: Option<Vec<Commit>>,
}

impl Commit {
    //This function is not error handled
    pub fn new_commit(commit_message: &str, root_repository_tree: Tree) -> io::Result<Self> {
        let date_time: DateTime<Utc> = Utc::now();
        let username: String = whoami::realname();
        //Retrieve the parent commits from the yaml file maybe?
        //For now
        let parent: Option<Vec<Commit>> = Option::None;
        Ok(Commit {
            date_time,
            message: commit_message.to_string(),
            author: username,
            tree: root_repository_tree,
            parent,
        })
    }
    
}

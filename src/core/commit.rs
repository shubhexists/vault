use chrono::{DateTime, Utc};
use std::io;

/* SAMPLE COMMIT

tree `sha_256_hash`
parent abcdef0123456789abcdef0123456789abcdef01
author Shubham Singh
data_time `data_time-UTC`
message `Commit message goes here.`

*/

#[derive(Debug)]
pub struct Commit {
    date_time: DateTime<Utc>,
    message: String,
    author: String,
    commit_hash: String,
    parent: Option<String>,
}

impl Commit {
    //This function is not error handled
    pub fn new_commit(commit_message: &str, root_repository_tree_hash: String) -> io::Result<Self> {
        let date_time: DateTime<Utc> = Utc::now();
        let username: String = whoami::realname();
        //@TODO - For now kept it None, read the last commit in the yaml and add it to this param..
        let parent: Option<String> = Option::None;
        let commit_hash: String = root_repository_tree_hash;
        Ok(Commit {
            date_time,
            message: commit_message.to_string(),
            author: username,
            commit_hash,
            parent,
        })
    }
    pub fn get_content_of_commit(self) -> io::Result<String> {
        let mut content: String = String::new();
        content.push_str("tree ");
        content.push_str(&self.commit_hash);
        content.push('\n');
        match self.parent {
            Some(parent_hash) => {
                content.push_str(&parent_hash);
                content.push('\n')
            }
            None => content.push('\n'),
        }
        content.push_str("author ");
        content.push_str(&self.author);
        content.push('\n');
        content.push_str("date_time ");
        //@TODO - Date here
        content.push('\n');
        content.push_str("message ");
        content.push_str(&self.message);
        Ok(content)
    }
}

use crate::utils::yaml_layouts;
use crate::utils::yaml_layouts::ConfigLayout;
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
        let parent: Option<yaml_layouts::Commit> = ConfigLayout::get_last_commit();
        let commit_hash: String = root_repository_tree_hash;
        match parent {
            Some(parent) => {
                let parent = Some(parent.hash);
                Ok(Commit {
                    date_time,
                    message: commit_message.to_string(),
                    author: username,
                    commit_hash,
                    parent,
                })
            }
            None => Ok(Commit {
                date_time,
                message: commit_message.to_string(),
                author: username,
                commit_hash,
                parent: Option::None,
            }),
        }
    }

    pub fn get_content_of_commit(self) -> String {
        let mut content: String = String::new();
        let date_str: &String = &self.date_time.to_string();
        content.push_str("tree ");
        content.push_str(&self.commit_hash);
        content.push('\n');
        content.push_str("parent ");
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
        content.push_str(&date_str);
        content.push('\n');
        content.push_str("message ");
        content.push_str(&self.message);
        content
    }
}

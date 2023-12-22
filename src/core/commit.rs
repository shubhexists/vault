use crate::utils::yaml_layouts;
use crate::utils::yaml_layouts::ConfigLayout;
use chrono::Utc;
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
    date_time: String,
    message: String,
    author: String,
    pub commit_hash: String,
    parent: Option<String>,
    pub parent_folder_name: String,
}

impl Commit {
    //This function is not error handled
    pub fn new_commit(
        commit_message: &str,
        root_repository_tree_hash: String,
        parent_folder_name: String,
    ) -> io::Result<Self> {
        let date_time: String = Utc::now().to_string();
        let username: String = whoami::realname();
        let parent: Option<yaml_layouts::Commit> = ConfigLayout::get_last_commit();
        let commit_hash: String = root_repository_tree_hash;
        match parent {
            Some(parent) => {
                let parent: Option<String> = Some(parent.hash);
                Ok(Commit {
                    date_time,
                    message: commit_message.to_string(),
                    author: username,
                    commit_hash,
                    parent,
                    parent_folder_name,
                })
            }
            None => Ok(Commit {
                date_time,
                message: commit_message.to_string(),
                author: username,
                commit_hash,
                parent: Option::None,
                parent_folder_name,
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
        content.push('\n');
        content.push_str("root_dir ");
        content.push_str(&self.parent_folder_name);
        content
    }

    pub fn get_commit_from_content(commit_content: &String) -> Commit {
        let break_by_new_line_char: Vec<&str> = commit_content.split("\n").collect();
        let is_valid_commit: bool = Commit::check_valid_content(&break_by_new_line_char);
        if is_valid_commit {
            let commit_object: Commit = Commit::parse_vec_of_contents(&break_by_new_line_char);
            return commit_object;
        }
        panic!("Unable to get commit content!")
    }

    fn check_valid_content(contents: &Vec<&str>) -> bool {
        if contents.len() == 6 {
            if contents[0].contains("tree ") {
                if contents[1].contains("parent") {
                    if contents[2].contains("author") {
                        if contents[3].contains("date_time") {
                            if contents[4].contains("message") {
                                if contents[5].contains("root_dir") {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn parse_vec_of_contents(contents: &Vec<&str>) -> Commit {
        let tree_hash: &str = &contents[0][5..];
        let parent = || -> Option<String> {
            if contents[1][7..].is_empty() {
                None
            } else {
                Some(contents[1][7..].to_string())
            }
        };
        let author: &str = &contents[2][7..];
        let date_time: &str = &contents[3][10..];
        let message: &str = &contents[4][8..];
        let root_dir: &str = &contents[5][9..];
        Commit {
            commit_hash: tree_hash.to_string(),
            message: message.to_string(),
            date_time: date_time.to_string(),
            author: author.to_string(),
            parent: parent(),
            parent_folder_name: root_dir.to_string(),
        }
    }
}

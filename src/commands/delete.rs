use std::fs;
use std::io;

use crate::utils::get_current_branch::get_current_branch;

pub fn delete(branch_to_delete: &str) -> io::Result<()> {
    if branch_to_delete == "." {
        let _ = fs::remove_dir_all(".vault");
        let _ = fs::remove_file(".vaultignore");
    } else {
        let current_branch: Result<String, io::Error> = get_current_branch();
        match current_branch {
            Ok(current_branch) => {
                if current_branch == branch_to_delete {
                    panic!("You can not delete an Active Branch!")
                } else {
                    //@TODO - Get add branches and check if the branch actually exists
                }
            }
            Err(e) => {
                panic!("Some Error Occurred: {e}")
            }
        }
    }
    panic!("Some unknown error occurred!")
}

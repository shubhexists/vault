use crate::core::commit::Commit;

#[test]
fn test_check_valid_content() {
    let valid_contents = "tree abcdef0123456789abcdef0123456789abcdef01
parent 1234567890123456789012345678901234567890
author Shubham Singh
date_time 2023-01-01 12:00:00 UTC
message Test commit
root_dir parent_folder";

    let output_commit_object: Commit = Commit {
        date_time: "2023-01-01 12:00:00 UTC".to_string(),
        message: "Test commit".to_string(),
        author: "Shubham Singh".to_string(),
        commit_hash: "abcdef0123456789abcdef0123456789abcdef01".to_string(),
        parent: Some("1234567890123456789012345678901234567890".to_string()),
        parent_folder_name: "parent_folder".to_string(),
    };
    assert_eq!(
        Commit::get_commit_from_content(&valid_contents.to_string()),
        output_commit_object
    );
}

#[test]
#[should_panic = "The content provided for the commit objects are invalid"]
fn test_check_invalid_content() {}

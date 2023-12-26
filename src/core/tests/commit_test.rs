use crate::core::commit::Commit;

#[test]
fn commit_content() {
    let valid_contents: &str = "tree abcdef0123456789abcdef0123456789abcdef01
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
        Commit::get_content_of_commit(output_commit_object),
        valid_contents
    );
}

#[test]
fn valid_content() {
    let valid_contents: &str = "tree abcdef0123456789abcdef0123456789abcdef01
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
fn valid_content_no_parent() {
    let valid_contents: &str = "tree abcdef0123456789abcdef0123456789abcdef01
parent 
author Shubham Singh
date_time 2023-01-01 12:00:00 UTC
message Test commit
root_dir parent_folder";

    let output_commit_object: Commit = Commit {
        date_time: "2023-01-01 12:00:00 UTC".to_string(),
        message: "Test commit".to_string(),
        author: "Shubham Singh".to_string(),
        commit_hash: "abcdef0123456789abcdef0123456789abcdef01".to_string(),
        parent: None,
        parent_folder_name: "parent_folder".to_string(),
    };
    assert_eq!(
        Commit::get_commit_from_content(&valid_contents.to_string()),
        output_commit_object
    );
}

#[test]
#[should_panic]
fn invalid_content_new_line() {
    let invalid_contents: &str = "tree abcdef0123456789abcdef0123456789abcdef01parent 1234567890123456789012345678901234567890
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
        Commit::get_commit_from_content(&invalid_contents.to_string()),
        output_commit_object
    );
}

#[test]
#[should_panic]
fn invalid_content_param_order() {
    let invalid_contents: &str = "tree abcdef0123456789abcdef0123456789abcdef01
author Shubham Singh
parent 1234567890123456789012345678901234567890
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
        Commit::get_commit_from_content(&invalid_contents.to_string()),
        output_commit_object
    );
}

#[test]
#[should_panic]
fn invalid_content_missing_param() {
    let invalid_contents: &str = "tree abcdef0123456789abcdef0123456789abcdef01
parent 1234567890123456789012345678901234567890
author Shubham Singh
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
        Commit::get_commit_from_content(&invalid_contents.to_string()),
        output_commit_object
    );
}

#[test]
#[should_panic]
fn invalid_content_missing_value() {
    let invalid_contents: &str = "tree abcdef0123456789abcdef0123456789abcdef01
parent 1234567890123456789012345678901234567890
author 
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
        Commit::get_commit_from_content(&invalid_contents.to_string()),
        output_commit_object
    );
}

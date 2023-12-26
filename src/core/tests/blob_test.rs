use crate::core::{blob::Blob, types::FileTypes};

#[test]
fn get_content_of_blob_utf8() {
    let blob: Blob = Blob {
        content_size: 10,
        content: String::from("Hello!"),
        is_utf8: FileTypes::Utf8,
    };

    let expected_content = "blob \0UTF\010\0Hello!";
    assert_eq!(blob.get_content_of_blob(), expected_content);
}

#[test]
fn get_content_of_blob_non_utf8() {
    let blob: Blob = Blob {
        content_size: 8,
        content: String::from("Binary\x00"),
        is_utf8: FileTypes::NonUTF8,
    };

    let expected_content: &str = "blob \0NonUTF\08\0Binary\x00";
    assert_eq!(blob.get_content_of_blob(), expected_content);
}

#[test]
fn get_blob_from_content_utf8() {
    let blob_content: &str = "blob \0UTF\012\0Hello, Rust!";
    let expected_blob: Blob = Blob {
        content_size: 12,
        content: String::from("Hello, Rust!"),
        is_utf8: FileTypes::Utf8,
    };

    assert_eq!(
        Blob::get_blob_from_content(&blob_content.to_string()),
        expected_blob
    );
}

#[test]
fn get_blob_from_content_non_utf8() {
    let blob_content: &str = "blob \0NonUTF\05\0Binary";
    let expected_blob: Blob = Blob {
        content_size: 5,
        content: String::from("Binary"),
        is_utf8: FileTypes::NonUTF8,
    };

    assert_eq!(
        Blob::get_blob_from_content(&blob_content.to_string()),
        expected_blob
    );
}

#[test]
#[should_panic(expected = "Invalid Blob Content")]
fn get_blob_from_content_invalid_content() {
    let invalid_blob_content: &str = "invalid_content";
    Blob::get_blob_from_content(&invalid_blob_content.to_string());
}

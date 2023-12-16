//Docummentation - https://docs.rs/sha256/latest/sha256/
use sha256::digest;

pub fn hash_in_sha256(string_to_hash: &str) -> String {
    let input_string: String = String::from(&string_to_hash.to_string());
    let hashed_value: String = digest(input_string);
    hashed_value
}

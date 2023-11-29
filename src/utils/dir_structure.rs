use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn get_directory_structure(
    directory_path: &str,
) -> HashMap<String, Vec<HashMap<String, String>>> {
    let path: &Path = Path::new(directory_path);

    if path.is_dir() {
        let mut result: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
        let mut files: Vec<HashMap<String, String>> = Vec::new();

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name: String = entry.file_name().to_string_lossy().into_owned();
                    let file_path: std::path::PathBuf = entry.path();

                    let is_directory: bool = file_path.is_dir();

                    let mut file_info: HashMap<String, String> = HashMap::new();
                    file_info.insert("name".to_string(), file_name.clone());
                    file_info.insert("is_directory".to_string(), is_directory.to_string());
                    file_info.insert("is_read".to_string(), "false".to_string());

                    if is_directory {
                        let subdirectory_structure: HashMap<String, Vec<HashMap<String, String>>> =
                            get_directory_structure(&file_path.to_string_lossy());
                        file_info.insert(
                            "contents".to_string(),
                            serde_json::to_string(&subdirectory_structure).unwrap(),
                        );
                    }

                    files.push(file_info);
                }
            }
        }

        result.insert("contents".to_string(), files);
        result
    } else {
        HashMap::new()
    }
}

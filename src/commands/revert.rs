use crate::utils::{
    create_fs_from_hash::create_fs,
    get_current_branch::get_current_branch,
    yaml_layouts::{self, ConfigLayout},
};
use std::{
    borrow::Cow,
    fs, io,
    path::{Path, PathBuf},
};

pub fn revert(level: &usize, path: &String) -> io::Result<()> {
    let current_branch: Result<String, io::Error> = get_current_branch();
    match current_branch {
        Ok(current_branch) => {
            let vault_path: &Path = Path::new(".vault");
            let branch_path: PathBuf = vault_path.join(&current_branch);
            let config_path: PathBuf = branch_path.join("config.yaml");
            let content_bytes: Vec<u8> = fs::read(config_path).expect("Unable to read config.yaml");
            let content: Cow<'_, str> = String::from_utf8_lossy(&content_bytes);
            let config_content: ConfigLayout = serde_yaml::from_str(&content).unwrap();
            let commits: Vec<yaml_layouts::Commit> = config_content.commits;
            if let Some(commit) = commits.get(commits.len() - 1 - level) {
                let n_th_last_commit_hash: &String = &commit.hash;
                let _ = create_fs(n_th_last_commit_hash, path, &current_branch);
            } else {
                panic!("Level provided is out of bound!")
            }
            Ok(())
        }
        Err(e) => panic!("Some error occurred: {e}"),
    }
}

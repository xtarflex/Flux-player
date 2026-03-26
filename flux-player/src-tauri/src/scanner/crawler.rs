use walkdir::WalkDir;

pub fn walk_directory(dir_path: &str) -> Vec<std::path::PathBuf> {
    WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.path().to_path_buf())
        .collect()
}

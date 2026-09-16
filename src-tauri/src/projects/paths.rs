use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ValidatedPath {
    pub display_path: String,
    pub canonical_path: PathBuf,
    pub normalized_path: String,
}

pub fn validate_project_path(input: &str) -> Result<ValidatedPath, String> {
    let display_path = input.trim();
    if display_path.is_empty() {
        return Err("project path cannot be empty".to_string());
    }

    let selected = PathBuf::from(display_path);
    let metadata = std::fs::metadata(&selected).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            "selected project path does not exist".to_string()
        } else {
            "selected project path is unreadable or inaccessible".to_string()
        }
    })?;
    if !metadata.is_dir() {
        return Err("selected project path is not a directory".to_string());
    }
    let canonical_path = std::fs::canonicalize(&selected)
        .map_err(|_| "selected project path is unreadable or inaccessible".to_string())?;
    let normalized_path = normalize_path(&canonical_path);
    Ok(ValidatedPath {
        display_path: display_path.to_string(),
        canonical_path,
        normalized_path,
    })
}

pub fn normalize_path(path: &Path) -> String {
    let mut normalized = path.to_string_lossy().replace('/', "\\");
    while normalized.len() > 3 && normalized.ends_with('\\') {
        normalized.pop();
    }
    if cfg!(windows) {
        normalized.make_ascii_lowercase();
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn empty_and_file_paths_are_rejected() {
        assert!(validate_project_path(" ").is_err());
        let directory = tempdir().unwrap();
        let file = directory.path().join("file.txt");
        std::fs::write(&file, "fixture").unwrap();
        assert!(validate_project_path(file.to_str().unwrap()).is_err());
    }

    #[test]
    fn existing_folder_is_canonicalized_without_mutation() {
        let directory = tempdir().unwrap();
        let child = directory.path().join("project");
        std::fs::create_dir(&child).unwrap();
        let before = std::fs::read_dir(directory.path()).unwrap().count();
        let validated = validate_project_path(child.to_str().unwrap()).unwrap();
        assert!(validated.canonical_path.is_dir());
        assert!(!validated.normalized_path.is_empty());
        assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), before);
    }
}

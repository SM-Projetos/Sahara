use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
}

pub fn read_directory(path: &Path) -> Result<Vec<DirectoryEntry>, String> {
    let entries = fs::read_dir(path)
        .map_err(|error| format!("Não foi possível abrir {}: {error}", path.display()))?;

    let mut result = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|error| format!("Falha ao ler um item: {error}"))?;
        let metadata = entry.metadata()
            .map_err(|error| format!("Não foi possível consultar {}: {error}", entry.path().display()))?;

        result.push(DirectoryEntry {
            name: entry.file_name().to_string_lossy().into_owned(),
            path: entry.path(),
            is_directory: metadata.is_dir(),
        });
    }

    result.sort_by(|a, b| {
        b.is_directory.cmp(&a.is_directory)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn lists_directories_before_files_and_sorts_names() {
        let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("sahara-test-{unique}"));
        fs::create_dir_all(root.join("zeta")).unwrap();
        fs::create_dir_all(root.join("Alpha")).unwrap();
        fs::write(root.join("b.txt"), "").unwrap();
        fs::write(root.join("a.txt"), "").unwrap();

        let entries = read_directory(&root).unwrap();
        let names: Vec<_> = entries.iter().map(|entry| entry.name.as_str()).collect();
        assert_eq!(names, ["Alpha", "zeta", "a.txt", "b.txt"]);
        assert!(entries[0].is_directory);
        assert!(!entries[2].is_directory);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn reports_missing_directory() {
        let missing = std::env::temp_dir().join("sahara-directory-that-does-not-exist");
        assert!(read_directory(&missing).is_err());
    }
}

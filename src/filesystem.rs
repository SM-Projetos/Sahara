use std::env;
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

/// Resolve one of the navigation shortcuts to an absolute filesystem path.
pub fn resolve_location(home: &Path, location: &str) -> Option<PathBuf> {
    match location {
        "home" => Some(home.to_path_buf()),
        "desktop" => Some(resolve_user_directory(home, "XDG_DESKTOP_DIR", "Desktop")),
        "documents" => Some(resolve_user_directory(home, "XDG_DOCUMENTS_DIR", "Documents")),
        "downloads" => Some(resolve_user_directory(home, "XDG_DOWNLOAD_DIR", "Downloads")),
        "pictures" => Some(resolve_user_directory(home, "XDG_PICTURES_DIR", "Pictures")),
        "music" => Some(resolve_user_directory(home, "XDG_MUSIC_DIR", "Music")),
        "videos" => Some(resolve_user_directory(home, "XDG_VIDEOS_DIR", "Videos")),
        "filesystem" => Some(PathBuf::from("/")),
        _ => None,
    }
}

fn resolve_user_directory(home: &Path, variable: &str, fallback: &str) -> PathBuf {
    let config_home = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .unwrap_or_else(|| home.join(".config"));
    let config_file = config_home.join("user-dirs.dirs");

    configured_user_directory(home, &config_file, variable)
        .unwrap_or_else(|| home.join(fallback))
}

fn configured_user_directory(home: &Path, config_file: &Path, variable: &str) -> Option<PathBuf> {
    let contents = fs::read_to_string(config_file).ok()?;

    contents.lines().find_map(|line| {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return None;
        }

        let (key, value) = line.split_once('=')?;
        if key.trim() != variable {
            return None;
        }

        parse_xdg_directory_value(value, home)
    })
}

fn parse_xdg_directory_value(value: &str, home: &Path) -> Option<PathBuf> {
    let value = value.trim();
    let value = if value.starts_with('"') {
        value.strip_prefix('"')?.strip_suffix('"')?
    } else {
        value
    };

    // Preserve an escaped dollar sign so it is not expanded as $HOME.
    const ESCAPED_DOLLAR: char = '\u{e000}';
    let mut decoded = String::new();
    let mut chars = value.chars();

    while let Some(character) = chars.next() {
        if character == '\\' {
            let escaped = chars.next()?;
            decoded.push(if escaped == '$' { ESCAPED_DOLLAR } else { escaped });
        } else {
            decoded.push(character);
        }
    }

    let home = home.to_string_lossy();
    let expanded = decoded
        .replace("$HOME", home.as_ref())
        .replace(ESCAPED_DOLLAR, "$");
    let path = PathBuf::from(expanded);

    path.is_absolute().then_some(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_directory() -> PathBuf {
        let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!("sahara-test-{unique}"));
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn lists_directories_before_files_and_sorts_names() {
        let root = temporary_directory();
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
        let root = temporary_directory();
        let missing = root.join("does-not-exist");
        assert!(read_directory(&missing).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn resolves_xdg_user_directory_from_custom_configuration() {
        let root = temporary_directory();
        let home = root.join("home");
        let config_file = root.join("user-dirs.dirs");
        fs::create_dir_all(&home).unwrap();
        fs::write(
            &config_file,
            "XDG_DOCUMENTS_DIR=\"$HOME/Arquivos/Documentos\"\n",
        ).unwrap();

        let resolved = configured_user_directory(&home, &config_file, "XDG_DOCUMENTS_DIR");
        assert_eq!(resolved, Some(home.join("Arquivos/Documentos")));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_relative_xdg_user_directory_and_unknown_shortcuts() {
        let home = PathBuf::from("/home/test");
        assert_eq!(parse_xdg_directory_value("\"Documentos\"", &home), None);
        assert_eq!(resolve_location(&home, "unknown"), None);
        assert_eq!(resolve_location(&home, "filesystem"), Some(PathBuf::from("/")));
    }

    #[test]
    fn expands_home_but_keeps_escaped_dollar_literal() {
        let home = PathBuf::from("/home/test");
        assert_eq!(
            parse_xdg_directory_value("\"$HOME/Área de Trabalho\"", &home),
            Some(home.join("Área de Trabalho")),
        );
        assert_eq!(
            parse_xdg_directory_value("\"/mnt/\\$HOME-files\"", &home),
            Some(PathBuf::from("/mnt/$HOME-files")),
        );
    }
}

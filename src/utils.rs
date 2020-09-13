use std::path::{Path, PathBuf};

const FILE_EXTENSIONS: &[&str] = &["toml", "json", "yaml", "ini", "hjson"];

pub fn walk_to_root(mut path: PathBuf) -> Vec<PathBuf> {
    let mut config;
    let mut candidates = Vec::new();
    if path.is_file() {
        path = path.parent().unwrap_or_else(|| Path::new("/")).into();
    }
    for ancestor in path.ancestors() {
        config = ancestor.to_path_buf().join("config");
        candidates.push(ancestor.to_path_buf());
        candidates.push(config);
    }
    candidates
}

pub fn config_locations(
    root_path: PathBuf,
) -> (Option<PathBuf>, Option<PathBuf>) {
    let candidates = walk_to_root(root_path);
    let mut settings = None;
    let mut secrets = None;
    for cand in candidates {
        for &ext in FILE_EXTENSIONS {
            let settings_cand = cand.join(format!("settings.{}", ext));
            if settings_cand.exists() {
                settings = Some(settings_cand);
            }
            let secrets_cand = cand.join(format!(".secrets.{}", ext));
            if secrets_cand.exists() {
                secrets = Some(secrets_cand);
            }
            if settings.is_some() || secrets.is_some() {
                return (settings, secrets);
            }
        }
    }

    (None, None)
}

pub fn dotenv_location(root_path: PathBuf) -> Option<PathBuf> {
    let candidates = walk_to_root(root_path);
    for cand in candidates {
        let dotenv_cand = cand.join(".env");
        if dotenv_cand.exists() {
            return Some(dotenv_cand);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    fn get_data_path() -> PathBuf {
        let mut target_dir = PathBuf::from(
            env::current_exe()
                .expect("exe path")
                .parent()
                .expect("exe parent"),
        );
        while target_dir.file_name() != Some(std::ffi::OsStr::new("target")) {
            if !target_dir.pop() {
                panic!("Cannot find target directory");
            }
        }
        target_dir.pop();
        target_dir.join("tests/data")
    }

    #[test]
    fn test_walk_to_root_dir() {
        assert_eq!(
            walk_to_root(PathBuf::from("/a/dir/located/somewhere")),
            vec![
                PathBuf::from("/a/dir/located/somewhere"),
                PathBuf::from("/a/dir/located/somewhere/config"),
                PathBuf::from("/a/dir/located"),
                PathBuf::from("/a/dir/located/config"),
                PathBuf::from("/a/dir"),
                PathBuf::from("/a/dir/config"),
                PathBuf::from("/a"),
                PathBuf::from("/a/config"),
                PathBuf::from("/"),
                PathBuf::from("/config"),
            ],
        );
    }

    #[test]
    fn test_walk_to_root_root() {
        assert_eq!(
            walk_to_root(PathBuf::from("/")),
            vec![PathBuf::from("/"), PathBuf::from("/config")],
        );
    }

    #[test]
    fn test_config_locations() {
        let data_path = get_data_path();
        assert_eq!(
            config_locations(data_path.clone()),
            (
                Some(data_path.clone().join("config/settings.toml")),
                Some(data_path.join("config/.secrets.toml")),
            ),
        );
    }
}

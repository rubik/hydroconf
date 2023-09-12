use std::path::{Path, PathBuf};

#[cfg(not(feature = "tracing"))]
use crate::tracing;
#[cfg(feature = "tracing")]
use tracing;

const SETTINGS_FILE_EXTENSIONS: &[&str] = &[
    "toml",
    #[cfg(feature = "json")]
    "json",
    #[cfg(feature = "yaml")]
    "yaml",
    #[cfg(feature = "ini")]
    "ini",
    #[cfg(feature = "json5")]
    "hjson",
];
const SETTINGS_DIRS: &[&str] = &["", "config"];

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FileSources {
    pub settings: Option<PathBuf>,
    // Local settings file is generally not tracked by version control.
    pub local_settings: Option<PathBuf>,
    pub secrets: Option<PathBuf>,
    pub dotenv: Vec<PathBuf>,
}

impl FileSources {
    pub fn from_root(root_path: PathBuf, env: &str) -> Self {
        let mut sources = Self {
            settings: None,
            local_settings: None,
            secrets: None,
            dotenv: Vec::new(),
        };
        let mut settings_found = false;
        let candidates = walk_to_root(root_path);

        for cand in candidates {
            let dotenv_cand = cand.join(".env");
            if dotenv_cand.exists() {
                tracing::debug!("Collect from {:?}", dotenv_cand);
                sources.dotenv.push(dotenv_cand);
            }
            let dotenv_cand = cand.join(format!(".env.{}", env));
            if dotenv_cand.exists() {
                tracing::debug!("Collect from {:?}", dotenv_cand);
                sources.dotenv.push(dotenv_cand);
            }
            'outer: for &settings_dir in SETTINGS_DIRS {
                let dir = cand.join(settings_dir);
                for &ext in SETTINGS_FILE_EXTENSIONS {
                    let settings_cand = dir.join(format!("settings.{}", ext));
                    if settings_cand.exists() {
                        tracing::debug!("Collect from {:?}", settings_cand);
                        sources.settings = Some(settings_cand);
                        settings_found = true;
                    }
                    let local_settings_cand =
                        dir.join(format!("settings.local.{}", ext));
                    if local_settings_cand.exists() {
                        tracing::debug!(
                            "Collect from {:?}",
                            local_settings_cand
                        );
                        sources.local_settings = Some(local_settings_cand);
                        settings_found = true;
                    }
                    let secrets_cand = dir.join(format!(".secrets.{}", ext));
                    if secrets_cand.exists() {
                        tracing::debug!("Collect from {:?}", secrets_cand);
                        sources.secrets = Some(secrets_cand);
                        settings_found = true;
                    }
                    if settings_found {
                        break 'outer;
                    }
                }
            }

            if sources.any() {
                break;
            }
        }

        sources
    }

    fn any(&self) -> bool {
        self.settings.is_some()
            || self.secrets.is_some()
            || !self.dotenv.is_empty()
    }
}

pub fn walk_to_root(mut path: PathBuf) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if path.is_file() {
        path = path.parent().unwrap_or_else(|| Path::new("/")).into();
    }
    for ancestor in path.ancestors() {
        candidates.push(ancestor.to_path_buf());
    }
    candidates
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    fn get_data_path(suffix: &str) -> PathBuf {
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
        target_dir.join(format!("tests/data{}", suffix))
    }

    #[test]
    fn test_walk_to_root_dir() {
        assert_eq!(
            walk_to_root(PathBuf::from("/a/dir/located/somewhere")),
            vec![
                PathBuf::from("/a/dir/located/somewhere"),
                PathBuf::from("/a/dir/located"),
                PathBuf::from("/a/dir"),
                PathBuf::from("/a"),
                PathBuf::from("/"),
            ],
        );
    }

    #[test]
    fn test_walk_to_root_root() {
        assert_eq!(walk_to_root(PathBuf::from("/")), vec![PathBuf::from("/")],);
    }

    #[test]
    fn test_sources() {
        let data_path = get_data_path("");
        assert_eq!(
            FileSources::from_root(data_path.clone(), "development"),
            FileSources {
                settings: Some(data_path.clone().join("config/settings.toml")),
                local_settings: None,
                secrets: Some(data_path.join("config/.secrets.toml")),
                dotenv: vec![data_path.join(".env")],
            },
        );

        let data_path = get_data_path("2");
        assert_eq!(
            FileSources::from_root(data_path.clone(), "development"),
            FileSources {
                settings: Some(data_path.clone().join("config/settings.toml")),
                local_settings: None,
                secrets: Some(data_path.join("config/.secrets.toml")),
                dotenv: vec![
                    data_path.join(".env"),
                    data_path.join(".env.development")
                ],
            },
        );

        let data_path = get_data_path("2");
        assert_eq!(
            FileSources::from_root(data_path.clone(), "production"),
            FileSources {
                settings: Some(data_path.clone().join("config/settings.toml")),
                local_settings: None,
                secrets: Some(data_path.join("config/.secrets.toml")),
                dotenv: vec![data_path.join(".env")],
            },
        );

        let data_path = get_data_path("3");
        assert_eq!(
            FileSources::from_root(data_path.clone(), "development"),
            FileSources {
                settings: Some(data_path.clone().join("settings.toml")),
                local_settings: None,
                secrets: Some(data_path.join(".secrets.toml")),
                dotenv: vec![data_path.join(".env")],
            },
        );

        let data_path = get_data_path("3");
        assert_eq!(
            FileSources::from_root(data_path.clone(), "production"),
            FileSources {
                settings: Some(data_path.clone().join("settings.toml")),
                local_settings: None,
                secrets: Some(data_path.join(".secrets.toml")),
                dotenv: vec![
                    data_path.join(".env"),
                    data_path.join(".env.production")
                ],
            },
        );

        let data_path = get_data_path("4");
        assert_eq!(
            FileSources::from_root(data_path.clone(), "development"),
            FileSources {
                settings: Some(data_path.clone().join("settings.toml")),
                local_settings: Some(data_path.join("settings.local.toml")),
                secrets: Some(data_path.join(".secrets.toml")),
                dotenv: vec![],
            },
        );
    }
}

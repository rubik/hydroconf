use std::path::{Path, PathBuf, Component};

use normpath::PathExt;

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
    pub(crate) local_settings: Option<PathBuf>,
    pub secrets: Option<PathBuf>,
    pub dotenv: Vec<PathBuf>,
}

impl FileSources {
    pub fn from_root(
        root_path: &Path,
        env_name: &str,
        filename: Option<&Path>,
        secret_filename: Option<&Path>,
    ) -> Self {
        let mut sources = Self {
            settings: None,
            local_settings: None,
            secrets: None,
            dotenv: Vec::new(),
        };
        let candidates = walk_to_root(root_path);

        find_file(&candidates, Path::new(".env"))
            .map(|p| sources.dotenv.push(p));
        find_file(&candidates, Path::new(&format!(".env.{env_name}")))
            .map(|p| sources.dotenv.push(p));

        // Make sure the passed argument is a pure filename, not a path.
        let filename = filename
            .and_then(|path| path.file_name())
            .or_else(|| {
                tracing::warn!("Please pass pure file name, not path!");
                None
            })
            .map(Path::new);
        let secret_filename = secret_filename
            .and_then(|path| path.file_name())
            .or_else(|| {
                tracing::warn!("Please pass pure file name, not path!");
                None
            })
            .map(Path::new);
        if let Some(filename) = filename {
            if let Some((ext, stem)) =
                filename.extension().zip(filename.file_stem())
            {
                let ext = ext.to_string_lossy();
                if SETTINGS_FILE_EXTENSIONS.contains(&ext.as_ref()) {
                    sources.settings = find_file(&candidates, filename);
                    let stem = stem.to_string_lossy();
                    sources.local_settings = find_file(
                        &candidates,
                        Path::new(&format!("{stem}.local.{ext}")),
                    );
                } else {
                    tracing::warn!(
                        "Unsupported settings file extension: {}",
                        ext
                    );
                };
            }
        }
        if let Some(filename) = secret_filename {
            if let Some(ext) = filename.extension() {
                let ext = ext.to_string_lossy();
                if SETTINGS_FILE_EXTENSIONS.contains(&ext.as_ref()) {
                    sources.secrets = find_file(&candidates, filename);
                } else {
                    tracing::warn!(
                        "Unsupported secrets file extension: {}",
                        ext
                    );
                };
            }
        }

        sources
    }

    pub fn local_settings(&self) -> Option<&Path> {
        self.local_settings.as_deref()
    }
}

pub fn walk_to_root(path: &Path) -> Vec<PathBuf> {
    let normalized = path
        .normalize()
        .map_or_else(|_e| {
            tracing::warn!("Failed to normalize path: {}", _e);
            let p: &Path = Component::RootDir.as_ref();
            p.to_path_buf()
        }, |p| p.into_path_buf());
    let dir_path = if normalized.is_dir() {
        path
    } else {
        normalized.parent().unwrap_or_else(|| Component::RootDir.as_ref())
    };
    dir_path
        .ancestors()
        .into_iter()
        .map(|p| p.to_path_buf())
        .collect()
}

fn find_file(level_dirs: &Vec<PathBuf>, filename: &Path) -> Option<PathBuf> {
    for level_dir in level_dirs {
        for &settings_dir in SETTINGS_DIRS {
            let dir = level_dir.join(settings_dir);
            let file_path = dir.join(filename);
            if file_path.is_file() {
                tracing::debug!("Collect from {:?}", file_path);
                return Some(file_path);
            }
        }
    }
    None
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
            walk_to_root(Path::new("/a/dir/located/somewhere")),
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
        assert_eq!(walk_to_root(Path::new("/")), vec![PathBuf::from("/")],);
    }

    #[test]
    fn test_sources() {
        let data_path = get_data_path("");
        assert_eq!(
            FileSources::from_root(
                data_path.as_path(),
                "development",
                Some(Path::new("settings.toml")),
                Some(Path::new(".secrets.toml"))
            ),
            FileSources {
                settings: Some(data_path.clone().join("config/settings.toml")),
                local_settings: None,
                secrets: Some(data_path.join("config/.secrets.toml")),
                dotenv: vec![data_path.join(".env")],
            },
        );

        let data_path = get_data_path("2");
        assert_eq!(
            FileSources::from_root(
                data_path.as_path(),
                "development",
                Some(Path::new("settings.toml")),
                Some(Path::new(".secrets.toml"))
            ),
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
            FileSources::from_root(
                data_path.as_path(),
                "production",
                Some(Path::new("settings.toml")),
                Some(Path::new(".secrets.toml"))
            ),
            FileSources {
                settings: Some(data_path.clone().join("config/settings.toml")),
                local_settings: None,
                secrets: Some(data_path.join("config/.secrets.toml")),
                dotenv: vec![data_path.join(".env")],
            },
        );

        let data_path = get_data_path("3");
        assert_eq!(
            FileSources::from_root(
                data_path.as_path(),
                "development",
                Some(Path::new("settings.toml")),
                Some(Path::new(".secrets.toml"))
            ),
            FileSources {
                settings: Some(data_path.clone().join("settings.toml")),
                local_settings: None,
                secrets: Some(data_path.join(".secrets.toml")),
                dotenv: vec![data_path.join(".env")],
            },
        );

        let data_path = get_data_path("3");
        assert_eq!(
            FileSources::from_root(
                data_path.as_path(),
                "production",
                Some(Path::new("settings.toml")),
                Some(Path::new(".secrets.toml"))
            ),
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
            FileSources::from_root(
                data_path.as_path(),
                "development",
                Some(Path::new("settings.toml")),
                Some(Path::new(".secrets.toml"))
            ),
            FileSources {
                settings: Some(data_path.clone().join("settings.toml")),
                local_settings: Some(data_path.join("settings.local.toml")),
                secrets: Some(data_path.join(".secrets.toml")),
                dotenv: vec![],
            },
        );

        let data_path = get_data_path("_custom_filename");
        assert_eq!(
            FileSources::from_root(
                data_path.as_path(),
                "development",
                Some(Path::new("base_settings.toml")),
                None
            ),
            FileSources {
                settings: Some(
                    data_path.clone().join("config/base_settings.toml")
                ),
                local_settings: Some(
                    data_path.clone().join("base_settings.local.toml")
                ),
                secrets: None,
                dotenv: vec![],
            },
        );
    }
}

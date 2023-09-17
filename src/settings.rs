use std::path::PathBuf;

use crate::env;

pub const AUTO_SETTING_FILENAME: &str = "settings.toml";
pub const AUTO_SECRET_FILENAME: &str = ".secrets.toml";

#[derive(Debug, Clone, PartialEq)]
pub struct HydroSettings {
    pub root_path: Option<PathBuf>,
    pub settings_file: Option<PathBuf>,
    pub secrets_file: Option<PathBuf>,
    pub env: String,
    pub envvar_prefix: String,
    pub encoding: String,
    pub envvar_nested_sep: String,
}

impl Default for HydroSettings {
    fn default() -> Self {
        let hydro_suffix = "_FOR_HYDRO";
        Self {
            root_path: env::get_var("ROOT_PATH", hydro_suffix),
            settings_file: env::get_var("SETTINGS_FILE", hydro_suffix)
                .or(Some(AUTO_SETTING_FILENAME.into())),
            secrets_file: env::get_var("SECRETS_FILE", hydro_suffix)
                .or(Some(AUTO_SECRET_FILENAME.into())),
            env: env::get_var_default(
                "ENV",
                hydro_suffix,
                "development".into(),
            ),
            envvar_prefix: env::get_var_default(
                "ENVVAR_PREFIX",
                hydro_suffix,
                "HYDRO".into(),
            ),
            encoding: env::get_var_default(
                "ENCODING",
                hydro_suffix,
                "utf-8".into(),
            ),
            envvar_nested_sep: env::get_var_default(
                "ENVVAR_NESTED_SEP",
                hydro_suffix,
                "__".into(),
            ),
        }
    }
}

impl HydroSettings {
    pub fn set_root_path(mut self, p: PathBuf) -> Self {
        self.root_path = Some(p);
        self
    }

    pub fn set_settings_file(mut self, p: PathBuf) -> Self {
        self.settings_file = Some(p);
        self
    }

    pub fn set_secrets_file(mut self, p: PathBuf) -> Self {
        self.secrets_file = Some(p);
        self
    }

    pub fn set_env(mut self, e: String) -> Self {
        self.env = e;
        self
    }

    pub fn set_envvar_prefix(mut self, p: String) -> Self {
        self.envvar_prefix = p;
        self
    }

    pub fn set_encoding(mut self, e: String) -> Self {
        self.encoding = e;
        self
    }

    pub fn set_envvar_nested_sep(mut self, s: String) -> Self {
        self.envvar_nested_sep = s;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::{remove_var, set_var};

    #[test]
    fn test_default() {
        assert_eq!(
            HydroSettings::default(),
            HydroSettings {
                root_path: None,
                settings_file: Some("settings.toml".into()),
                secrets_file: Some(".secrets.toml".into()),
                env: "development".into(),
                envvar_prefix: "HYDRO".into(),
                encoding: "utf-8".into(),
                envvar_nested_sep: "__".into(),
            },
        );
    }

    #[test]
    fn test_default_with_env() {
        set_var("ENCODING_FOR_HYDRO", "latin-1");
        set_var("ROOT_PATH_FOR_HYDRO", "/an/absolute/path");
        assert_eq!(
            HydroSettings::default(),
            HydroSettings {
                root_path: Some("/an/absolute/path".into()),
                settings_file: Some("settings.toml".into()),
                secrets_file: Some(".secrets.toml".into()),
                env: "development".into(),
                envvar_prefix: "HYDRO".into(),
                encoding: "latin-1".into(),
                envvar_nested_sep: "__".into(),
            },
        );
        remove_var("ENCODING_FOR_HYDRO");
        remove_var("ROOT_PATH_FOR_HYDRO");
    }

    #[test]
    fn test_one_builder_method() {
        assert_eq!(
            HydroSettings::default()
                .set_root_path(PathBuf::from("~/test/dir")),
            HydroSettings {
                root_path: Some(PathBuf::from("~/test/dir")),
                settings_file: Some("settings.toml".into()),
                secrets_file: Some(".secrets.toml".into()),
                env: "development".into(),
                envvar_prefix: "HYDRO".into(),
                encoding: "utf-8".into(),
                envvar_nested_sep: "__".into(),
            },
        );
    }

    #[test]
    fn test_all_builder_methods() {
        assert_eq!(
            HydroSettings::default()
                .set_envvar_prefix("HY_".into())
                .set_encoding("latin-1".into())
                .set_secrets_file(PathBuf::from(".secrets.toml"))
                .set_env("production".into())
                .set_envvar_nested_sep("-".into())
                .set_root_path(PathBuf::from("~/test/dir"))
                .set_settings_file(PathBuf::from("settings.toml")),
            HydroSettings {
                root_path: Some(PathBuf::from("~/test/dir")),
                settings_file: Some(PathBuf::from("settings.toml")),
                secrets_file: Some(PathBuf::from(".secrets.toml")),
                env: "production".into(),
                envvar_prefix: "HY_".into(),
                encoding: "latin-1".into(),
                envvar_nested_sep: "-".into(),
            },
        );
    }
}

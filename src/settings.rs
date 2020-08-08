use std::path::PathBuf;

use crate::env;

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
        let envvar_prefix = env::get_var("HYDRO_", "ENVVAR_PREFIX")
            .unwrap_or(String::from("HYDRO_"));
        let envvar_prefix = envvar_prefix.as_str();
        Self {
            root_path: env::get_var(envvar_prefix, "ROOT_PATH"),
            settings_file: env::get_var(envvar_prefix, "SETTINGS_FILE"),
            secrets_file: env::get_var(envvar_prefix, "SECRETS_FILE"),
            env: env::get_var_default(
                envvar_prefix,
                "ENV",
                "development".into(),
            ),
            envvar_prefix: envvar_prefix.into(),
            encoding: env::get_var_default(
                envvar_prefix,
                "ENCODING",
                "utf-8".into(),
            ),
            envvar_nested_sep: "__".into(),
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
                settings_file: None,
                secrets_file: None,
                env: "development".into(),
                envvar_prefix: "HYDRO_".into(),
                encoding: "utf-8".into(),
                envvar_nested_sep: "__".into(),
            },
        );
    }

    #[test]
    fn test_default_with_env() {
        set_var("HYDRO_ENCODING", "latin-1");
        set_var("HYDRO_ROOT_PATH", "/an/absolute/path");
        assert_eq!(
            HydroSettings::default(),
            HydroSettings {
                root_path: Some("/an/absolute/path".into()),
                settings_file: None,
                secrets_file: None,
                env: "development".into(),
                envvar_prefix: "HYDRO_".into(),
                encoding: "latin-1".into(),
                envvar_nested_sep: "__".into(),
            },
        );
        remove_var("HYDRO_ENCODING");
        remove_var("HYDRO_ROOT_PATH");
    }

    #[test]
    fn test_default_with_env_and_custom_prefix() {
        set_var("HYDRO_ENVVAR_PREFIX", "HY_");
        set_var("HY_ROOT_PATH", "/an/absolute/path");
        set_var("HYDRO_ENCODING", "latin-1");
        assert_eq!(
            HydroSettings::default(),
            HydroSettings {
                root_path: Some("/an/absolute/path".into()),
                settings_file: None,
                secrets_file: None,
                env: "development".into(),
                envvar_prefix: "HY_".into(),
                encoding: "utf-8".into(),
                envvar_nested_sep: "__".into(),
            },
        );
        remove_var("HYDRO_ENVVAR_PREFIX");
        remove_var("HYDRO_ENCODING");
        remove_var("HY_ROOT_PATH");
    }

    #[test]
    fn test_one_builder_method() {
        assert_eq!(
            HydroSettings::default()
                .set_root_path(PathBuf::from("~/test/dir")),
            HydroSettings {
                root_path: Some(PathBuf::from("~/test/dir")),
                settings_file: None,
                secrets_file: None,
                env: "development".into(),
                envvar_prefix: "HYDRO_".into(),
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

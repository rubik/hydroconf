use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct HydroSettings {
    pub root_path: Option<&'static Path>,
    pub settings_file: Option<&'static Path>,
    pub secrets_file: Option<&'static Path>,
    pub env: String,
    pub envvar_prefix: String,
    pub encoding: String,
    pub envvar_nested_sep: String,
}


impl Default for HydroSettings {
    fn default() -> Self {
        // TODO: Get the default value from the env - 2020-08-08 08:34am
        Self {
            root_path: None,
            settings_file: None,
            secrets_file: None,
            env: "development".into(),
            envvar_prefix: "HYDRO_".into(),
            encoding: "utf-8".into(),
            envvar_nested_sep: "__".into(),
        }
    }
}

impl HydroSettings {
    pub fn set_root_path(mut self, p: &'static Path) -> Self {
        self.root_path = Some(p);
        self
    }

    pub fn set_settings_file(mut self, p: &'static Path) -> Self {
        self.settings_file = Some(p);
        self
    }

    pub fn set_secrets_file(mut self, p: &'static Path) -> Self {
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
        )
    }

    #[test]
    fn test_one_builder_method() {
        assert_eq!(
            HydroSettings::default().set_root_path(Path::new("~/test/dir")),
            HydroSettings {
                root_path: Some(Path::new("~/test/dir")),
                settings_file: None,
                secrets_file: None,
                env: "development".into(),
                envvar_prefix: "HYDRO_".into(),
                encoding: "utf-8".into(),
                envvar_nested_sep: "__".into(),
            },
        )
    }

    #[test]
    fn test_all_builder_methods() {
        assert_eq!(
            HydroSettings::default()
                .set_envvar_prefix("HY_".into())
                .set_encoding("latin-1".into())
                .set_secrets_file(Path::new(".secrets.toml"))
                .set_env("production".into())
                .set_envvar_nested_sep("-".into())
                .set_root_path(Path::new("~/test/dir"))
                .set_settings_file(Path::new("settings.toml"))
            ,
            HydroSettings {
                root_path: Some(Path::new("~/test/dir")),
                settings_file: Some(Path::new("settings.toml")),
                secrets_file: Some(Path::new(".secrets.toml")),
                env: "production".into(),
                envvar_prefix: "HY_".into(),
                encoding: "latin-1".into(),
                envvar_nested_sep: "-".into(),
            },
        )
    }
}

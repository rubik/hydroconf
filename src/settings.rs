use std::path::Path;

#[derive(Debug, Clone)]
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
            envvar_prefix: "HYDRO_".into(),
        }
    }
}

impl HydroSettings {
    pub fn set_root_path(mut self, p: Option<&'static Path>) -> Self {
        self.root_path = p;
        self
    }
}

use std::collections::HashMap;
use std::path::PathBuf;

pub use config::{
    builder::DefaultState, Config, ConfigBuilder, ConfigError, Environment,
    File, Value,
};
use dotenv_parser::parse_dotenv;
use serde::Deserialize;
use log::debug;

use crate::settings::HydroSettings;
use crate::sources::FileSources;
use crate::utils::path_to_string;

type Table = HashMap<String, Value>;
const PREFIX_SEPARATOR: &str = "_";

#[derive(Debug, Clone)]
pub struct Hydroconf {
    config: Config,
    // This builder is for per-environment config (the "config" field above)
    builder: ConfigBuilder<DefaultState>,
    orig_config: Config,
    hydro_settings: HydroSettings,
    sources: FileSources,
}

impl Default for Hydroconf {
    fn default() -> Self {
        Self::new(HydroSettings::default())
    }
}

impl Hydroconf {
    pub fn new(hydro_settings: HydroSettings) -> Self {
        Self {
            config: Config::default(),
            builder: Config::builder(),
            orig_config: Config::default(),
            hydro_settings,
            sources: FileSources::default(),
        }
    }

    pub fn hydrate<'de, T: Deserialize<'de>>(
        mut self,
    ) -> Result<T, ConfigError> {
        self.discover_sources();
        self.load_settings()?;
        self.merge_settings()?;
        self.override_from_dotenv()?;
        self.override_from_env()?;
        self.try_deserialize()
    }

    pub fn discover_sources(&mut self) {
        let HydroSettings {
            root_path,
            settings_file,
            secrets_file,
            env,
            ..
        } = &self.hydro_settings;
        self.sources = match root_path {
            Some(p) => FileSources::from_root(p, &env, settings_file.as_deref(), secrets_file.as_deref()),
            None => FileSources::default(),
        };
    }

    pub fn load_settings(&mut self) -> Result<&mut Self, ConfigError> {
        let mut builder = Config::builder();
        if let Some(ref settings_path) = self.sources.settings {
            builder = builder.add_source(File::from(settings_path.clone()));
        }
        if let Some(ref local_settings_path) = self.sources.local_settings {
            builder =
                builder.add_source(File::from(local_settings_path.clone()));
        }
        if let Some(ref secrets_path) = self.sources.secrets {
            builder = builder.add_source(File::from(secrets_path.clone()));
        }
        self.orig_config = builder.build()?;

        Ok(self)
    }

    pub fn merge_settings(&mut self) -> Result<&mut Self, ConfigError> {
        let mut builder = self.builder.clone();
        for &name in &["default", self.hydro_settings.env.as_str()] {
            let table_value: Option<Table> = self.orig_config.get(name).ok();
            if let Some(value) = table_value {
                let mut new_config = Config::default();
                new_config.cache = value.into();
                builder = builder.add_source(new_config);
            }
        }
        self.config = builder.build_cloned()?;
        self.builder = builder;

        Ok(self)
    }

    pub fn override_from_dotenv(&mut self) -> Result<&mut Self, ConfigError> {
        let mut builder = self.builder.clone();
        for dotenv_path in &self.sources.dotenv {
            let source = std::fs::read_to_string(dotenv_path.clone())
                .map_err(|e| ConfigError::FileParse {
                    uri: path_to_string(dotenv_path.clone()),
                    cause: e.into(),
                })?;
            let map =
                parse_dotenv(&source).map_err(|e| ConfigError::FileParse {
                    uri: path_to_string(dotenv_path.clone()),
                    cause: e.into(),
                })?;

            for (key, val) in map.iter() {
                if val.is_empty() {
                    continue;
                }
                let prefix = self.hydro_settings.envvar_prefix.to_lowercase()
                    + PREFIX_SEPARATOR;
                let mut key = key.to_lowercase();
                if !key.starts_with(&prefix) {
                    continue;
                } else {
                    key = key[prefix.len()..].to_string();
                }
                let sep = self.hydro_settings.envvar_nested_sep.clone();
                key = key.replace(&sep, ".");
                builder =
                    builder.set_override::<String, String>(key, val.into())?;
            }
        }
        self.config = builder.build_cloned()?;
        self.builder = builder;

        Ok(self)
    }

    pub fn override_from_env(&mut self) -> Result<&mut Self, ConfigError> {
        let env_source = Environment::with_prefix(
            self.hydro_settings.envvar_prefix.as_str(),
        )
        .prefix_separator(PREFIX_SEPARATOR)
        .separator(self.hydro_settings.envvar_nested_sep.as_str());
        debug!("Environment source: {:?}", env_source);
        let builder = self.builder.clone().add_source(env_source);
        self.config = builder.build_cloned()?;
        self.builder = builder;

        Ok(self)
    }

    pub fn root_path(&self) -> Option<PathBuf> {
        self.hydro_settings
            .root_path
            .clone()
            .or_else(|| std::env::current_exe().ok())
    }

    pub fn try_deserialize<'de, T: Deserialize<'de>>(
        self,
    ) -> Result<T, ConfigError> {
        self.config.try_deserialize()
    }

    //pub fn refresh(&mut self) -> Result<&mut Self, ConfigError> {
    //self.orig_config.refresh()?;
    //self.config.cache = Value::new(None, Table::new());
    //self.merge()?;
    //self.override_from_env()?;
    //Ok(self)
    //}

    pub fn set_default<T>(
        &mut self,
        key: &str,
        value: T,
    ) -> Result<&mut Self, ConfigError>
    where
        T: Into<Value>,
    {
        let builder = self.builder.clone().set_default(key, value)?;
        self.config = builder.build_cloned()?;
        self.builder = builder;
        Ok(self)
    }

    pub fn set<T>(
        &mut self,
        key: &str,
        value: T,
    ) -> Result<&mut Self, ConfigError>
    where
        T: Into<Value>,
    {
        let builder = self.builder.clone().set_override(key, value)?;
        self.config = builder.build_cloned()?;
        self.builder = builder;
        Ok(self)
    }

    pub fn get<'de, T>(&self, key: &'de str) -> Result<T, ConfigError>
    where
        T: Deserialize<'de>,
    {
        self.config.get(key)
    }

    pub fn get_str(&self, key: &str) -> Result<String, ConfigError> {
        self.get(key).and_then(Value::into_string)
    }

    pub fn get_int(&self, key: &str) -> Result<i64, ConfigError> {
        self.get(key).and_then(Value::into_int)
    }

    pub fn get_float(&self, key: &str) -> Result<f64, ConfigError> {
        self.get(key).and_then(Value::into_float)
    }

    pub fn get_bool(&self, key: &str) -> Result<bool, ConfigError> {
        self.get(key).and_then(Value::into_bool)
    }

    pub fn get_table(
        &self,
        key: &str,
    ) -> Result<HashMap<String, Value>, ConfigError> {
        self.get(key).and_then(Value::into_table)
    }

    pub fn get_array(&self, key: &str) -> Result<Vec<Value>, ConfigError> {
        self.get(key).and_then(Value::into_array)
    }
}

use std::collections::HashMap;
use std::path::PathBuf;

pub use config::{Config, ConfigError, Environment, File, Value};
use dotenv_parser::parse_dotenv;
use serde::Deserialize;

use crate::settings::HydroSettings;
use crate::sources::FileSources;
use crate::utils::path_to_string;

type Table = HashMap<String, Value>;

#[derive(Debug, Clone)]
pub struct Hydroconf {
    config: Config,
    orig_config: Config,
    hydro_settings: HydroSettings,
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
            orig_config: Config::default(),
            hydro_settings,
        }
    }

    pub fn hydrate<'de, T: Deserialize<'de>>(
        mut self,
    ) -> Result<T, ConfigError> {
        let sources = self.discover_sources();
        if let Some(sources) = sources {
            self.load(sources)?;
        }
        self.merge()?;
        self.override_from_env()?;
        self.try_into()
    }

    pub fn discover_sources(&self) -> Option<FileSources> {
        self.root_path().map(|p| {
            FileSources::from_root(p, self.hydro_settings.env.as_str())
        })
    }

    pub fn load(
        &mut self,
        sources: FileSources,
    ) -> Result<&mut Self, ConfigError> {
        if let Some(settings_path) = sources.settings {
            self.orig_config.merge(File::from(settings_path))?;
        }
        if let Some(secrets_path) = sources.secrets {
            self.orig_config.merge(File::from(secrets_path))?;
        }

        for dotenv_path in sources.dotenv {
            let source = std::fs::read_to_string(dotenv_path.clone())
                .map_err(|e| ConfigError::FileParse {
                    uri: path_to_string(dotenv_path.clone()),
                    cause: e.into(),
                })?;
            let map =
                parse_dotenv(&source).map_err(|e| ConfigError::FileParse {
                    uri: path_to_string(dotenv_path),
                    cause: e.into(),
                })?;

            for (key, val) in map.iter() {
                self.config.set::<String>(key, val.into())?;
            }
        }

        Ok(self)
    }

    pub fn merge(&mut self) -> Result<&mut Self, ConfigError> {
        for &name in &["default", self.hydro_settings.env.as_str()] {
            let table_value: Option<Table> = self.orig_config.get(name).ok();
            if let Some(value) = table_value {
                let mut new_config = Config::default();
                new_config.cache = value.into();
                self.config.merge(new_config)?;
            }
        }

        Ok(self)
    }

    pub fn override_from_env(&mut self) -> Result<&mut Self, ConfigError> {
        self.config.merge(
            Environment::with_prefix(
                self.hydro_settings.envvar_prefix.as_str(),
            )
            .separator(self.hydro_settings.envvar_nested_sep.as_str()),
        )?;

        Ok(self)
    }

    pub fn root_path(&self) -> Option<PathBuf> {
        self.hydro_settings
            .root_path
            .clone()
            .or_else(|| std::env::current_exe().ok())
    }

    pub fn try_into<'de, T: Deserialize<'de>>(self) -> Result<T, ConfigError> {
        self.config.try_into()
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
        self.config.set_default(key, value)?;
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
        self.config.set(key, value)?;
        Ok(self)
    }

    pub fn get<'de, T>(&self, key: &'de str) -> Result<T, ConfigError>
    where
        T: Deserialize<'de>,
    {
        self.config.get(key)
    }

    pub fn get_str(&self, key: &str) -> Result<String, ConfigError> {
        self.get(key).and_then(Value::into_str)
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

use serde::Deserialize;

use config::{Config, ConfigError};
use crate::settings::HydroSettings;


#[derive(Debug, Clone)]
pub struct Hydroconf {
    config: Config,
    hydro: HydroSettings,
}


impl Default for Hydroconf {
    fn default() -> Self {
        Self::new(HydroSettings::default())
    }
}


impl Hydroconf {
    fn new(hydro: HydroSettings) -> Self {
        Self {
            config: Config::default(),
            hydro,
        }
    }

    pub fn hydrate<'de, T: Deserialize<'de>>(mut self) -> Result<T, ConfigError> {
        self.initialize();
        self.try_into()
    }

    pub fn initialize(&mut self) {

    }

    pub fn try_into<'de, T: Deserialize<'de>>(self) -> Result<T, ConfigError> {
        self.config.try_into()
    }
}

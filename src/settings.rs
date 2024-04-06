use config::{Config, File, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StagConfig {

}

impl StagConfig {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        return Config::builder().add_source(File::with_name(path)).build()?.try_deserialize()
    }

}



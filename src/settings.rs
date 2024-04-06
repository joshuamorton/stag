use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StagConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl StagConfig {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        return Config::builder()
            .add_source(File::with_name(path))
            .build()?
            .try_deserialize();
    }
}

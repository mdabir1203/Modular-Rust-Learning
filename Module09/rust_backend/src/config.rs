use serde::Deserialize;
use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
}

pub fn load_config() -> Result<Settings, ConfigError> {
    let mut s = Config::default();

    s.merge(File::with_name("config/default"))?;
    s.merge(Environment::with_prefix("APP"))?;

    s.try_into()
}


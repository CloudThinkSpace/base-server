use config::{Config, ConfigError, File};
use serde::Deserialize;

use crate::{database::DatabaseConfig, jwt::JwtConfig, log::LogConfig};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub server_name: String,
    pub database: Option<DatabaseConfig>,
    pub jwt: Option<JwtConfig>,
    pub log: Option<LogConfig>,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            // 可选：后续可加 .add_source(Environment::with_prefix("APP")) 支持环境变量
            .build()?;

        config.try_deserialize()
    }
}

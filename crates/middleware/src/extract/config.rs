use std::sync::Arc;

use axum::{extract::FromRequestParts, http::request::Parts};
use base_config::app::AppConfig;
use common::error::config::ConfigError;

pub struct Config(pub Arc<AppConfig>);
/// > 自定义提取器，提取配置文件对象
///
impl<S> FromRequestParts<S> for Config
where
    S: Send + Sync,
{
    type Rejection = ConfigError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let config = parts
            .extensions
            .get::<Arc<AppConfig>>()
            .ok_or(ConfigError::NotExistError)?;
        Ok(Config(config.clone()))
    }
}

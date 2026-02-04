use std::env;

use serde::{Deserialize, Serialize};

use crate::uri::IgnoreUri;

// JWT 配置结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
    pub ignore_uris: Option<Vec<IgnoreUri>>,
}

impl JwtConfig {
    pub fn new(secret: String, expiration_hours: i64, ignore_uris: Option<Vec<IgnoreUri>>) -> Self {
        Self {
            secret,
            expiration_hours,
            ignore_uris,
        }
    }
}

impl Default for JwtConfig {
    fn default() -> Self {
        let secret =
            env::var("JWT_SECRET").unwrap_or_else(|_| "cloud-think-space-secret".to_string());
        Self {
            secret,
            expiration_hours: 24,
            ignore_uris: None,
        }
    }
}

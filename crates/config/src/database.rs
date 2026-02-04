use std::time::Duration;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
    #[serde(default = "default_connect_timeout_secs")]
    pub connect_timeout: u64,
    #[serde(default = "default_idle_timeout_secs")]
    pub idle_timeout: Option<u64>,
    #[serde(default = "default_max_lifetime_secs")]
    pub max_lifetime: Option<u64>,
}

fn default_max_connections() -> u32 {
    10
}
fn default_min_connections() -> u32 {
    0
} // sqlx 默认是 0（无最小空闲）
fn default_connect_timeout_secs() -> u64 {
    5
}
fn default_idle_timeout_secs() -> Option<u64> {
    Some(600)
} // 10分钟
fn default_max_lifetime_secs() -> Option<u64> {
    Some(1800)
} // 30分钟

impl DatabaseConfig {
    /// 将配置转换为 Duration 类型，用于 SQLx
    pub fn connect_timeout_duration(&self) -> Duration {
        Duration::from_secs(self.connect_timeout)
    }

    pub fn idle_timeout_duration(&self) -> Option<Duration> {
        self.idle_timeout.map(Duration::from_secs)
    }

    pub fn max_lifetime_duration(&self) -> Option<Duration> {
        self.max_lifetime.map(Duration::from_secs)
    }
}

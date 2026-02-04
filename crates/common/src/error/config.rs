use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// 定义配置错误类型
#[derive(Debug)]
pub enum ConfigError {
    DatabaseError,
    RedisError,
    PathError,
    NotExistError,
}

// 为 DatabaseError 实现 IntoResponse
impl IntoResponse for ConfigError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ConfigError::DatabaseError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "database config error")
            }
            ConfigError::RedisError => (StatusCode::INTERNAL_SERVER_ERROR, "redis config error"),
            ConfigError::PathError => (StatusCode::INTERNAL_SERVER_ERROR, "config path error"),
            ConfigError::NotExistError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "config not exist error")
            }
        };

        (status, error_message).into_response()
    }
}

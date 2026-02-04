use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// 定义数据库错误类型
#[derive(Debug)]
pub enum DbError {
    ConnectError,
    QueryError,
    PoolIsNotExistError,
}

// 为 DatabaseError 实现 IntoResponse
impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DbError::ConnectError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to connect to database",
            ),
            DbError::QueryError => (StatusCode::INTERNAL_SERVER_ERROR, "Database query failed"),
            DbError::PoolIsNotExistError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "The database connection pool does not exist",
            ),
        };

        (status, error_message).into_response()
    }
}

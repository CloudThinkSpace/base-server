use axum::{Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use server_middleware::extract::db::DbPool;
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct RequestLog {
    id: String,
    path: String,
    created_at: NaiveDateTime,
}

pub fn app_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/root/{id}", get(root_id))
        .route("/aaa", get(aaa))
}

pub async fn root() -> String {
    "hello world".to_string()
}

pub async fn root_id(Path(id): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, format!("hello world: {}", id))
}

pub async fn aaa(DbPool(pool): DbPool) -> impl IntoResponse {
    let res = sqlx::query_as!(RequestLog, "SELECT * FROM request_logs where id = $1", "1")
        .fetch_one(&*pool)
        .await;
    match res {
        Ok(data) => (StatusCode::OK, format!("hello world: {:?}", data)),
        Err(_) => (StatusCode::OK, format!("hello world: {}", "error")),
    }
}

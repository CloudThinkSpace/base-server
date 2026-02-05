use axum::{Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};

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

pub async fn aaa() -> impl IntoResponse {
    (StatusCode::OK, format!("hello world: {}", "aaa"))
}

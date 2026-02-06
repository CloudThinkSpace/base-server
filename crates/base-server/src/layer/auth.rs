use std::sync::Arc;

use axum::{Extension, Router, middleware::from_fn};
use server_common::jwt::JwtService;
use server_config::app::AppConfig;
use server_middleware::middleware::auth::auth_middleware;

pub fn apply_auth_layer(app: Router, config: &AppConfig) -> Router {
    match &config.jwt {
        Some(data) => {
            let jwt_service = JwtService::new(data.clone());
            app.layer(from_fn(auth_middleware))
                .layer(Extension(Arc::new(jwt_service)))
        }
        None => app,
    }
}

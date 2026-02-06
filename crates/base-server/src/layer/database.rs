use std::sync::Arc;

use axum::{Extension, Router};
use server_config::app::AppConfig;
use server_database::connect_db;

pub async fn apply_database_layer(app: Router, config: &AppConfig) -> Router {
    match &config.database {
        Some(database_config) => {
            let pg_pool = connect_db(database_config).await.unwrap();
            // 添加数据连接池
            app.layer(Extension(Arc::new(pg_pool)))
        }
        None => app,
    }
}

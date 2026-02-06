use std::sync::Arc;

use axum::{Extension, Router};

#[cfg(feature = "auth")]
use crate::layer::auth::apply_auth_layer;
#[cfg(feature = "postgres")]
use crate::layer::database::apply_database_layer;
#[cfg(feature = "log")]
use crate::layer::log::apply_log_layer;
use server_config::app::AppConfig;
use tokio::net::TcpListener;
use tracing::info;

/// ## 启动服务
/// @param `app` 自定义router
pub async fn start(app: Router) {
    // 加载配置文件
    let config = AppConfig::from_env().unwrap();
    // 服务器名称
    let server_name = &config.server_name;
    // 服务器主机
    let server_host = &config.server_host;
    // 服务器端口
    let server_port = &config.server_port;

    // 先添加中间件，后添加Extension，中间件才能读取Extension中的内容，执行顺序正好是反向的

    #[cfg(feature = "log")]
    // 请求日志中间件
    let (app, _wg) = apply_log_layer(app, &config);

    #[cfg(feature = "auth")]
    let app = apply_auth_layer(app, &config);

    #[cfg(feature = "postgres")]
    let app = apply_database_layer(app, &config).await;

    // 添加配置文件
    let app = app.layer(Extension(Arc::new(config.clone())));

    // 打印服务器信息
    info!(
        "Starting {}: http://{}:{}",
        server_name, server_host, server_port
    );

    // 监听服务
    let listener = TcpListener::bind(format!("{}:{}", server_host, server_port))
        .await
        .unwrap();
    // 启动服务
    axum::serve(listener, app).await.unwrap();
}

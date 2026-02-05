use std::sync::Arc;

use axum::{Extension, Router};

#[cfg(any(feature = "log", feature = "auth"))]
use axum::middleware::from_fn;
#[cfg(feature = "auth")]
use server_common::jwt::JwtService;
use server_config::app::AppConfig;
#[cfg(feature = "postgres")]
use server_database::connect_db;
#[cfg(feature = "auth")]
use server_middleware::middleware::auth::auth_middleware;
#[cfg(feature = "log")]
use server_middleware::middleware::log::logging_middleware;
use tokio::net::TcpListener;
#[cfg(feature = "log")]
use trace_log::{LogLevel, init_logger};
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

    #[cfg(feature = "log")]
    let _wg = match &config.log {
        Some(data) => {
            let level = match &data.level.parse::<LogLevel>() {
                Ok(level) => level.clone(),
                Err(_) => LogLevel::Debug,
            };
            let wg = init_logger(level);
            Some(wg)
        }
        None => None,
    };

    // 先添加中间件，后添加Extension，中间件才能读取Extension中的内容，执行顺序正好是反向的

    #[cfg(feature = "auth")]
    // 添加认证中间件
    let app = match &config.jwt {
        Some(_) => app.layer(from_fn(auth_middleware)),
        None => app,
    };

    #[cfg(feature = "log")]
    // 请求日志中间件
    let app = match &config.log {
        Some(data) if data.http => app.layer(from_fn(logging_middleware)),
        _ => app,
    };

    #[cfg(feature = "auth")]
    // 添加jwt service
    let app = match &config.jwt {
        Some(data) => {
            let jwt_service = JwtService::new(data.clone());
            app.layer(Extension(Arc::new(jwt_service)))
        }
        None => app,
    };

    #[cfg(feature = "postgres")]
    let app = match &config.database {
        Some(database_config) => {
            let pg_pool = connect_db(database_config).await.unwrap();
            // 添加数据连接池
            let app = app.layer(Extension(Arc::new(pg_pool)));
            app
        }
        None => app,
    };

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

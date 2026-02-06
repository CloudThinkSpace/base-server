use axum::{Router, middleware::from_fn};
use server_config::app::AppConfig;
use server_middleware::middleware::log::logging_middleware;
use trace_log::{LogLevel, init_logger};
use tracing_appender::non_blocking::WorkerGuard;

pub fn apply_log_layer(app: Router, config: &AppConfig) -> (Router, Option<WorkerGuard>) {
    match &config.log {
        Some(data) => {
            let level = match &data.level.parse::<LogLevel>() {
                Ok(level) => level.clone(),
                Err(_) => LogLevel::Debug,
            };
            let wg = init_logger(level);
            let app = app.layer(from_fn(logging_middleware));
            (app, Some(wg))
        }
        None => {
            let app = app.layer(from_fn(logging_middleware));
            (app, None)
        }
    }
}

use server_config::database::DatabaseConfig;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn connect_db(config: &DatabaseConfig) -> anyhow::Result<PgPool> {
    let mut pool_options = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.connect_timeout_duration());

    // 设置空闲超时（可选）
    if let Some(timeout) = config.idle_timeout_duration() {
        pool_options = pool_options.idle_timeout(timeout);
    }

    // 设置连接最大生命周期（可选）
    if let Some(lifetime) = config.max_lifetime_duration() {
        pool_options = pool_options.max_lifetime(lifetime);
    }

    let pool = pool_options
        .connect(&config.url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    // 可选：测试连接
    sqlx::query_scalar!("SELECT 1")
        .fetch_one(&pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database test query failed: {}", e))?;

    Ok(pool)
}

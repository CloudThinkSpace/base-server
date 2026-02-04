use base_server::start::start;

use crate::router::app_router;

pub mod handler;
pub mod router;

#[tokio::main]
async fn main() {
    let app = app_router();
    // 启动服务器
    start(app).await;
}

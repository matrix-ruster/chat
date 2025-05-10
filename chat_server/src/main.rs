use anyhow::Result;
use chat_server::{try_init_router, AppConfig};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing::{info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, Layer};

#[tokio::main]
async fn main() -> Result<()> {
    // 加载config
    let config = AppConfig::load()?;
    // 初始化log
    let layer = fmt::Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    // 创建tcp监听
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    info!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    // 注册router
    let app = try_init_router(config).await?;
    // 开启axum服务
    if let Err(e) = axum::serve::serve(listener, app.into_make_service()).await {
        warn!("server error: {}", e);
    }
    Ok(())
}

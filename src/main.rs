use crate::domain::article::repository::ArticleRepository;
use crate::domain::article::service::ArticleDomainService;
use axum::Router;
use axum::routing::get;
use infrastructure::persistence::db;
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{filter, fmt};

mod core;
mod domain;
mod infrastructure;
mod interface;
mod shared;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化日志记录器，设置日志级别为DEBUG
    fmt().with_max_level(filter::LevelFilter::DEBUG).init();

    let db = db::connection::connect().await?;
    let article_repository: Arc<dyn ArticleRepository> = Arc::new(
        infrastructure::persistence::article::repository::SeaOrmArticleRepository::new(db),
    );
    let article_service: Arc<dyn ArticleDomainService> = Arc::new(
        domain::article::service::ArticleDomainServiceImpl::new(article_repository),
    );

    let app = Router::new()
        .merge(Router::new().route("/", get(hello)))
        .nest("/articles", interface::article::route::routes())
        // TraceLayer是一个中间件，用于记录请求和响应的详细信息
        .layer(TraceLayer::new_for_http())
        .with_state(article_service);

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    tracing::debug!("rhyon service listening on {}", listener.local_addr()?);

    // into_make_service使每个连接都会有一个独立的服务实例。可以在每个连接上应用不同的中间件或配置，从而更好地处理并发请求
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

async fn hello() -> &'static str {
    "hello world, this is rhyou"
}

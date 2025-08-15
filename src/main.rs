use axum::Router;
use axum::routing::get;
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{filter, fmt};

// 新架构的导入
use crate::adapters::inbound::http::{controllers::ArticleController, routes::article_routes};
use crate::adapters::outbound::{
    events::InMemoryEventPublisher,
    persistence::{
        database::connection::connect,
        read::SeaOrmArticleReadRepository,
        write::SeaOrmArticleWriteRepository,
    },
};
use crate::application::{
    commands::ArticleCommandHandlerImpl,
    queries::ArticleQueryHandlerImpl,
    services::ArticleApplicationServiceImpl,
};
use crate::shared::events::EventPublisher;

mod adapters;
mod application;
mod core;
mod domain;
mod shared;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化日志记录器，设置日志级别为DEBUG
    fmt().with_max_level(filter::LevelFilter::DEBUG).init();
    
    tracing::info!("🚀 启动Rhyon博客服务 - CQRS+六边形架构");

    // 1. 基础设施层 - 数据库连接
    let db = connect().await?;
    tracing::info!("✅ 数据库连接已建立");

    // 2. 出站适配器 - 仓储实现
    let write_repository = Arc::new(SeaOrmArticleWriteRepository::new(db.clone()));
    let read_repository = Arc::new(SeaOrmArticleReadRepository::new(db));
    let event_publisher: Arc<dyn EventPublisher> = Arc::new(InMemoryEventPublisher::new());
    
    tracing::info!("✅ 仓储适配器已创建");

    // 3. 应用层 - 命令和查询处理器
    let command_handler = Arc::new(ArticleCommandHandlerImpl::new(
        write_repository, 
        event_publisher
    ));
    let query_handler = Arc::new(ArticleQueryHandlerImpl::new(read_repository));
    
    tracing::info!("✅ 命令和查询处理器已创建");

    // 4. 应用服务 - 门面模式
    let application_service = Arc::new(ArticleApplicationServiceImpl::new(
        command_handler, 
        query_handler
    ));
    
    tracing::info!("✅ 应用服务已创建");

    // 5. 入站适配器 - HTTP控制器
    let article_controller = Arc::new(ArticleController::new(application_service));
    
    tracing::info!("✅ HTTP控制器已创建");

    // 6. 路由配置
    let app = Router::new()
        .route("/", get(hello))
        .nest("/articles", article_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(article_controller);

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("🌐 Rhyon服务正在监听: {}", listener.local_addr()?);
    tracing::info!("📖 API文档: http://localhost:8080/articles");

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

async fn hello() -> &'static str {
    "🎉 Rhyon博客服务 - CQRS+六边形架构已启动！"
}

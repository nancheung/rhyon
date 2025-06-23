use crate::core::error::RhyonError;
use crate::core::response::R;
use crate::domain::article::service::ArticleDomainService;
use crate::interface::article::dto::request::CreateArticleRequest;
use crate::interface::article::dto::response::{ArticleNoContentResponse, ArticleResponse};
use crate::shared::pagination::{PageRequest, PageResponse};
use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use std::sync::Arc;
use uuid::Uuid;

pub fn routes() -> Router<Arc<dyn ArticleDomainService>> {
    Router::new()
        .route("/", get(get_all))
        .route("/", post(create))
        .route("/{slug}", get(get_by_slug))
}

async fn get_all(
    State(service): State<Arc<dyn ArticleDomainService>>,
    Query(page_params): Query<PageRequest>,
) -> Result<R<PageResponse<ArticleNoContentResponse>>, RhyonError> {
    let articles = service.get_published_articles(page_params.into()).await?;
    Ok(articles.map(|articles| articles.into()).into())
}

async fn get_by_slug(
    State(service): State<Arc<dyn ArticleDomainService>>,
    Path(slug): Path<String>,
) -> Result<R<ArticleResponse>, RhyonError> {
    let article = service
        .find_article_by_slug(slug)
        .await?
        .ok_or(RhyonError::NotFound)?;
    Ok(R::success(article.into()))
}

async fn create(
    State(service): State<Arc<dyn ArticleDomainService>>,
    Json(article): Json<CreateArticleRequest>,
) -> Result<R<Uuid>, RhyonError> {
    let article = service
        .create_article(
            article.title,
            Some(article.slug),
            Some(article.summary),
            article.content,
        )
        .await?;
    if article.id.is_none() {
        return Err(RhyonError::ServerError("创建文章失败".to_string()));
    }

    todo!("发布文章不是更新，是插入，会导致重复key");
    service
        .publish_article(article.slug.value().to_string())
        .await?;
    Ok(R::success(*article.id.unwrap().value()))
}

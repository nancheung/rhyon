use crate::core::error::RhyonError;
use crate::core::page::page_params::PageParams;
use crate::core::page::page_result::PageResult;
use crate::core::response::R;
use crate::service::articles_service::{
    ArticleDTO, ArticleNoContentDTO, ArticlesService, ArticlesServiceTrait,
};
use axum::Router;
use axum::extract::{Path, Query, State};
use axum::routing::get;
use sea_orm::DatabaseConnection;

pub fn routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", get(get_all))
        .route("/{slug}", get(get_by_slug))
}

async fn get_all(
    State(db): State<DatabaseConnection>,
    Query(page_params): Query<PageParams>,
) -> Result<R<PageResult<ArticleNoContentDTO>>, RhyonError> {
    let articles = ArticlesService::page_published(&db, page_params).await?;
    Ok(articles.into())
}

async fn get_by_slug(
    State(db): State<DatabaseConnection>,
    Path(slug): Path<String>,
) -> Result<R<ArticleDTO>, RhyonError> {
    let article = ArticlesService::find_by_slug(&db, slug)
        .await?
        .ok_or(RhyonError::NotFound)?;
    Ok(R::success(article))
}

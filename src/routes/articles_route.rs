use crate::core::error::RhyonError;
use crate::core::response::R;
use crate::model::articles::Model;
use crate::service::articles_service::{
    ArticleNoContentDTO, ArticlesService, ArticlesServiceTrait,
};
use axum::Router;
use axum::extract::{Path, State};
use axum::routing::get;
use sea_orm::DatabaseConnection;

pub fn routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", get(get_all))
        .route("/{slug}", get(get_by_slug))
}

async fn get_all(
    State(db): State<DatabaseConnection>,
) -> Result<R<Vec<ArticleNoContentDTO>>, RhyonError> {
    let articles = ArticlesService::find_by_page(&db, 1, 10).await?;
    Ok(articles.into())
}

async fn get_by_slug(
    State(db): State<DatabaseConnection>,
    Path(slug): Path<String>,
) -> Result<R<Model>, RhyonError> {
    let article = ArticlesService::find_by_slug(&db, slug)
        .await?
        .ok_or(RhyonError::NotFound)?;
    Ok(R::success(article))
}

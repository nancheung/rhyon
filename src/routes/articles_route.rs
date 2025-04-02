use crate::core::error::RhyonError;
use crate::core::response::R;
use crate::model::articles::Model;
use crate::service::articles_service::{ArticlesService, ArticlesServiceTrait};
use axum::Router;
use axum::extract::{Path, State};
use axum::routing::get;
use sea_orm::DatabaseConnection;
use sea_orm::prelude::Uuid;

pub fn routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", get(get_all))
        .route("/{id}", get(get_by_id))
}

async fn get_all(State(db): State<DatabaseConnection>) -> Result<R<Vec<Model>>, RhyonError> {
    let articles = ArticlesService::find_all(&db).await?;
    Ok(articles.into())
}

async fn get_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<R<Model>, RhyonError> {
    let article = ArticlesService::find_by_id(&db, id)
        .await?
        .ok_or(RhyonError::NotFound)?;
    Ok(R::success(article))
}

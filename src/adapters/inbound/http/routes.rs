use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

use crate::adapters::inbound::http::controllers::ArticleController;

pub fn article_routes() -> Router<Arc<ArticleController>> {
    Router::new()
        .route("/", get(ArticleController::get_articles))
        .route("/", post(ArticleController::create_article))
        .route("/{slug}", get(ArticleController::get_article_by_slug))
}
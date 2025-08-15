use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::application::models::ArticleQueryModel;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ArticleListHttpResponse {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub status: String,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<ArticleQueryModel> for ArticleListHttpResponse {
    fn from(model: ArticleQueryModel) -> Self {
        Self {
            id: model.id,
            title: model.title,
            slug: model.slug,
            summary: model.summary,
            status: model.status,
            published_at: model.published_at,
            created_at: model.created_at,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ArticleDetailHttpResponse {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    pub status: String,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ArticleQueryModel> for ArticleDetailHttpResponse {
    fn from(model: ArticleQueryModel) -> Self {
        Self {
            id: model.id,
            title: model.title,
            slug: model.slug,
            summary: model.summary,
            content: model.content.unwrap_or_default(),
            status: model.status,
            published_at: model.published_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
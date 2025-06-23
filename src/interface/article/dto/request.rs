use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct CreateArticleRequest {
    pub summary: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: String,
    pub published_at: Option<DateTime<Utc>>,
}

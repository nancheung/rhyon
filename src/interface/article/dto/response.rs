use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ArticleNoContentResponse {
    pub summary: String,
    pub title: String,
    pub slug: String,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ArticleResponse {
    pub summary: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published_at: Option<DateTime<Utc>>,
}

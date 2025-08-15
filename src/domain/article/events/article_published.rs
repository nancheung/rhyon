use chrono::{DateTime, Utc};
use crate::shared::events::DomainEvent;
use uuid::Uuid;

#[derive(Debug)]
pub struct ArticlePublishedEvent {
    pub article_id: Uuid,
    pub slug: String,
    pub published_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
    pub version: u32,
}

impl ArticlePublishedEvent {
    pub fn new(article_id: Uuid, slug: String, published_at: DateTime<Utc>) -> Self {
        Self {
            article_id,
            slug,
            published_at,
            occurred_on: Utc::now(),
            version: 1,
        }
    }
}

impl DomainEvent for ArticlePublishedEvent {
    fn event_type(&self) -> &'static str {
        "article.published"
    }

    fn aggregate_id(&self) -> String {
        self.article_id.to_string()
    }

    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_on
    }

    fn version(&self) -> u32 {
        self.version
    }
}
use crate::shared::events::DomainEvent;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct ArticleCreatedEvent {
    pub article_id: Uuid,
    pub title: String,
    pub slug: String,
    pub occurred_on: DateTime<Utc>,
    pub version: u32,
}

impl ArticleCreatedEvent {
    pub fn new(article_id: Uuid, title: String, slug: String) -> Self {
        Self {
            article_id,
            title,
            slug,
            occurred_on: Utc::now(),
            version: 1,
        }
    }
}

impl DomainEvent for ArticleCreatedEvent {
    fn event_type(&self) -> &'static str {
        "article.created"
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

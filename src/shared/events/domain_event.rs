use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::fmt::Debug;

/// 领域事件trait
pub trait DomainEvent: Debug + Send + Sync {
    fn event_type(&self) -> &'static str;
    fn aggregate_id(&self) -> String;
    fn occurred_on(&self) -> DateTime<Utc>;
    fn version(&self) -> u32;
}

/// 事件发布器trait
#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(
        &self,
        event: Box<dyn DomainEvent>,
    ) -> Result<(), crate::shared::errors::RhyonError>;

    async fn publish_all(
        &self,
        events: Vec<Box<dyn DomainEvent>>,
    ) -> Result<(), crate::shared::errors::RhyonError>;
}

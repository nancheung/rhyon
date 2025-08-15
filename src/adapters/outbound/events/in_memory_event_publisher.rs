use async_trait::async_trait;
use crate::shared::errors::RhyonError;
use crate::shared::events::{DomainEvent, EventPublisher};

/// 内存事件发布器实现
pub struct InMemoryEventPublisher;

impl InMemoryEventPublisher {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventPublisher for InMemoryEventPublisher {
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<(), RhyonError> {
        tracing::info!(
            "📢 发布领域事件: {} - 聚合ID: {} - 时间: {}",
            event.event_type(),
            event.aggregate_id(),
            event.occurred_on()
        );
        Ok(())
    }

    async fn publish_all(&self, events: Vec<Box<dyn DomainEvent>>) -> Result<(), RhyonError> {
        for event in events {
            self.publish(event).await?;
        }
        Ok(())
    }
}
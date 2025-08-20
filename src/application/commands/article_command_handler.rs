use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::article::aggregate::Article;
use crate::domain::article::commands::{CreateArticleCommand, PublishArticleCommand};
use crate::domain::article::ports::article_write_repository::ArticleWriteRepository;
use crate::domain::article::value_objects::{Content, Slug, Summary, Title};
use crate::shared::errors::RhyonError;
use crate::shared::events::EventPublisher;

/// 文章命令处理器trait
#[async_trait]
pub trait ArticleCommandHandler: Send + Sync {
    async fn handle_create(&self, command: CreateArticleCommand) -> Result<Uuid, RhyonError>;
    async fn handle_publish(&self, command: PublishArticleCommand) -> Result<(), RhyonError>;
}

/// 文章命令处理器实现
pub struct ArticleCommandHandlerImpl {
    write_repository: Arc<dyn ArticleWriteRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl ArticleCommandHandlerImpl {
    pub fn new(
        write_repository: Arc<dyn ArticleWriteRepository>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            write_repository,
            event_publisher,
        }
    }
}

#[async_trait]
impl ArticleCommandHandler for ArticleCommandHandlerImpl {
    async fn handle_create(&self, command: CreateArticleCommand) -> Result<Uuid, RhyonError> {
        // 创建值对象
        let title = Title::new(command.title)?;
        let content = Content::new(command.content);

        // 处理可选摘要
        let summary = match command.summary {
            Some(summary_text) => Some(Summary::new(summary_text)?),
            None => None,
        };

        // 处理slug（如果未提供则从标题生成）
        let slug = match command.slug {
            Some(slug_str) => Slug::new(slug_str)?,
            None => Slug::from_title(title.value())?,
        };

        // 创建文章聚合根
        let mut article = Article::create(title, slug, summary, content)?;

        // 保存到仓储（会设置数据库ID）
        let id = self.write_repository.save(article.clone()).await?;
        article.set_id(id);

        // 发布领域事件
        let events = article.get_uncommitted_events();
        for event in events {
            self.event_publisher.publish(event).await?;
        }
        article.mark_events_as_committed();

        Ok(id)
    }

    async fn handle_publish(&self, command: PublishArticleCommand) -> Result<(), RhyonError> {
        // 创建slug值对象
        let slug_obj = Slug::new(command.slug)?;

        // 查找文章
        let mut article = self
            .write_repository
            .find_for_update(&slug_obj)
            .await?
            .ok_or_else(|| RhyonError::NotFound)?;

        // 执行发布操作
        article.publish()?;

        // 更新文章状态
        self.write_repository.update(article.clone()).await?;

        // 发布领域事件
        let events = article.get_uncommitted_events();
        for event in events {
            self.event_publisher.publish(event).await?;
        }

        Ok(())
    }
}

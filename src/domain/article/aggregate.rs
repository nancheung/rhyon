use crate::domain::article::events::{ArticleCreatedEvent, ArticlePublishedEvent};
use crate::domain::article::value_objects::{Content, Id, Slug, Status, Summary, Title};
use crate::shared::errors::RhyonError;
use crate::shared::events::DomainEvent;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 文章聚合根
#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    // 内部技术ID（从数据库获得）
    pub id: Option<Id>,
    // 自然键 - 业务标识符
    pub slug: Slug,
    pub title: Title,
    pub summary: Summary,
    pub content: Content,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    // 未提交的领域事件
    #[serde(skip)]
    uncommitted_events: Vec<Box<dyn DomainEvent>>,
}

// 手动实现Clone，跳过events字段
impl Clone for Article {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            slug: self.slug.clone(),
            title: self.title.clone(),
            summary: self.summary.clone(),
            content: self.content.clone(),
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
            published_at: self.published_at,
            uncommitted_events: Vec::new(), // 克隆时不包含事件
        }
    }
}

impl Article {
    /// 创建新文章（草稿状态）
    pub fn create(
        title: Title,
        slug: Slug,
        summary: Option<Summary>,
        content: Content,
    ) -> Result<Self, RhyonError> {
        let now = Utc::now();

        // 如果未提供摘要，则自动从内容生成
        let summary = match summary {
            Some(s) => s,
            None => Summary::generate_from_content(&content),
        };

        let mut article = Self {
            id: None, // 数据库插入时会分配ID
            title: title.clone(),
            slug: slug.clone(),
            summary,
            content,
            status: Status::Draft,
            created_at: now,
            updated_at: now,
            published_at: None,
            uncommitted_events: Vec::new(),
        };

        // 记录创建事件（暂时使用临时ID，保存后会更新）
        let temp_id = Uuid::new_v4();
        let event =
            ArticleCreatedEvent::new(temp_id, title.value().to_string(), slug.value().to_string());
        article.add_event(Box::new(event));

        Ok(article)
    }

    /// 从已有数据重建文章实体（通常用于从存储中加载）
    pub fn reconstitute(
        id: Option<Id>,
        slug: Slug,
        title: Title,
        summary: Summary,
        content: Content,
        status: Status,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        published_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            slug,
            title,
            summary,
            content,
            status,
            created_at,
            updated_at,
            published_at,
            uncommitted_events: Vec::new(), // 从存储重建时不包含事件
        }
    }

    /// 获取构建器用于重建文章
    pub fn builder() -> ArticleBuilder {
        ArticleBuilder::new()
    }

    /// 设置技术ID（仅在持久化层调用）
    pub fn set_id(&mut self, id: Uuid) {
        if self.id.is_none() {
            self.id = Some(Id::from(id));

            // 更新创建事件中的ID
            for event in &mut self.uncommitted_events {
                if event.event_type() == "article.created" {
                    // 这里需要重新创建事件，因为我们不能修改Box<dyn DomainEvent>
                    // 在实际实现中，可以考虑使用其他方式来处理这个问题
                }
            }
        }
    }

    /// 发布文章
    pub fn publish(&mut self) -> Result<(), RhyonError> {
        if self.content.is_empty() {
            return Err(RhyonError::Domain("无法发布空内容的文章".to_string()));
        }

        if self.status == Status::Published {
            return Err(RhyonError::Domain("文章已经是发布状态".to_string()));
        }

        self.status = Status::Published;
        let now = Utc::now();
        self.published_at = Some(now);
        self.updated_at = now;

        // 记录发布事件
        if let Some(id) = &self.id {
            let event = ArticlePublishedEvent::new(*id.value(), self.slug.value().to_string(), now);
            self.add_event(Box::new(event));
        }

        Ok(())
    }

    /// 将已发布文章设为草稿
    pub fn unpublish(&mut self) -> Result<(), RhyonError> {
        if self.status != Status::Published {
            return Err(RhyonError::Domain(
                "只有已发布的文章可以设为草稿".to_string(),
            ));
        }

        self.status = Status::Draft;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 更新文章标题
    pub fn update_title(&mut self, title: Title) {
        self.title = title;
        self.updated_at = Utc::now();
    }

    /// 更新文章内容
    pub fn update_content(&mut self, content: Content) {
        self.content = content;
        self.updated_at = Utc::now();
    }

    /// 更新文章摘要
    pub fn update_summary(&mut self, summary: Option<Summary>) {
        // 如果提供了新摘要，则使用新摘要；否则从内容自动生成
        self.summary = summary.unwrap_or_else(|| Summary::generate_from_content(&self.content));
        self.updated_at = Utc::now();
    }

    /// 更新文章slug（仅在草稿状态可以修改）
    pub fn update_slug(&mut self, slug: Slug) -> Result<(), RhyonError> {
        if self.status == Status::Published {
            return Err(RhyonError::Domain("已发布文章不能修改slug".to_string()));
        }

        self.slug = slug;
        self.updated_at = Utc::now();
        Ok(())
    }

    // 事件管理方法
    fn add_event(&mut self, event: Box<dyn DomainEvent>) {
        self.uncommitted_events.push(event);
    }

    /// 获取未提交的事件
    pub fn get_uncommitted_events(&mut self) -> Vec<Box<dyn DomainEvent>> {
        std::mem::take(&mut self.uncommitted_events)
    }

    /// 标记所有事件为已提交
    pub fn mark_events_as_committed(&mut self) {
        self.uncommitted_events.clear();
    }

    // 只读访问器
    pub fn id(&self) -> Option<&Id> {
        self.id.as_ref()
    }

    pub fn slug(&self) -> &Slug {
        &self.slug
    }

    pub fn title(&self) -> &Title {
        &self.title
    }

    pub fn summary(&self) -> &Summary {
        &self.summary
    }

    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn published_at(&self) -> Option<&DateTime<Utc>> {
        self.published_at.as_ref()
    }
}

/// 文章构建器（用于测试和复杂构建场景）
#[derive(Debug, Default)]
pub struct ArticleBuilder {
    id: Option<Id>,
    slug: Option<Slug>,
    title: Option<Title>,
    summary: Option<Summary>,
    content: Option<Content>,
    status: Option<Status>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    published_at: Option<Option<DateTime<Utc>>>,
}

impl ArticleBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: Option<Id>) -> Self {
        self.id = id;
        self
    }

    pub fn slug(mut self, slug: Slug) -> Self {
        self.slug = Some(slug);
        self
    }

    pub fn title(mut self, title: Title) -> Self {
        self.title = Some(title);
        self
    }

    pub fn summary(mut self, summary: Summary) -> Self {
        self.summary = Some(summary);
        self
    }

    pub fn content(mut self, content: Content) -> Self {
        self.content = Some(content);
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }

    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = Some(updated_at);
        self
    }

    pub fn published_at(mut self, published_at: Option<DateTime<Utc>>) -> Self {
        self.published_at = Some(published_at);
        self
    }

    pub fn build(self) -> Article {
        Article::reconstitute(
            self.id,
            self.slug.expect("slug is required"),
            self.title.expect("title is required"),
            self.summary.expect("summary is required"),
            self.content.expect("content is required"),
            self.status.expect("status is required"),
            self.created_at.expect("created_at is required"),
            self.updated_at.expect("updated_at is required"),
            self.published_at.unwrap_or(None),
        )
    }
}

use crate::core::error::RhyonError;
use crate::domain::article::value_objects::{Content, Id, Slug, Status, Summary, Title};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 文章聚合根实体
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

        Ok(Self {
            id: None, // 数据库插入时会分配ID
            title,
            slug,
            summary,
            content,
            status: Status::Draft,
            created_at: now,
            updated_at: now,
            published_at: None,
        })
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
        }
    }

    /// 设置技术ID（仅在持久化层调用）
    pub(crate) fn set_id(&mut self, id: Uuid) {
        if self.id.is_none() {
            self.id = Some(Id::from(id));
        }
    }

    /// 发布文章
    pub fn publish(&mut self) -> Result<(), RhyonError> {
        if self.content.is_empty() {
            return Err(RhyonError::Validation("无法发布空内容的文章".to_string()));
        }

        self.status = Status::Published;
        let now = Utc::now();
        self.published_at = Some(now);
        self.updated_at = now;
        Ok(())
    }

    /// 将已发布文章设为草稿
    pub fn unpublish(&mut self) -> Result<(), RhyonError> {
        if self.status != Status::Published {
            return Err(RhyonError::Validation(
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
            return Err(RhyonError::Validation("已发布文章不能修改slug".to_string()));
        }

        self.slug = slug;
        self.updated_at = Utc::now();
        Ok(())
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

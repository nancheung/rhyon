use chrono::{DateTime, Utc};
use serde::Serialize;

/// 文章查询模型（专门用于读操作优化）
#[derive(Debug, Clone, Serialize)]
pub struct ArticleQueryModel {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: Option<String>, // 列表查询时为None，详情查询时有值
    pub status: String,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ArticleQueryModel {
    /// 使用构建器模式创建新实例
    pub fn builder() -> ArticleQueryModelBuilder {
        ArticleQueryModelBuilder::new()
    }

    /// 创建列表视图模型（不包含内容）
    pub fn for_list(
        id: String,
        title: String,
        slug: String,
        summary: String,
        status: String,
        published_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self::builder()
            .id(id)
            .title(title)
            .slug(slug)
            .summary(summary)
            .status(status)
            .published_at(published_at)
            .created_at(created_at)
            .updated_at(updated_at)
            .build()
    }

    /// 创建详情视图模型（包含完整内容）
    pub fn for_detail(
        id: String,
        title: String,
        slug: String,
        summary: String,
        content: String,
        status: String,
        published_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self::builder()
            .id(id)
            .title(title)
            .slug(slug)
            .summary(summary)
            .content(content)
            .status(status)
            .published_at(published_at)
            .created_at(created_at)
            .updated_at(updated_at)
            .build()
    }
}

/// 文章查询模型构建器
#[derive(Debug, Default)]
pub struct ArticleQueryModelBuilder {
    id: Option<String>,
    title: Option<String>,
    slug: Option<String>,
    summary: Option<String>,
    content: Option<String>,
    status: Option<String>,
    published_at: Option<Option<DateTime<Utc>>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl ArticleQueryModelBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn slug(mut self, slug: String) -> Self {
        self.slug = Some(slug);
        self
    }

    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    pub fn published_at(mut self, published_at: Option<DateTime<Utc>>) -> Self {
        self.published_at = Some(published_at);
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

    pub fn build(self) -> ArticleQueryModel {
        ArticleQueryModel {
            id: self.id.expect("id is required"),
            title: self.title.expect("title is required"),
            slug: self.slug.expect("slug is required"),
            summary: self.summary.expect("summary is required"),
            content: self.content,
            status: self.status.expect("status is required"),
            published_at: self.published_at.unwrap_or(None),
            created_at: self.created_at.expect("created_at is required"),
            updated_at: self.updated_at.expect("updated_at is required"),
        }
    }
}

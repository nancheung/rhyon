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
    pub fn new(
        id: String,
        title: String,
        slug: String,
        summary: String,
        content: Option<String>,
        status: String,
        published_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            title,
            slug,
            summary,
            content,
            status,
            published_at,
            created_at,
            updated_at,
        }
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
        Self::new(
            id,
            title,
            slug,
            summary,
            None, // 列表查询不包含内容
            status,
            published_at,
            created_at,
            updated_at,
        )
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
        Self::new(
            id,
            title,
            slug,
            summary,
            Some(content), // 详情查询包含内容
            status,
            published_at,
            created_at,
            updated_at,
        )
    }
}
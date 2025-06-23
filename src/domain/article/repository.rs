use crate::core::error::RhyonError;
use crate::domain::article::entity::Article;
use crate::domain::article::value_objects::{Slug, Status};
use crate::shared::pagination::{PageResponse, PaginationQuery};
use async_trait::async_trait;
use uuid::Uuid;

/// 文章仓储接口
#[async_trait]
pub trait ArticleRepository: Send + Sync {
    /// 通过Slug查找文章（自然键）
    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<Article>, RhyonError>;

    /// 保存文章（创建或更新）
    /// 返回数据库分配的UUID
    async fn save(&self, article: Article) -> Result<Uuid, RhyonError>;

    /// 分页查找指定状态的文章
    async fn find_by_status(
        &self,
        status: Status,
        page_request: PaginationQuery,
    ) -> Result<PageResponse<Article>, RhyonError>;
}

use crate::application::models::ArticleQueryModel;
use crate::domain::article::specifications::{ArticleSortSpec, ArticleSpec};
use crate::shared::errors::RhyonError;
use crate::shared::pagination::{QueryPage, QueryPagination};
use async_trait::async_trait;

/// 文章读仓储端口
#[async_trait]
pub trait ArticleReadRepository: Send + Sync {
    /// 根据规约查找文章（支持复杂查询和分页）
    async fn find_by_specification(
        &self,
        specification: ArticleSpec,
        sort: ArticleSortSpec,
        pagination: QueryPagination,
    ) -> Result<QueryPage<ArticleQueryModel>, RhyonError>;

    /// 简单的slug查询（无需分页）
    async fn find_by_slug(&self, slug: &str) -> Result<Option<ArticleQueryModel>, RhyonError>;
}

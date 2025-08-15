use async_trait::async_trait;
use crate::application::models::ArticleQueryModel;
use crate::application::queries::ArticleQuerySpec;
use crate::shared::errors::RhyonError;
use crate::shared::pagination::QueryPage;

/// 文章读仓储端口
#[async_trait]
pub trait ArticleReadRepository: Send + Sync {
    /// 根据查询规约查找文章（支持分页）
    async fn find_by_spec(
        &self, 
        spec: ArticleQuerySpec
    ) -> Result<QueryPage<ArticleQueryModel>, RhyonError>;
    
    /// 简单的slug查询（无需分页）
    async fn find_by_slug(&self, slug: &str) -> Result<Option<ArticleQueryModel>, RhyonError>;
}
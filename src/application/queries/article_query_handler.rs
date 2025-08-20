use async_trait::async_trait;
use std::sync::Arc;

use crate::application::models::ArticleQueryModel;
use crate::application::ports::ArticleReadRepository;
use crate::application::queries::{GetArticleBySlugQuery, GetArticlesQuery};
use crate::shared::errors::RhyonError;
use crate::shared::pagination::QueryPage;

/// 文章查询处理器trait
#[async_trait]
pub trait ArticleQueryHandler: Send + Sync {
    async fn handle_get_articles(
        &self,
        query: GetArticlesQuery,
    ) -> Result<QueryPage<ArticleQueryModel>, RhyonError>;

    async fn handle_get_by_slug(
        &self,
        query: GetArticleBySlugQuery,
    ) -> Result<Option<ArticleQueryModel>, RhyonError>;
}

/// 文章查询处理器实现
pub struct ArticleQueryHandlerImpl {
    read_repository: Arc<dyn ArticleReadRepository>,
}

impl ArticleQueryHandlerImpl {
    pub fn new(read_repository: Arc<dyn ArticleReadRepository>) -> Self {
        Self { read_repository }
    }
}

#[async_trait]
impl ArticleQueryHandler for ArticleQueryHandlerImpl {
    async fn handle_get_articles(
        &self,
        query: GetArticlesQuery,
    ) -> Result<QueryPage<ArticleQueryModel>, RhyonError> {
        // 使用新的规约系统执行查询
        self.read_repository
            .find_by_specification(query.specification, query.sort, query.pagination)
            .await
    }

    async fn handle_get_by_slug(
        &self,
        query: GetArticleBySlugQuery,
    ) -> Result<Option<ArticleQueryModel>, RhyonError> {
        self.read_repository.find_by_slug(&query.slug).await
    }
}

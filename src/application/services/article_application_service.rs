use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::commands::ArticleCommandHandler;
use crate::application::models::ArticleQueryModel;
use crate::application::queries::{ArticleQueryHandler, GetArticleBySlugQuery, GetArticlesQuery};
use crate::domain::article::commands::{CreateArticleCommand, PublishArticleCommand};
use crate::shared::errors::RhyonError;
use crate::shared::pagination::QueryPage;

/// 文章应用服务（门面模式）
#[async_trait]
pub trait ArticleApplicationService: Send + Sync {
    // 命令操作
    async fn create_article(&self, command: CreateArticleCommand) -> Result<Uuid, RhyonError>;
    async fn publish_article(&self, command: PublishArticleCommand) -> Result<(), RhyonError>;

    // 查询操作
    async fn get_articles(
        &self,
        query: GetArticlesQuery,
    ) -> Result<QueryPage<ArticleQueryModel>, RhyonError>;
    async fn get_article_by_slug(
        &self,
        query: GetArticleBySlugQuery,
    ) -> Result<Option<ArticleQueryModel>, RhyonError>;
}

/// 文章应用服务实现
pub struct ArticleApplicationServiceImpl {
    command_handler: Arc<dyn ArticleCommandHandler>,
    query_handler: Arc<dyn ArticleQueryHandler>,
}

impl ArticleApplicationServiceImpl {
    pub fn new(
        command_handler: Arc<dyn ArticleCommandHandler>,
        query_handler: Arc<dyn ArticleQueryHandler>,
    ) -> Self {
        Self {
            command_handler,
            query_handler,
        }
    }
}

#[async_trait]
impl ArticleApplicationService for ArticleApplicationServiceImpl {
    async fn create_article(&self, command: CreateArticleCommand) -> Result<Uuid, RhyonError> {
        self.command_handler.handle_create(command).await
    }

    async fn publish_article(&self, command: PublishArticleCommand) -> Result<(), RhyonError> {
        self.command_handler.handle_publish(command).await
    }

    async fn get_articles(
        &self,
        query: GetArticlesQuery,
    ) -> Result<QueryPage<ArticleQueryModel>, RhyonError> {
        self.query_handler.handle_get_articles(query).await
    }

    async fn get_article_by_slug(
        &self,
        query: GetArticleBySlugQuery,
    ) -> Result<Option<ArticleQueryModel>, RhyonError> {
        self.query_handler.handle_get_by_slug(query).await
    }
}

use crate::core::error::RhyonError;
use crate::domain::article::entity::Article;
use crate::domain::article::repository::ArticleRepository;
use crate::domain::article::value_objects::{Content, Slug, Status, Summary, Title};
use crate::shared::pagination::{PageRequest, PageResponse};
use async_trait::async_trait;
use std::sync::Arc;

/// 文章领域服务接口
#[async_trait]
pub trait ArticleDomainService: Send + Sync {
    /// 创建新文章
    async fn create_article(
        &self,
        title: String,
        slug: Option<String>,    // 可选，如未提供则从标题生成
        summary: Option<String>, // 可选，如未提供则从内容生成
        content: String,
    ) -> Result<Article, RhyonError>;

    /// 发布文章
    async fn publish_article(&self, slug: String) -> Result<Article, RhyonError>;

    /// 通过Slug查找文章
    async fn find_article_by_slug(&self, slug: String) -> Result<Option<Article>, RhyonError>;

    /// 分页获取已发布文章
    async fn get_published_articles(
        &self,
        page_request: PageRequest,
    ) -> Result<PageResponse<Article>, RhyonError>;
}

/// 文章领域服务实现
pub struct ArticleDomainServiceImpl {
    repository: Arc<dyn ArticleRepository>,
}

impl ArticleDomainServiceImpl {
    pub fn new(repository: Arc<dyn ArticleRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ArticleDomainService for ArticleDomainServiceImpl {
    async fn create_article(
        &self,
        title: String,
        slug: Option<String>,
        summary: Option<String>,
        content: String,
    ) -> Result<Article, RhyonError> {
        // 创建值对象
        let title = Title::new(title)?;
        let content = Content::new(content);

        // 处理可选摘要
        let summary = match summary {
            Some(summary_text) => Some(Summary::new(summary_text)?),
            None => None,
        };

        // 处理slug（如果未提供则从标题生成）
        let slug = match slug {
            Some(slug_str) => Slug::new(slug_str),
            None => Slug::from_title(title.value()),
        }?;

        // 创建文章实体
        let mut article = Article::create(title, slug, summary, content)?;

        // 保存到仓储（会设置数据库ID）
        let id = self.repository.save(article.clone()).await?;
        article.set_id(id);

        Ok(article)
    }

    async fn publish_article(&self, slug: String) -> Result<Article, RhyonError> {
        // 创建slug值对象
        let slug_obj = Slug::new(slug)?;

        // 查找文章
        let mut article = self
            .repository
            .find_by_slug(&slug_obj)
            .await?
            .ok_or_else(|| RhyonError::Validation("文章不存在".to_string()))?;

        // 执行发布操作
        article.publish()?;

        // 保存更改
        self.repository.save(article.clone()).await?;

        Ok(article)
    }

    async fn find_article_by_slug(&self, slug: String) -> Result<Option<Article>, RhyonError> {
        let slug_obj = Slug::new(slug)?;
        self.repository.find_by_slug(&slug_obj).await
    }

    async fn get_published_articles(
        &self,
        page_request: PageRequest,
    ) -> Result<PageResponse<Article>, RhyonError> {
        let articles = self
            .repository
            .find_by_status(Status::Published, page_request.into())
            .await?;

        Ok(articles)
    }
}

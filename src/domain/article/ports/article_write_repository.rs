use async_trait::async_trait;
use crate::domain::article::aggregate::Article;
use crate::domain::article::value_objects::Slug;
use crate::shared::errors::RhyonError;
use uuid::Uuid;

/// 文章写仓储端口
#[async_trait]
pub trait ArticleWriteRepository: Send + Sync {
    /// 保存新文章
    async fn save(&self, article: Article) -> Result<Uuid, RhyonError>;
    
    /// 更新已存在的文章
    async fn update(&self, article: Article) -> Result<(), RhyonError>;
    
    /// 通过Slug查找文章（用于更新操作）
    async fn find_for_update(&self, slug: &Slug) -> Result<Option<Article>, RhyonError>;
}
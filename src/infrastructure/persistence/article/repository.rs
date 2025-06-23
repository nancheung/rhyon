use crate::core::error::RhyonError;
use crate::domain::article::entity::Article;
use crate::domain::article::repository::ArticleRepository;
use crate::domain::article::value_objects::{Slug, Status};
use crate::infrastructure::persistence::article::entity::{ActiveModel, Column, Entity};
use crate::shared::pagination::{PageResponse, PaginationQuery};
use async_trait::async_trait;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, InsertResult, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use uuid::Uuid;

pub struct SeaOrmArticleRepository {
    db: DatabaseConnection,
}

impl SeaOrmArticleRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ArticleRepository for SeaOrmArticleRepository {
    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<Article>, RhyonError> {
        let article = Entity::find()
            .filter(Column::Slug.eq(slug.value()))
            .one(&self.db)
            .await?
            .map(|model| {
                let a: Result<Article, RhyonError> = model.into();
                a
            })
            .transpose()?;

        Ok(article)
    }

    async fn save(&self, article: Article) -> Result<Uuid, RhyonError> {
        let result: InsertResult<ActiveModel> =
            Entity::insert(article.into()).exec(&self.db).await?;

        Ok(result.last_insert_id)
    }

    async fn find_by_status(
        &self,
        status: Status,
        page_request: PaginationQuery,
    ) -> Result<PageResponse<Article>, RhyonError> {
        let paginator = Entity::find()
            .filter(Column::Status.eq(status.as_str()))
            .order_by_desc(Column::PublishedAt)
            .paginate(&self.db, page_request.size);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page_request.page - 1).await?;

        let articles: Vec<Article> = items
            .into_iter()
            .map(|model| {
                let slug = model.slug.clone();
                let article: Result<Article, RhyonError> = model.into();
                article.map_err(|e| {
                    RhyonError::ServerError(format!("获取文章《{}》失败：{:?}", slug, e))
                })
            })
            .collect::<Result<Vec<_>, RhyonError>>()?;

        Ok(PageResponse::new(
            articles,
            page_request.page,
            page_request.size,
            total,
        ))
    }
}

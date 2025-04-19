use crate::core::page::page_params::PageParams;
use crate::core::page::page_result::PageResult;
use crate::model::articles::{ActiveModel, Column, Entity};
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::prelude::{DateTimeWithTimeZone, Uuid};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, DerivePartialModel, EntityTrait,
    FromQueryResult, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait ArticlesServiceTrait {
    /// 分页查询文章列表
    async fn page_published(
        db: &DatabaseConnection,
        page_params: PageParams,
    ) -> Result<PageResult<ArticleNoContentDTO>, DbErr>;
    /// 通过slug查询文章
    async fn find_by_slug(
        db: &DatabaseConnection,
        slug: String,
    ) -> Result<Option<ArticleDTO>, DbErr>;
    /// 创建文章
    async fn create(db: &DatabaseConnection, article: CreateArticleVO) -> Result<Uuid, DbErr>;
}

pub struct ArticlesService;

#[async_trait]
impl ArticlesServiceTrait for ArticlesService {
    async fn page_published(
        db: &DatabaseConnection,
        page_params: PageParams,
    ) -> Result<PageResult<ArticleNoContentDTO>, DbErr> {
        let paginator = Entity::find()
            .filter(Column::Status.eq("published"))
            .order_by_desc(Column::PublishedAt)
            .into_partial_model::<ArticleNoContentDTO>()
            .paginate(db, page_params.page_size());

        let total = paginator.num_items().await?;
        let result = paginator.fetch_page(page_params.page() - 1).await?;

        Ok(PageResult::new(page_params, total, result))
    }

    async fn find_by_slug(
        db: &DatabaseConnection,
        slug: String,
    ) -> Result<Option<ArticleDTO>, DbErr> {
        Entity::find()
            .filter(Column::Slug.eq(slug))
            .into_partial_model::<ArticleDTO>()
            .one(db)
            .await
    }

    async fn create(db: &DatabaseConnection, article: CreateArticleVO) -> Result<Uuid, DbErr> {
        let article_insert = Entity::insert(article.to_active_model()).exec(db).await?;

        Ok(article_insert.last_insert_id)
    }
}

#[derive(DerivePartialModel, FromQueryResult, Serialize)]
#[sea_orm(entity = "Entity")]
#[serde(rename_all = "camelCase")]
pub struct ArticleNoContentDTO {
    pub summary: String,
    pub title: String,
    pub slug: String,
    pub published_at: Option<DateTimeWithTimeZone>,
}

#[derive(DerivePartialModel, FromQueryResult, Serialize)]
#[sea_orm(entity = "Entity")]
#[serde(rename_all = "camelCase")]
pub struct ArticleDTO {
    pub summary: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published_at: Option<DateTimeWithTimeZone>,
}

#[derive(DerivePartialModel, FromQueryResult, Deserialize)]
#[sea_orm(entity = "Entity")]
#[serde(rename_all = "camelCase")]
pub struct CreateArticleVO {
    pub summary: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: String,
    pub published_at: Option<DateTimeWithTimeZone>,
}

trait CreateArticleVOTrait {
    fn to_active_model(self) -> ActiveModel;
}
impl CreateArticleVOTrait for CreateArticleVO {
    fn to_active_model(self) -> ActiveModel {
        ActiveModel {
            summary: Set(self.summary),
            title: Set(self.title),
            slug: Set(self.slug),
            content: Set(self.content),
            status: Set(self.status),
            published_at: Set(self.published_at),
            ..Default::default()
        }
    }
}


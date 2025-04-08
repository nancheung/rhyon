use crate::core::page::page_params::PageParams;
use crate::core::page::page_result::PageResult;
use crate::model::articles::{Column, Entity};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, DerivePartialModel, EntityTrait, FromQueryResult,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use serde::Serialize;

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

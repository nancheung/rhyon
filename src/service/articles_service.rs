use crate::model::articles::{Column, Entity, Model};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, DerivePartialModel, EntityTrait, FromQueryResult,
    QueryFilter, QueryOrder, QuerySelect,
};
use serde::Serialize;

#[async_trait]
pub trait ArticlesServiceTrait {
    /// 分页查询文章列表
    async fn find_by_page(
        db: &DatabaseConnection,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<ArticleNoContentDTO>, DbErr>;
    /// 通过slug查询文章
    async fn find_by_slug(db: &DatabaseConnection, slug: String) -> Result<Option<Model>, DbErr>;
}

pub struct ArticlesService;

#[async_trait]
impl ArticlesServiceTrait for ArticlesService {
    async fn find_by_page(
        db: &DatabaseConnection,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<ArticleNoContentDTO>, DbErr> {
        let offset = (page - 1) * page_size;
        Entity::find()
            .order_by_desc(Column::PublishedAt)
            .limit(page_size)
            .offset(offset)
            .into_partial_model::<ArticleNoContentDTO>()
            .all(db)
            .await
    }

    async fn find_by_slug(db: &DatabaseConnection, slug: String) -> Result<Option<Model>, DbErr> {
        Entity::find().filter(Column::Slug.eq(slug)).one(db).await
    }
}

#[derive(DerivePartialModel, FromQueryResult, Serialize)]
#[sea_orm(entity = "Entity")]
pub struct ArticleNoContentDTO {
    pub summary: String,
    pub title: String,
    pub slug: String,
    pub published_at: Option<DateTimeWithTimeZone>,
}

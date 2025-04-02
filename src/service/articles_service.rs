use crate::model::articles::{Entity, Model};
use sea_orm::prelude::Uuid;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QuerySelect};

#[async_trait]
pub trait ArticlesServiceTrait {
    async fn find_all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr>;
    async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<Model>, DbErr>;
}

pub struct ArticlesService;

#[async_trait]
impl ArticlesServiceTrait for ArticlesService {
    async fn find_all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(db).await
    }

    async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }
}

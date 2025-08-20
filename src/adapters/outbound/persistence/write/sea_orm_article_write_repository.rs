use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, InsertResult, QueryFilter, Set,
};
use uuid::Uuid;

use crate::adapters::outbound::persistence::entities::{ActiveModel, Column, Entity};
use crate::domain::article::aggregate::Article;
use crate::domain::article::ports::article_write_repository::ArticleWriteRepository;
use crate::domain::article::value_objects::Slug;
use crate::shared::errors::RhyonError;

pub struct SeaOrmArticleWriteRepository {
    db: DatabaseConnection,
}

impl SeaOrmArticleWriteRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ArticleWriteRepository for SeaOrmArticleWriteRepository {
    async fn save(&self, article: Article) -> Result<Uuid, RhyonError> {
        let active_model: ActiveModel = article.into();
        let result: InsertResult<ActiveModel> = Entity::insert(active_model).exec(&self.db).await?;
        Ok(result.last_insert_id)
    }

    async fn update(&self, article: Article) -> Result<(), RhyonError> {
        if article.id().is_none() {
            return Err(RhyonError::Domain("无法更新没有ID的文章".to_string()));
        }

        let id = *article
            .id()
            .ok_or_else(|| RhyonError::Domain("无法更新没有ID的文章".to_string()))?
            .value();
        let mut active_model: ActiveModel = article.into();
        active_model.id = Set(id);

        active_model.update(&self.db).await?;
        Ok(())
    }

    async fn find_for_update(&self, slug: &Slug) -> Result<Option<Article>, RhyonError> {
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
}

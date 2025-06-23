use crate::core::error::RhyonError;
use crate::core::types::conversions::Converter;
use crate::domain::article::entity::Article;
use crate::domain::article::value_objects::*;
use crate::infrastructure::persistence::article::entity::{ActiveModel, Model};
use sea_orm::Set;

impl From<Article> for Model {
    fn from(article: Article) -> Self {
        Model {
            id: article.id().map(|id| *id.value()).unwrap(),
            slug: article.slug().value().to_string(),
            title: article.title().value().to_string(),
            summary: article.summary().value().to_string(),
            content: article.content().value().to_string(),
            status: article.status().as_str().to_string(),
            created_at: article.created_at().convert(),
            updated_at: article.updated_at().convert(),
            published_at: article.published_at().map(|dt| dt.convert()),
        }
    }
}

impl From<Article> for ActiveModel {
    fn from(article: Article) -> Self {
        ActiveModel {
            summary: Set(article.summary.value().to_string()),
            title: Set(article.title.value().to_string()),
            slug: Set(article.slug.value().to_string()),
            content: Set(article.content.value().to_string()),
            status: Set(article.status.as_str().to_string()),
            published_at: Set(article.published_at.map(|dt| dt.convert())),
            ..Default::default()
        }
    }
}

impl From<Model> for Result<Article, RhyonError> {
    fn from(model: Model) -> Self {
        // 创建值对象
        let id = Some(Id::from(model.id));
        let title = Title::new(model.title)?;
        let slug = Slug::new(model.slug)?;
        let summary = Summary::new(model.summary)?;
        let content = Content::new(model.content);
        let status = Status::from_str(&model.status)?;

        // 重构文章实体
        Ok(Article::reconstitute(
            id,
            slug,
            title,
            summary,
            content,
            status,
            model.created_at.convert(),
            model.updated_at.convert(),
            model.published_at.map(|dt| dt.convert()),
        ))
    }
}

use crate::domain::article::entity::Article;
use crate::interface::article::dto::response::{ArticleNoContentResponse, ArticleResponse};
impl From<Article> for ArticleNoContentResponse {
    fn from(article: Article) -> Self {
        Self {
            slug: article.slug.to_string(),
            title: article.title.to_string(),
            summary: article.summary.to_string(),
            published_at: article.published_at,
        }
    }
}

impl From<Article> for ArticleResponse {
    fn from(article: Article) -> Self {
        Self {
            slug: article.slug.value().to_string(),
            title: article.title.value().to_string(),
            summary: article.summary.value().to_string(),
            content: article.content.value().to_string(),
            published_at: article.published_at,
        }
    }
}

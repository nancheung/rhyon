use crate::domain::article::commands::{CreateArticleCommand, PublishArticleCommand};
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct CreateArticleHttpRequest {
    pub summary: Option<String>,
    pub title: String,
    pub slug: Option<String>,
    pub content: String,
}

impl From<CreateArticleHttpRequest> for CreateArticleCommand {
    fn from(req: CreateArticleHttpRequest) -> Self {
        CreateArticleCommand {
            title: req.title,
            slug: req.slug,
            summary: req.summary,
            content: req.content,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct PublishArticleHttpRequest {
    pub slug: String,
}

impl From<PublishArticleHttpRequest> for PublishArticleCommand {
    fn from(req: PublishArticleHttpRequest) -> Self {
        PublishArticleCommand { slug: req.slug }
    }
}

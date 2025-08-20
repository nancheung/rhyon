#[derive(Debug, Clone)]
pub struct PublishArticleCommand {
    pub slug: String,
}

impl PublishArticleCommand {
    pub fn new(slug: String) -> Self {
        Self { slug }
    }
}

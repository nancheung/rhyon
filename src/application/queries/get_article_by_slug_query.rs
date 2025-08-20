#[derive(Debug)]
pub struct GetArticleBySlugQuery {
    pub slug: String,
}

impl GetArticleBySlugQuery {
    pub fn new(slug: String) -> Self {
        Self { slug }
    }
}

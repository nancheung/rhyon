#[derive(Debug, Clone)]
pub struct CreateArticleCommand {
    pub title: String,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub content: String,
}

impl CreateArticleCommand {
    pub fn new(title: String, content: String) -> Self {
        Self {
            title,
            slug: None,
            summary: None,
            content,
        }
    }

    pub fn with_slug(mut self, slug: String) -> Self {
        self.slug = Some(slug);
        self
    }

    pub fn with_summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }
}

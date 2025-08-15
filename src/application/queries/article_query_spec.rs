use crate::shared::pagination::QueryPagination;

#[derive(Debug)]
pub struct ArticleQuerySpec {
    pub status: Option<String>,
    pub pagination: QueryPagination,
}

impl ArticleQuerySpec {
    pub fn new() -> Self {
        Self {
            status: None,
            pagination: QueryPagination::default(),
        }
    }

    pub fn with_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_pagination(mut self, pagination: QueryPagination) -> Self {
        self.pagination = pagination;
        self
    }
}

impl Default for ArticleQuerySpec {
    fn default() -> Self {
        Self::new()
    }
}
use crate::shared::pagination::QueryPagination;

#[derive(Debug)]
pub struct GetArticlesQuery {
    pub status_filter: Option<String>,
    pub pagination: QueryPagination,
}

impl GetArticlesQuery {
    pub fn new(pagination: QueryPagination) -> Self {
        Self {
            status_filter: Some("published".to_string()), // 默认只查询已发布的文章
            pagination,
        }
    }

    pub fn with_status(mut self, status: Option<String>) -> Self {
        self.status_filter = status;
        self
    }
}
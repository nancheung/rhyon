use crate::domain::article::specifications::{ArticleSortSpec, ArticleSpec};
use crate::shared::pagination::QueryPagination;

#[derive(Debug)]
pub struct GetArticlesQuery {
    pub specification: ArticleSpec,
    pub sort: ArticleSortSpec,
    pub pagination: QueryPagination,
}

impl GetArticlesQuery {
    pub fn new(pagination: QueryPagination) -> Self {
        Self {
            specification: ArticleSpec::published(), // 默认只查询已发布的文章
            sort: ArticleSortSpec::default(),
            pagination,
        }
    }

    pub fn with_specification(mut self, spec: ArticleSpec) -> Self {
        self.specification = spec;
        self
    }

    pub fn with_sort(mut self, sort: ArticleSortSpec) -> Self {
        self.sort = sort;
        self
    }

    pub fn with_status(self, status: Option<String>) -> Self {
        let spec = match status.as_deref() {
            Some("published") => ArticleSpec::published(),
            Some("draft") => ArticleSpec::draft(),
            _ => ArticleSpec::published(), // 默认已发布
        };
        self.with_specification(spec)
    }
}

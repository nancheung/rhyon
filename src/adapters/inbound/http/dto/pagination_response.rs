use serde::Serialize;
use crate::shared::pagination::QueryPage;

#[derive(Debug, Serialize)]
pub struct HttpPaginationResponse<T> {
    pub data: Vec<T>,
    pub pagination: HttpPaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct HttpPaginationMeta {
    pub current_page: u64,
    pub page_size: u64,
    pub total_items: u64,
    pub total_pages: u64,
}

impl<T> From<QueryPage<T>> for HttpPaginationResponse<T> {
    fn from(page: QueryPage<T>) -> Self {
        HttpPaginationResponse {
            data: page.items,
            pagination: HttpPaginationMeta {
                current_page: page.current_page,
                page_size: page.page_size,
                total_items: page.total_items,
                total_pages: page.total_pages,
            },
        }
    }
}
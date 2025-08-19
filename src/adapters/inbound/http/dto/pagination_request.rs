use serde::Deserialize;
use crate::shared::pagination::QueryPagination;

#[derive(Debug, Deserialize)]
pub struct HttpPaginationRequest {
    #[serde(default = "default_page")]
    pub page: u64,
    
    #[serde(default = "default_size")]  
    pub size: u64,
    
    pub sort: Option<String>, // "published_at:desc,title:asc" - 将在应用层处理
}

impl HttpPaginationRequest {
    pub fn into_pagination(self) -> QueryPagination {
        QueryPagination::new(self.page, self.size)
    }
    
    pub fn sort_string(&self) -> Option<String> {
        self.sort.clone()
    }
}

impl From<HttpPaginationRequest> for QueryPagination {
    fn from(req: HttpPaginationRequest) -> Self {
        req.into_pagination()
    }
}

impl Default for HttpPaginationRequest {
    fn default() -> Self {
        Self {
            page: 1,
            size: 10,
            sort: Some("published_at:desc".to_string()),
        }
    }
}

fn default_page() -> u64 {
    1
}

fn default_size() -> u64 {
    10
}
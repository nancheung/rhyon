use serde::Deserialize;
use crate::shared::pagination::{QueryPagination, SortCriteria};

#[derive(Debug, Deserialize)]
pub struct HttpPaginationRequest {
    #[serde(default = "default_page")]
    pub page: u64,
    
    #[serde(default = "default_size")]  
    pub size: u64,
    
    pub sort: Option<String>, // "published_at:desc,title:asc"
}

impl From<HttpPaginationRequest> for QueryPagination {
    fn from(req: HttpPaginationRequest) -> Self {
        QueryPagination::new(req.page, req.size, req.sort.map(|s| s.into()))
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
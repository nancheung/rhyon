use crate::core::page::page_params::PageParams;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T> {
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
    pub items: Vec<T>,
}

impl<T> PageResult<T> {
    pub fn new(page_params: PageParams, total: u64, items: Vec<T>) -> Self {
        Self {
            total,
            page: page_params.page(),
            page_size: page_params.page_size(),
            items,
        }
    }
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageParams {
    page: Option<u64>,
    page_size: Option<u64>,
}

impl PageParams {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1)
    }

    pub fn page_size(&self) -> u64 {
        self.page_size.unwrap_or(10)
    }

    pub fn offset(&self) -> u64 {
        (self.page() - 1) * self.page_size()
    }
}

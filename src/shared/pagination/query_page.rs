use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct QueryPage<T> {
    pub items: Vec<T>,
    pub current_page: u64,
    pub page_size: u64,
    pub total_items: u64,
    pub total_pages: u64,
}

impl<T> QueryPage<T> {
    pub fn new(items: Vec<T>, current_page: u64, page_size: u64, total_items: u64) -> Self {
        let total_pages = if total_items == 0 {
            0
        } else {
            (total_items + page_size - 1) / page_size
        };

        Self {
            items,
            current_page,
            page_size,
            total_items,
            total_pages,
        }
    }

    pub fn empty(current_page: u64, page_size: u64) -> Self {
        Self::new(Vec::new(), current_page, page_size, 0)
    }

    /// 转换项目类型
    pub fn map<U, F>(self, f: F) -> QueryPage<U>
    where
        F: FnMut(T) -> U,
    {
        QueryPage {
            items: self.items.into_iter().map(f).collect(),
            current_page: self.current_page,
            page_size: self.page_size,
            total_items: self.total_items,
            total_pages: self.total_pages,
        }
    }
}
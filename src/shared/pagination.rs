use serde::{Deserialize, Serialize};

/// 分页请求参数
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct PageRequest {
    page: u64, // 当前页码（从1开始）
    size: u64, // 每页大小
}

impl PageRequest {
    /// 创建新的分页请求
    pub fn new(page: u64, size: u64) -> Self {
        // 确保页码和大小都有合理的默认值
        let page = if page == 0 { 1 } else { page };
        let size = if size == 0 { 10 } else { size.min(100) };

        Self { page, size }
    }

    /// 获取当前页码
    pub fn page(&self) -> u64 {
        self.page
    }

    /// 获取每页大小
    pub fn size(&self) -> u64 {
        self.size
    }

    /// 计算偏移量
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.size
    }
}

impl Default for PageRequest {
    fn default() -> Self {
        Self::new(1, 10)
    }
}

/// 分页结果
#[derive(Debug, Serialize)]
pub struct PageResponse<T> {
    /// 分页数据项
    pub items: Vec<T>,
    /// 当前页码
    pub page: u64,
    /// 每页大小
    pub size: u64,
    /// 总记录数
    pub total: u64,
    /// 总页数
    pub pages: u64,
}

impl<T> PageResponse<T> {
    pub fn new(items: Vec<T>, page: u64, size: u64, total: u64) -> Self {
        let pages = if total == 0 {
            0
        } else {
            (total + size - 1) / size
        };

        Self {
            items,
            page,
            size,
            total,
            pages,
        }
    }

    pub fn empty(page: u64, size: u64) -> Self {
        Self::new(Vec::new(), page, size, 0)
    }

    pub fn map<U, F>(self, f: F) -> PageResponse<U>
    where
        F: FnMut(T) -> U,
    {
        PageResponse {
            items: self.items.into_iter().map(f).collect(),
            page: self.page,
            size: self.size,
            total: self.total,
            pages: self.pages,
        }
    }
}

/// 分页查询参数DTO
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_size")]
    pub size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_size() -> u64 {
    10
}

impl From<PaginationQuery> for PageRequest {
    fn from(query: PaginationQuery) -> Self {
        PageRequest::new(query.page, query.size)
    }
}

impl From<PageRequest> for PaginationQuery {
    fn from(page_request: PageRequest) -> Self {
        PaginationQuery {
            page: page_request.page(),
            size: page_request.size(),
        }
    }
}

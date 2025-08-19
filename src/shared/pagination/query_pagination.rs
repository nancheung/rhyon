use super::repository_pagination::RepositoryPagination;

#[derive(Debug, Clone)]
pub struct QueryPagination {
    page: u64,
    size: u64,
}

impl QueryPagination {
    pub fn new(page: u64, size: u64) -> Self {
        let page = page.max(1);
        let size = size.clamp(1, 100); // 业务规则：限制页面大小
        Self { page, size }
    }

    pub fn page(&self) -> u64 {
        self.page
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    /// 转换为仓储层分页参数
    pub fn to_repository_pagination(&self) -> RepositoryPagination {
        RepositoryPagination::new(
            (self.page - 1) * self.size,
            self.size,
        )
    }
}

impl Default for QueryPagination {
    fn default() -> Self {
        Self::new(1, 10)
    }
}
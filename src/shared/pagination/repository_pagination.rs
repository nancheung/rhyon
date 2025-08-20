#[derive(Debug, Clone)]
pub struct RepositoryPagination {
    pub offset: u64,
    pub limit: u64,
}

impl RepositoryPagination {
    pub fn new(offset: u64, limit: u64) -> Self {
        Self { offset, limit }
    }

    /// 转换为SeaORM分页参数 (页码从0开始)
    pub fn to_sea_orm_params(&self) -> (u64, u64) {
        (self.offset / self.limit, self.limit)
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn limit(&self) -> u64 {
        self.limit
    }
}

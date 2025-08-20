use crate::shared::query::{SortCriteria, SortDirection};

/// Article领域的排序规约
#[derive(Debug, Clone, Default)]
pub enum ArticleSortSpec {
    /// 按发布时间降序
    #[default]
    PublishedAtDesc,
    /// 按发布时间升序
    PublishedAtAsc,
    /// 按标题升序
    TitleAsc,
    /// 按标题降序
    TitleDesc,
    /// 按创建时间降序
    CreatedAtDesc,
    /// 按创建时间升序
    CreatedAtAsc,
    /// 按更新时间降序
    UpdatedAtDesc,
    /// 按更新时间升序
    UpdatedAtAsc,
    /// 自定义多字段排序
    Custom(SortCriteria),
}

impl ArticleSortSpec {
    // 便捷构造函数
    pub fn published_at_desc() -> Self {
        ArticleSortSpec::PublishedAtDesc
    }

    pub fn published_at_asc() -> Self {
        ArticleSortSpec::PublishedAtAsc
    }

    pub fn title_asc() -> Self {
        ArticleSortSpec::TitleAsc
    }

    pub fn title_desc() -> Self {
        ArticleSortSpec::TitleDesc
    }

    pub fn created_at_desc() -> Self {
        ArticleSortSpec::CreatedAtDesc
    }

    pub fn created_at_asc() -> Self {
        ArticleSortSpec::CreatedAtAsc
    }

    pub fn updated_at_desc() -> Self {
        ArticleSortSpec::UpdatedAtDesc
    }

    pub fn updated_at_asc() -> Self {
        ArticleSortSpec::UpdatedAtAsc
    }

    /// 创建自定义排序
    pub fn custom() -> ArticleSortBuilder {
        ArticleSortBuilder::new()
    }

    /// 转换为通用排序条件
    pub fn to_sort_criteria(&self) -> SortCriteria {
        match self {
            ArticleSortSpec::PublishedAtDesc => SortCriteria::new().desc("published_at"),
            ArticleSortSpec::PublishedAtAsc => SortCriteria::new().asc("published_at"),
            ArticleSortSpec::TitleAsc => SortCriteria::new().asc("title"),
            ArticleSortSpec::TitleDesc => SortCriteria::new().desc("title"),
            ArticleSortSpec::CreatedAtDesc => SortCriteria::new().desc("created_at"),
            ArticleSortSpec::CreatedAtAsc => SortCriteria::new().asc("created_at"),
            ArticleSortSpec::UpdatedAtDesc => SortCriteria::new().desc("updated_at"),
            ArticleSortSpec::UpdatedAtAsc => SortCriteria::new().asc("updated_at"),
            ArticleSortSpec::Custom(criteria) => criteria.clone(),
        }
    }
}

impl From<String> for ArticleSortSpec {
    fn from(sort_str: String) -> Self {
        match sort_str.as_str() {
            "published_at:desc" => ArticleSortSpec::PublishedAtDesc,
            "published_at:asc" => ArticleSortSpec::PublishedAtAsc,
            "title:asc" => ArticleSortSpec::TitleAsc,
            "title:desc" => ArticleSortSpec::TitleDesc,
            "created_at:desc" => ArticleSortSpec::CreatedAtDesc,
            "created_at:asc" => ArticleSortSpec::CreatedAtAsc,
            "updated_at:desc" => ArticleSortSpec::UpdatedAtDesc,
            "updated_at:asc" => ArticleSortSpec::UpdatedAtAsc,
            _ => {
                // 尝试解析为多字段排序
                let criteria = SortCriteria::from(sort_str);
                if criteria.fields.is_empty() {
                    ArticleSortSpec::PublishedAtDesc // 默认排序
                } else {
                    ArticleSortSpec::Custom(criteria)
                }
            }
        }
    }
}

/// 用于构建复杂排序的Builder模式
#[derive(Debug)]
pub struct ArticleSortBuilder {
    criteria: SortCriteria,
}

impl ArticleSortBuilder {
    pub fn new() -> Self {
        Self {
            criteria: SortCriteria::new(),
        }
    }

    pub fn published_at(self, direction: SortDirection) -> Self {
        Self {
            criteria: self.criteria.add("published_at", direction),
        }
    }

    pub fn published_at_desc(self) -> Self {
        self.published_at(SortDirection::Desc)
    }

    pub fn published_at_asc(self) -> Self {
        self.published_at(SortDirection::Asc)
    }

    pub fn title(self, direction: SortDirection) -> Self {
        Self {
            criteria: self.criteria.add("title", direction),
        }
    }

    pub fn title_asc(self) -> Self {
        self.title(SortDirection::Asc)
    }

    pub fn title_desc(self) -> Self {
        self.title(SortDirection::Desc)
    }

    pub fn created_at(self, direction: SortDirection) -> Self {
        Self {
            criteria: self.criteria.add("created_at", direction),
        }
    }

    pub fn created_at_desc(self) -> Self {
        self.created_at(SortDirection::Desc)
    }

    pub fn created_at_asc(self) -> Self {
        self.created_at(SortDirection::Asc)
    }

    pub fn updated_at(self, direction: SortDirection) -> Self {
        Self {
            criteria: self.criteria.add("updated_at", direction),
        }
    }

    pub fn updated_at_desc(self) -> Self {
        self.updated_at(SortDirection::Desc)
    }

    pub fn updated_at_asc(self) -> Self {
        self.updated_at(SortDirection::Asc)
    }

    pub fn build(self) -> ArticleSortSpec {
        ArticleSortSpec::Custom(self.criteria)
    }
}

impl Default for ArticleSortBuilder {
    fn default() -> Self {
        Self::new()
    }
}

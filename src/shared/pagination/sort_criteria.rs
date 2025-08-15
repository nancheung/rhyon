#[derive(Debug, Clone)]
pub enum SortCriteria {
    PublishedAtDesc,
    PublishedAtAsc,
    TitleAsc,
    TitleDesc,
    CreatedAtDesc,
    CreatedAtAsc,
    Multiple(Vec<SortCriteria>),
}

impl SortCriteria {
    pub fn to_sql_order(&self) -> Vec<(&'static str, &'static str)> {
        match self {
            SortCriteria::PublishedAtDesc => vec![("published_at", "DESC")],
            SortCriteria::PublishedAtAsc => vec![("published_at", "ASC")],
            SortCriteria::TitleAsc => vec![("title", "ASC")],
            SortCriteria::TitleDesc => vec![("title", "DESC")],
            SortCriteria::CreatedAtDesc => vec![("created_at", "DESC")],
            SortCriteria::CreatedAtAsc => vec![("created_at", "ASC")],
            SortCriteria::Multiple(criteria) => {
                criteria.iter()
                    .flat_map(|c| c.to_sql_order())
                    .collect()
            }
        }
    }
}

impl From<String> for SortCriteria {
    fn from(sort_str: String) -> Self {
        match sort_str.as_str() {
            "published_at:desc" => SortCriteria::PublishedAtDesc,
            "published_at:asc" => SortCriteria::PublishedAtAsc,
            "title:asc" => SortCriteria::TitleAsc,
            "title:desc" => SortCriteria::TitleDesc,
            "created_at:desc" => SortCriteria::CreatedAtDesc,
            "created_at:asc" => SortCriteria::CreatedAtAsc,
            _ => SortCriteria::PublishedAtDesc, // 默认排序
        }
    }
}
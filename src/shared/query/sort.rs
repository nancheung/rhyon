use std::fmt;

/// 通用排序方向枚举
#[derive(Debug, Clone, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl From<&str> for SortDirection {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "asc" | "ascending" => SortDirection::Asc,
            "desc" | "descending" => SortDirection::Desc,
            _ => SortDirection::Desc, // 默认降序
        }
    }
}

impl From<String> for SortDirection {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortDirection::Asc => write!(f, "ASC"),
            SortDirection::Desc => write!(f, "DESC"),
        }
    }
}

/// 通用排序字段
#[derive(Debug, Clone, PartialEq)]
pub struct SortField {
    pub field_name: String,
    pub direction: SortDirection,
}

impl SortField {
    pub fn new(field_name: impl Into<String>, direction: SortDirection) -> Self {
        Self {
            field_name: field_name.into(),
            direction,
        }
    }

    pub fn asc(field_name: impl Into<String>) -> Self {
        Self::new(field_name, SortDirection::Asc)
    }

    pub fn desc(field_name: impl Into<String>) -> Self {
        Self::new(field_name, SortDirection::Desc)
    }

    pub fn to_sql_order(&self) -> (String, String) {
        (self.field_name.clone(), self.direction.to_string())
    }
}

/// 多字段排序组合
#[derive(Debug, Clone)]
pub struct SortCriteria {
    pub fields: Vec<SortField>,
}

impl SortCriteria {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn add_field(mut self, field: SortField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn add(mut self, field_name: impl Into<String>, direction: SortDirection) -> Self {
        self.fields.push(SortField::new(field_name, direction));
        self
    }

    pub fn asc(self, field_name: impl Into<String>) -> Self {
        self.add(field_name, SortDirection::Asc)
    }

    pub fn desc(self, field_name: impl Into<String>) -> Self {
        self.add(field_name, SortDirection::Desc)
    }

    pub fn to_sql_order(&self) -> Vec<(String, String)> {
        self.fields
            .iter()
            .map(|field| field.to_sql_order())
            .collect()
    }
}

impl Default for SortCriteria {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for SortCriteria {
    fn from(sort_str: String) -> Self {
        let mut criteria = SortCriteria::new();

        // 解析格式: "field1:asc,field2:desc"
        for part in sort_str.split(',') {
            let mut parts = part.split(':');
            if let (Some(field), Some(direction)) = (parts.next(), parts.next()) {
                criteria = criteria.add(field.trim(), SortDirection::from(direction.trim()));
            }
        }

        criteria
    }
}

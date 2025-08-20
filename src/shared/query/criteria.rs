/// 通用查询条件
#[derive(Debug, Clone, PartialEq)]
pub enum QueryCriteria {
    Equals(String, QueryValue),
    NotEquals(String, QueryValue),
    Contains(String, String),
    StartsWith(String, String),
    EndsWith(String, String),
    IsNull(String),
    IsNotNull(String),
    GreaterThan(String, QueryValue),
    GreaterThanOrEqual(String, QueryValue),
    LessThan(String, QueryValue),
    LessThanOrEqual(String, QueryValue),
    In(String, Vec<QueryValue>),
    NotIn(String, Vec<QueryValue>),
    Between(String, QueryValue, QueryValue),
    And(Box<QueryCriteria>, Box<QueryCriteria>),
    Or(Box<QueryCriteria>, Box<QueryCriteria>),
    Not(Box<QueryCriteria>),
}

/// 查询值类型
#[derive(Debug, Clone, PartialEq)]
pub enum QueryValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl QueryCriteria {
    // 相等条件
    pub fn eq(field: impl Into<String>, value: impl Into<QueryValue>) -> Self {
        QueryCriteria::Equals(field.into(), value.into())
    }

    // 不等条件
    pub fn ne(field: impl Into<String>, value: impl Into<QueryValue>) -> Self {
        QueryCriteria::NotEquals(field.into(), value.into())
    }

    // 包含条件
    pub fn contains(field: impl Into<String>, value: impl Into<String>) -> Self {
        QueryCriteria::Contains(field.into(), value.into())
    }

    // 开始于条件
    pub fn starts_with(field: impl Into<String>, value: impl Into<String>) -> Self {
        QueryCriteria::StartsWith(field.into(), value.into())
    }

    // 结束于条件
    pub fn ends_with(field: impl Into<String>, value: impl Into<String>) -> Self {
        QueryCriteria::EndsWith(field.into(), value.into())
    }

    // 为空条件
    pub fn is_null(field: impl Into<String>) -> Self {
        QueryCriteria::IsNull(field.into())
    }

    // 非空条件
    pub fn is_not_null(field: impl Into<String>) -> Self {
        QueryCriteria::IsNotNull(field.into())
    }

    // 大于条件
    pub fn gt(field: impl Into<String>, value: impl Into<QueryValue>) -> Self {
        QueryCriteria::GreaterThan(field.into(), value.into())
    }

    // 大于等于条件
    pub fn gte(field: impl Into<String>, value: impl Into<QueryValue>) -> Self {
        QueryCriteria::GreaterThanOrEqual(field.into(), value.into())
    }

    // 小于条件
    pub fn lt(field: impl Into<String>, value: impl Into<QueryValue>) -> Self {
        QueryCriteria::LessThan(field.into(), value.into())
    }

    // 小于等于条件
    pub fn lte(field: impl Into<String>, value: impl Into<QueryValue>) -> Self {
        QueryCriteria::LessThanOrEqual(field.into(), value.into())
    }

    // 在范围内条件
    pub fn in_values(field: impl Into<String>, values: Vec<impl Into<QueryValue>>) -> Self {
        let values = values.into_iter().map(|v| v.into()).collect();
        QueryCriteria::In(field.into(), values)
    }

    // 不在范围内条件
    pub fn not_in(field: impl Into<String>, values: Vec<impl Into<QueryValue>>) -> Self {
        let values = values.into_iter().map(|v| v.into()).collect();
        QueryCriteria::NotIn(field.into(), values)
    }

    // 区间条件
    pub fn between(
        field: impl Into<String>,
        start: impl Into<QueryValue>,
        end: impl Into<QueryValue>,
    ) -> Self {
        QueryCriteria::Between(field.into(), start.into(), end.into())
    }

    // AND组合
    pub fn and(self, other: QueryCriteria) -> Self {
        QueryCriteria::And(Box::new(self), Box::new(other))
    }

    // OR组合
    pub fn or(self, other: QueryCriteria) -> Self {
        QueryCriteria::Or(Box::new(self), Box::new(other))
    }

    // NOT操作
    pub fn not(self) -> Self {
        QueryCriteria::Not(Box::new(self))
    }
}

// 实现QueryValue的From trait
impl From<String> for QueryValue {
    fn from(value: String) -> Self {
        QueryValue::String(value)
    }
}

impl From<&str> for QueryValue {
    fn from(value: &str) -> Self {
        QueryValue::String(value.to_string())
    }
}

impl From<i64> for QueryValue {
    fn from(value: i64) -> Self {
        QueryValue::Integer(value)
    }
}

impl From<i32> for QueryValue {
    fn from(value: i32) -> Self {
        QueryValue::Integer(value as i64)
    }
}

impl From<f64> for QueryValue {
    fn from(value: f64) -> Self {
        QueryValue::Float(value)
    }
}

impl From<f32> for QueryValue {
    fn from(value: f32) -> Self {
        QueryValue::Float(value as f64)
    }
}

impl From<bool> for QueryValue {
    fn from(value: bool) -> Self {
        QueryValue::Boolean(value)
    }
}

use crate::domain::article::aggregate::Article;
use crate::shared::query::{QueryCriteria, Specification};
use chrono::{DateTime, Utc};

/// Article领域的查询规约
#[derive(Debug, Clone)]
pub enum ArticleSpec {
    /// 已发布状态
    Published,
    /// 草稿状态
    Draft,
    /// 标题等于
    TitleEquals(String),
    /// 标题包含
    TitleContains(String),
    /// slug等于
    SlugEquals(String),
    /// 摘要包含
    SummaryContains(String),
    /// 内容包含
    ContentContains(String),
    /// 创建时间晚于
    CreatedAfter(DateTime<Utc>),
    /// 创建时间早于
    CreatedBefore(DateTime<Utc>),
    /// 发布时间晚于
    PublishedAfter(DateTime<Utc>),
    /// 发布时间早于
    PublishedBefore(DateTime<Utc>),
    /// 组合查询
    Complex(QueryCriteria),
}

impl ArticleSpec {
    // 便捷构造函数
    pub fn published() -> Self {
        ArticleSpec::Published
    }

    pub fn draft() -> Self {
        ArticleSpec::Draft
    }

    pub fn title_eq(title: impl Into<String>) -> Self {
        ArticleSpec::TitleEquals(title.into())
    }

    pub fn title_contains(title: impl Into<String>) -> Self {
        ArticleSpec::TitleContains(title.into())
    }

    pub fn slug_eq(slug: impl Into<String>) -> Self {
        ArticleSpec::SlugEquals(slug.into())
    }

    pub fn summary_contains(summary: impl Into<String>) -> Self {
        ArticleSpec::SummaryContains(summary.into())
    }

    pub fn content_contains(content: impl Into<String>) -> Self {
        ArticleSpec::ContentContains(content.into())
    }

    pub fn created_after(date: DateTime<Utc>) -> Self {
        ArticleSpec::CreatedAfter(date)
    }

    pub fn created_before(date: DateTime<Utc>) -> Self {
        ArticleSpec::CreatedBefore(date)
    }

    pub fn published_after(date: DateTime<Utc>) -> Self {
        ArticleSpec::PublishedAfter(date)
    }

    pub fn published_before(date: DateTime<Utc>) -> Self {
        ArticleSpec::PublishedBefore(date)
    }

    /// 创建复杂查询
    pub fn complex() -> ArticleQueryBuilder {
        ArticleQueryBuilder::new()
    }

    /// 转换为SQL查询条件
    pub fn to_query_criteria(&self) -> QueryCriteria {
        match self {
            ArticleSpec::Published => QueryCriteria::eq("status", "published"),
            ArticleSpec::Draft => QueryCriteria::eq("status", "draft"),
            ArticleSpec::TitleEquals(title) => QueryCriteria::eq("title", title.clone()),
            ArticleSpec::TitleContains(title) => QueryCriteria::contains("title", title.clone()),
            ArticleSpec::SlugEquals(slug) => QueryCriteria::eq("slug", slug.clone()),
            ArticleSpec::SummaryContains(summary) => {
                QueryCriteria::contains("summary", summary.clone())
            }
            ArticleSpec::ContentContains(content) => {
                QueryCriteria::contains("content", content.clone())
            }
            ArticleSpec::CreatedAfter(date) => QueryCriteria::gt("created_at", date.timestamp()),
            ArticleSpec::CreatedBefore(date) => QueryCriteria::lt("created_at", date.timestamp()),
            ArticleSpec::PublishedAfter(date) => {
                QueryCriteria::gt("published_at", date.timestamp())
            }
            ArticleSpec::PublishedBefore(date) => {
                QueryCriteria::lt("published_at", date.timestamp())
            }
            ArticleSpec::Complex(criteria) => criteria.clone(),
        }
    }
}

impl Specification<Article> for ArticleSpec {
    fn is_satisfied_by(&self, article: &Article) -> bool {
        match self {
            ArticleSpec::Published => article.status.to_string() == "published",
            ArticleSpec::Draft => article.status.to_string() == "draft",
            ArticleSpec::TitleEquals(title) => article.title.value() == title,
            ArticleSpec::TitleContains(title) => article.title.value().contains(title),
            ArticleSpec::SlugEquals(slug) => article.slug.value() == slug,
            ArticleSpec::SummaryContains(summary) => article.summary.value().contains(summary),
            ArticleSpec::ContentContains(content) => article.content.value().contains(content),
            ArticleSpec::CreatedAfter(date) => article.created_at > *date,
            ArticleSpec::CreatedBefore(date) => article.created_at < *date,
            ArticleSpec::PublishedAfter(date) => article
                .published_at
                .is_some_and(|pub_date| pub_date > *date),
            ArticleSpec::PublishedBefore(date) => article
                .published_at
                .is_some_and(|pub_date| pub_date < *date),
            ArticleSpec::Complex(_) => {
                // 复杂查询需要在数据库层面处理，这里返回true
                true
            }
        }
    }
}

/// 用于构建复杂查询的Builder模式
#[derive(Debug)]
pub struct ArticleQueryBuilder {
    criteria: Option<QueryCriteria>,
}

impl ArticleQueryBuilder {
    pub fn new() -> Self {
        Self { criteria: None }
    }

    pub fn published(self) -> Self {
        self.add_criteria(QueryCriteria::eq("status", "published"))
    }

    pub fn draft(self) -> Self {
        self.add_criteria(QueryCriteria::eq("status", "draft"))
    }

    pub fn title_eq(self, title: impl Into<String>) -> Self {
        self.add_criteria(QueryCriteria::eq("title", title.into()))
    }

    pub fn title_contains(self, title: impl Into<String>) -> Self {
        self.add_criteria(QueryCriteria::contains("title", title.into()))
    }

    pub fn slug_eq(self, slug: impl Into<String>) -> Self {
        self.add_criteria(QueryCriteria::eq("slug", slug.into()))
    }

    pub fn content_contains(self, content: impl Into<String>) -> Self {
        self.add_criteria(QueryCriteria::contains("content", content.into()))
    }

    pub fn created_after(self, date: DateTime<Utc>) -> Self {
        self.add_criteria(QueryCriteria::gt("created_at", date.timestamp()))
    }

    pub fn created_before(self, date: DateTime<Utc>) -> Self {
        self.add_criteria(QueryCriteria::lt("created_at", date.timestamp()))
    }

    pub fn and(self, other: ArticleQueryBuilder) -> Self {
        match (self.criteria, other.criteria) {
            (Some(left), Some(right)) => Self {
                criteria: Some(left.and(right)),
            },
            (Some(criteria), None) | (None, Some(criteria)) => Self {
                criteria: Some(criteria),
            },
            (None, None) => Self { criteria: None },
        }
    }

    pub fn or(self, other: ArticleQueryBuilder) -> Self {
        match (self.criteria, other.criteria) {
            (Some(left), Some(right)) => Self {
                criteria: Some(left.or(right)),
            },
            (Some(criteria), None) | (None, Some(criteria)) => Self {
                criteria: Some(criteria),
            },
            (None, None) => Self { criteria: None },
        }
    }

    pub fn build(self) -> ArticleSpec {
        match self.criteria {
            Some(criteria) => ArticleSpec::Complex(criteria),
            None => ArticleSpec::Published, // 默认返回已发布
        }
    }

    fn add_criteria(self, criteria: QueryCriteria) -> Self {
        match self.criteria {
            Some(existing) => Self {
                criteria: Some(existing.and(criteria)),
            },
            None => Self {
                criteria: Some(criteria),
            },
        }
    }
}

impl Default for ArticleQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

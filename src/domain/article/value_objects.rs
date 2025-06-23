use crate::core::error::RhyonError;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// 技术ID - 仅用于数据库标识
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Id(Uuid);

impl Id {
    pub fn from(id: Uuid) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 文章标题
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Title {
    value: String,
}

impl Title {
    pub fn new(title: String) -> Result<Self, RhyonError> {
        let title = title.trim().to_string();

        if title.is_empty() {
            return Err(RhyonError::Validation("文章标题不能为空".to_string()));
        }

        if title.len() > 200 {
            return Err(RhyonError::Validation(
                "文章标题过长（最多200个字符）".to_string(),
            ));
        }

        Ok(Self { value: title })
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// 文章Slug - 用于URL的自然键
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Slug {
    value: String,
}

impl Slug {
    pub fn new(slug: String) -> Result<Self, RhyonError> {
        let slug = slug.trim().to_lowercase();

        if slug.is_empty() {
            return Err(RhyonError::Validation("文章Slug不能为空".to_string()));
        }

        Ok(Self { value: slug })
    }

    /// 从标题生成Slug
    pub fn from_title(title: &str) -> Result<Self, RhyonError> {
        let slug = title
            .to_lowercase()
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() {
                    c
                } else if c.is_whitespace() {
                    '-'
                } else {
                    '_'
                }
            })
            .collect::<String>();

        // 去除多余的连字符
        let slug = slug.split("--").collect::<Vec<&str>>().join("-");

        Self::new(slug)
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// 文章摘要
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Summary {
    value: String,
}

impl Summary {
    pub fn new(summary: String) -> Result<Self, RhyonError> {
        let summary = summary.trim().to_string();

        if summary.len() > 500 {
            return Err(RhyonError::Validation(
                "摘要过长（最多500个字符）".to_string(),
            ));
        }

        Ok(Self { value: summary })
    }

    /// 从内容生成摘要
    pub fn generate_from_content(content: &Content) -> Self {
        let content_text = content.value();
        let max_length = 200;

        let summary = if content_text.len() <= max_length {
            content_text.to_string()
        } else {
            // 尝试在句号或段落处截断
            let mut end_pos = max_length;

            // 尝试找到最后一个句号或换行符位置
            for (i, c) in content_text
                .char_indices()
                .take(max_length)
                .collect::<Vec<_>>()
                .iter()
                .rev()
            {
                if *c == '.' || *c == '\n' {
                    end_pos = i + 1; // 包含句号或换行符
                    break;
                }
            }

            format!("{}...", content_text[..end_pos].trim())
        };

        Self { value: summary }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// 文章内容
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Content {
    value: String,
}

impl Content {
    pub fn new(content: String) -> Self {
        Self { value: content }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn is_empty(&self) -> bool {
        self.value.trim().is_empty()
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// 文章状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Draft,     // 草稿
    Published, // 已发布
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Draft => "draft",
            Status::Published => "published",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, RhyonError> {
        match s.to_lowercase().as_str() {
            "draft" => Ok(Status::Draft),
            "published" => Ok(Status::Published),
            _ => Err(RhyonError::Validation("无效的文章状态".to_string())),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

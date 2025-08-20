use crate::adapters::outbound::persistence::entities::{Column, Entity};
use crate::domain::article::specifications::{ArticleSortSpec, ArticleSpec};
use crate::shared::query::{QueryCriteria, QueryValue, SortCriteria, SortField};
use sea_orm::{ColumnTrait, Condition, QueryOrder};

pub struct QueryTranslator;

impl QueryTranslator {
    /// 将ArticleSpec转换为SeaORM条件
    pub fn translate_specification(spec: &ArticleSpec) -> Condition {
        let criteria = spec.to_query_criteria();
        Self::translate_criteria(&criteria)
    }

    /// 将QueryCriteria转换为SeaORM的Condition
    fn translate_criteria(criteria: &QueryCriteria) -> Condition {
        match criteria {
            QueryCriteria::Equals(field, value) => {
                let column = Self::field_to_column(field);
                Condition::all().add(column.eq(Self::value_to_sea_orm(value)))
            }
            QueryCriteria::NotEquals(field, value) => {
                let column = Self::field_to_column(field);
                Condition::all().add(column.ne(Self::value_to_sea_orm(value)))
            }
            QueryCriteria::Contains(field, value) => {
                let column = Self::field_to_column(field);
                Condition::all().add(column.contains(value))
            }
            QueryCriteria::StartsWith(field, value) => {
                let column = Self::field_to_column(field);
                Condition::all().add(column.starts_with(value))
            }
            QueryCriteria::EndsWith(field, value) => {
                let column = Self::field_to_column(field);
                Condition::all().add(column.ends_with(value))
            }
            QueryCriteria::IsNull(field) => {
                let column = Self::field_to_column(field);
                Condition::all().add(column.is_null())
            }
            QueryCriteria::IsNotNull(field) => {
                let column = Self::field_to_column(field);
                Condition::all().add(column.is_not_null())
            }
            QueryCriteria::GreaterThan(field, value) => {
                let column = Self::field_to_column(field);
                Condition::all().add(column.gt(Self::value_to_sea_orm(value)))
            }
            QueryCriteria::GreaterThanOrEqual(field, value) => {
                let column = Self::field_to_column(field);
                Condition::all().add(column.gte(Self::value_to_sea_orm(value)))
            }
            QueryCriteria::LessThan(field, value) => {
                let column = Self::field_to_column(field);
                Condition::all().add(column.lt(Self::value_to_sea_orm(value)))
            }
            QueryCriteria::LessThanOrEqual(field, value) => {
                let column = Self::field_to_column(field);
                Condition::all().add(column.lte(Self::value_to_sea_orm(value)))
            }
            QueryCriteria::In(field, values) => {
                let column = Self::field_to_column(field);
                let sea_orm_values: Vec<_> = values.iter().map(Self::value_to_sea_orm).collect();
                Condition::all().add(column.is_in(sea_orm_values))
            }
            QueryCriteria::NotIn(field, values) => {
                let column = Self::field_to_column(field);
                let sea_orm_values: Vec<_> = values.iter().map(Self::value_to_sea_orm).collect();
                Condition::all().add(column.is_not_in(sea_orm_values))
            }
            QueryCriteria::Between(field, start, end) => {
                let column = Self::field_to_column(field);
                Condition::all()
                    .add(column.gte(Self::value_to_sea_orm(start)))
                    .add(column.lte(Self::value_to_sea_orm(end)))
            }
            QueryCriteria::And(left, right) => Condition::all()
                .add(Self::translate_criteria(left))
                .add(Self::translate_criteria(right)),
            QueryCriteria::Or(left, right) => Condition::any()
                .add(Self::translate_criteria(left))
                .add(Self::translate_criteria(right)),
            QueryCriteria::Not(criteria) => {
                // SeaORM没有直接的NOT支持，这里简化处理
                Self::translate_criteria(criteria)
            }
        }
    }

    /// 将ArticleSortSpec应用到查询
    pub fn apply_sort(
        query: sea_orm::Select<Entity>,
        sort_spec: &ArticleSortSpec,
    ) -> sea_orm::Select<Entity> {
        let sort_criteria = sort_spec.to_sort_criteria();
        Self::apply_sort_criteria(query, &sort_criteria)
    }

    /// 应用通用排序条件
    fn apply_sort_criteria(
        query: sea_orm::Select<Entity>,
        criteria: &SortCriteria,
    ) -> sea_orm::Select<Entity> {
        let mut q = query;

        for field in &criteria.fields {
            q = Self::apply_sort_field(q, field);
        }

        q
    }

    /// 应用单个排序字段
    fn apply_sort_field(
        query: sea_orm::Select<Entity>,
        field: &SortField,
    ) -> sea_orm::Select<Entity> {
        let column = Self::field_to_column(&field.field_name);

        match field.direction {
            crate::shared::query::SortDirection::Asc => query.order_by_asc(column),
            crate::shared::query::SortDirection::Desc => query.order_by_desc(column),
        }
    }

    /// 将字段名转换为SeaORM列
    fn field_to_column(field_name: &str) -> Column {
        match field_name {
            "id" => Column::Id,
            "title" => Column::Title,
            "slug" => Column::Slug,
            "summary" => Column::Summary,
            "content" => Column::Content,
            "status" => Column::Status,
            "published_at" => Column::PublishedAt,
            "created_at" => Column::CreatedAt,
            "updated_at" => Column::UpdatedAt,
            _ => Column::Id, // 默认使用ID列，实际中应该抛出错误
        }
    }

    /// 将QueryValue转换为SeaORM值
    fn value_to_sea_orm(value: &QueryValue) -> sea_orm::Value {
        match value {
            QueryValue::String(s) => sea_orm::Value::String(Some(Box::new(s.clone()))),
            QueryValue::Integer(i) => sea_orm::Value::BigInt(Some(*i)),
            QueryValue::Float(f) => sea_orm::Value::Double(Some(*f)),
            QueryValue::Boolean(b) => sea_orm::Value::Bool(Some(*b)),
            QueryValue::Null => sea_orm::Value::String(None),
        }
    }
}

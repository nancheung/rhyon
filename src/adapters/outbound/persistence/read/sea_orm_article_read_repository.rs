use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};

use crate::adapters::outbound::persistence::entities::{Column, Entity};
use crate::adapters::outbound::persistence::read::QueryTranslator;
use crate::application::models::ArticleQueryModel;
use crate::application::ports::ArticleReadRepository;
use crate::core::types::conversions::Converter;
use crate::domain::article::specifications::{ArticleSortSpec, ArticleSpec};
use crate::shared::errors::RhyonError;
use crate::shared::pagination::{QueryPage, QueryPagination};

pub struct SeaOrmArticleReadRepository {
    db: DatabaseConnection,
}

impl SeaOrmArticleReadRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ArticleReadRepository for SeaOrmArticleReadRepository {
    async fn find_by_specification(
        &self,
        specification: ArticleSpec,
        sort: ArticleSortSpec,
        pagination: QueryPagination,
    ) -> Result<QueryPage<ArticleQueryModel>, RhyonError> {
        let repo_pagination = pagination.to_repository_pagination();

        // 构建SeaORM查询
        let mut query = Entity::find();

        // 应用查询条件
        let condition = QueryTranslator::translate_specification(&specification);
        query = query.filter(condition);

        // 应用排序
        query = QueryTranslator::apply_sort(query, &sort);

        // 执行分页查询
        let paginator = query.paginate(&self.db, repo_pagination.limit());
        let (page_num, _) = repo_pagination.to_sea_orm_params();

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page_num).await?;

        // 转换为查询模型（列表查询不包含content字段）
        let models = items
            .into_iter()
            .map(|model| {
                ArticleQueryModel::for_list(
                    model.id.to_string(),
                    model.title,
                    model.slug,
                    model.summary,
                    model.status,
                    model.published_at.map(|dt| dt.convert()),
                    model.created_at.convert(),
                    model.updated_at.convert(),
                )
            })
            .collect();

        Ok(QueryPage::new(
            models,
            pagination.page(),
            pagination.size(),
            total,
        ))
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<ArticleQueryModel>, RhyonError> {
        let model = Entity::find()
            .filter(Column::Slug.eq(slug))
            .one(&self.db)
            .await?;

        Ok(model.map(|m| {
            ArticleQueryModel::for_detail(
                m.id.to_string(),
                m.title,
                m.slug,
                m.summary,
                m.content, // 详情查询包含完整内容
                m.status,
                m.published_at.map(|dt| dt.convert()),
                m.created_at.convert(),
                m.updated_at.convert(),
            )
        }))
    }
}

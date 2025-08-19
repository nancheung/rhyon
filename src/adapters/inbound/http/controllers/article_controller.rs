use axum::extract::{Path, Query, State};
use axum::Json;
use std::sync::Arc;

use crate::adapters::inbound::http::dto::{
    ArticleDetailHttpResponse, ArticleListHttpResponse, CreateArticleHttpRequest,
    HttpPaginationRequest, HttpPaginationResponse,
};
use crate::application::queries::{GetArticleBySlugQuery, GetArticlesQuery};
use crate::application::services::ArticleApplicationService;
use crate::domain::article::specifications::ArticleSortSpec;
use crate::core::response::R;
use crate::shared::errors::RhyonError;

pub struct ArticleController {
    application_service: Arc<dyn ArticleApplicationService>,
}

impl ArticleController {
    pub fn new(application_service: Arc<dyn ArticleApplicationService>) -> Self {
        Self { application_service }
    }

    /// POST /articles - 创建文章
    pub async fn create_article(
        State(controller): State<Arc<ArticleController>>,
        Json(request): Json<CreateArticleHttpRequest>,
    ) -> Result<R<String>, RhyonError> {
        let command = request.into();
        let article_id = controller
            .application_service
            .create_article(command)
            .await?;
        Ok(R::success(article_id.to_string()))
    }

    /// GET /articles - 获取文章列表
    pub async fn get_articles(
        State(controller): State<Arc<ArticleController>>,
        Query(params): Query<HttpPaginationRequest>,
    ) -> Result<R<HttpPaginationResponse<ArticleListHttpResponse>>, RhyonError> {
        // 从HTTP参数构建查询对象
        let sort = params.sort_string()
            .map(|s| ArticleSortSpec::from(s))
            .unwrap_or_default();
        let pagination = params.into_pagination();
        
        let query = GetArticlesQuery::new(pagination).with_sort(sort);
        let result = controller.application_service.get_articles(query).await?;
        
        let response: HttpPaginationResponse<ArticleListHttpResponse> = result
            .map(|model| model.into())
            .into();
            
        Ok(R::success(response))
    }

    /// GET /articles/{slug} - 根据slug获取文章详情
    pub async fn get_article_by_slug(
        State(controller): State<Arc<ArticleController>>,
        Path(slug): Path<String>,
    ) -> Result<R<ArticleDetailHttpResponse>, RhyonError> {
        let query = GetArticleBySlugQuery::new(slug);
        let article = controller
            .application_service
            .get_article_by_slug(query)
            .await?
            .ok_or(RhyonError::NotFound)?;
            
        Ok(R::success(article.into()))
    }
}
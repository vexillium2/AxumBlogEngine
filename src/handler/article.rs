// src/handler/article.rs
use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    database::article,
    error::{CustomError, CustomResult},
    handler::auth::Claims,
    AppState,
};

// ---- 请求/响应数据结构 ----
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateArticleRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(min = 10))]
    pub content: String,
    pub category_id: i32,
    #[serde(default)]
    pub is_published: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateArticleRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,
    #[validate(length(min = 10))]
    pub content: Option<String>,
    pub category_id: Option<i32>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleResponse {
    pub id: i32,
    pub title: String,
    pub author_id: i32,
    pub category_id: i32,
    pub is_published: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SearchParams {
    #[validate(length(max = 50))]
    pub q: Option<String>,      // 搜索关键词
    pub category: Option<i32>,  // 分类过滤
    #[serde(default = "default_page")]
    pub page: u32,             // 分页页码
    #[serde(default = "default_per_page")]
    pub per_page: u32,         // 每页数量
}

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 10 }

// ---- 处理器实现 ----

/// 创建文章
/// POST /articles
pub async fn create_article(
    claims: Claims,
    State(state): State<AppState>,
    Json(req): Json<CreateArticleRequest>,
) -> CustomResult<Json<ArticleResponse>> {
    req.validate()?;

    let article = article::create(
        &state.db,
        &req.title,
        &req.content,
        claims.sub.parse()?,
        req.category_id,
        req.is_published,
    )
    .await?;

    Ok(Json(ArticleResponse {
        id: article.id,
        title: article.title,
        author_id: article.author_id,
        category_id: article.category_id,
        is_published: article.is_published,
        created_at: article.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
    }))
}

/// 搜索文章
/// GET /articles?q=...&category=...&page=...
pub async fn search_articles(
    Query(params): Query<SearchParams>,
    State(state): State<AppState>,
) -> CustomResult<Json<Vec<ArticleResponse>>> {
    params.validate()?;

    let articles = article::search(
        &state.db,
        params.q.as_deref(),
        params.category,
        params.page,
        params.per_page,
    )
    .await?;

    Ok(Json(
        articles
            .into_iter()
            .map(|a| ArticleResponse {
                id: a.id,
                title: a.title,
                author_id: a.author_id,
                category_id: a.category_id,
                is_published: a.is_published,
                created_at: a.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            })
            .collect(),
    ))
}

/// 更新文章
/// PUT /articles/{id}
pub async fn update_article(
    claims: Claims,
    Path(article_id): Path<i32>,
    State(state): State<AppState>,
    Json(req): Json<UpdateArticleRequest>,
) -> CustomResult<Json<ArticleResponse>> {
    req.validate()?;

    // 验证文章所有权
    let target = article::find_by_id(&state.db, article_id)
        .await?
        .ok_or(CustomError::ArticleNotFound)?;

    if target.author_id != claims.sub.parse()? && claims.role != "admin" {
        return Err(CustomError::Forbidden);
    }

    let mut update_data = article::ActiveModel {
        id: sea_orm::Set(article_id),
        ..Default::default()
    };

    if let Some(title) = req.title {
        update_data.title = sea_orm::Set(title);
    }
    if let Some(content) = req.content {
        update_data.content = sea_orm::Set(content);
    }
    if let Some(category_id) = req.category_id {
        update_data.category_id = sea_orm::Set(category_id);
    }
    if let Some(is_published) = req.is_published {
        update_data.is_published = sea_orm::Set(is_published);
    }

    let updated = article::update(&state.db, update_data).await?;

    Ok(Json(ArticleResponse {
        id: updated.id,
        title: updated.title,
        author_id: updated.author_id,
        category_id: updated.category_id,
        is_published: updated.is_published,
        created_at: updated.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
    }))
}

/// 删除文章
/// DELETE /articles/{id}
pub async fn delete_article(
    claims: Claims,
    Path(article_id): Path<i32>,
    State(state): State<AppState>,
) -> CustomResult<Json<()>> {
    // 验证文章所有权
    let target = article::find_by_id(&state.db, article_id)
        .await?
        .ok_or(CustomError::ArticleNotFound)?;

    if target.author_id != claims.sub.parse()? && claims.role != "admin" {
        return Err(CustomError::Forbidden);
    }

    article::delete(&state.db, article_id).await?;
    Ok(Json(()))
}
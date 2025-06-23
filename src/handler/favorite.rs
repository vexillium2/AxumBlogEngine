// src/handler/favorite.rs
use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;

use crate::{
    database::{article, favorite},
    error::{CustomError, CustomResult},
    handler::auth::Claims,
    AppState,
};

// ---- 响应数据结构 ----
#[derive(Debug, Serialize)]
pub struct FavoriteResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

// ---- 处理器实现 ----

/// 点赞文章
/// POST /articles/{id}/favorite
pub async fn favorite_article(
    claims: Claims,
    Path(article_id): Path<i32>,
    State(state): State<AppState>,
) -> CustomResult<Json<FavoriteResponse>> {
    // 验证文章是否存在
    if !article::exists(&state.db, article_id).await? {
        return Err(CustomError::ArticleNotFound);
    }

    let user_id = claims.sub.parse().map_err(|_| CustomError::InvalidUserId)?;
    
    // 检查是否已点赞
    if favorite::exists(&state.db, user_id, article_id).await? {
        return Ok(Json(FavoriteResponse {
            success: false,
            message: "Already favorited".to_string(),
            count: None,
        }));
    }

    // 创建点赞记录
    favorite::create(&state.db, user_id, article_id).await?;

    // 获取最新点赞数
    let count = favorite::count(&state.db, article_id).await?;

    Ok(Json(FavoriteResponse {
        success: true,
        message: "Favorited successfully".to_string(),
        count: Some(count),
    }))
}

/// 取消点赞
/// DELETE /articles/{id}/favorite
pub async fn unfavorite_article(
    claims: Claims,
    Path(article_id): Path<i32>,
    State(state): State<AppState>,
) -> CustomResult<Json<FavoriteResponse>> {
    let user_id = claims.sub.parse().map_err(|_| CustomError::InvalidUserId)?;

    // 检查是否已点赞
    if !favorite::exists(&state.db, user_id, article_id).await? {
        return Ok(Json(FavoriteResponse {
            success: false,
            message: "Not favorited yet".to_string(),
            count: None,
        }));
    }

    // 删除点赞记录
    favorite::delete(&state.db, user_id, article_id).await?;

    // 获取最新点赞数
    let count = favorite::count(&state.db, article_id).await?;

    Ok(Json(FavoriteResponse {
        success: true,
        message: "Unfavorited successfully".to_string(),
        count: Some(count),
    }))
}

/// 获取文章点赞状态
/// GET /articles/{id}/favorite/status
pub async fn get_favorite_status(
    claims: Claims,
    Path(article_id): Path<i32>,
    State(state): State<AppState>,
) -> CustomResult<Json<FavoriteResponse>> {
    let user_id = claims.sub.parse().map_err(|_| CustomError::InvalidUserId)?;

    let is_favorited = favorite::exists(&state.db, user_id, article_id).await?;
    let count = favorite::count(&state.db, article_id).await?;

    Ok(Json(FavoriteResponse {
        success: is_favorited,
        message: if is_favorited {
            "Already favorited".to_string()
        } else {
            "Not favorited".to_string()
        },
        count: Some(count),
    }))
}
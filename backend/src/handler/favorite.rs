//! 文章收藏相关的 HTTP 请求处理函数。
//! 遵循 RESTful API 设计风格。

use axum::{
    extract::{Path, State, Json, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{post, get}, // 收藏目前只有 POST 和 GET 动词
    Router,
};
use serde_json::json;
use validator::Validate;
use tracing::{debug, info, error, warn};

// 导入 crate 根目录下的 AppState
use crate::AppState;
// 导入数据库模块的 FavoriteRepository
use crate::database::favorite::FavoriteRepository;
// 导入用户实体模型，用于权限检查
use crate::database::user::Model as UserModel;
// 导入 IDL 中定义的请求和响应 DTOs
use crate::handler::idl::{
    ToggleFavoriteRequest, FavoriteListResponse, BaseResponse,
};
// 导入自定义错误类型和通用 Result
use crate::error::{CustomError, CustomResult};
// 导入认证提取器
use crate::handler::user::AuthUser; // 从 user 模块引入 AuthUser


// ======================== 文章收藏相关 API 处理函数 (RESTful 风格) ========================

/// 切换文章收藏状态 (POST /post_fav)
/// 需要用户认证。
/// 如果用户已收藏某文章，则取消收藏；否则，添加收藏。
pub async fn toggle_post_favorite(
    AuthUser(claims): AuthUser, // 认证用户，获取操作者 ID
    State(state): State<AppState>,
    Json(req): Json<ToggleFavoriteRequest>,
) -> CustomResult<Json<BaseResponse>> {
    req.validate()?; // 验证请求数据

    let user_id = claims.sub.parse::<i32>()
        .map_err(|_| CustomError::Unauthorized("JWT令牌中的用户ID无效".to_string()))?;

    // 检查文章是否存在，确保操作的合法性
    if state.post_repo.get_post_by_id(req.post_id).await?.is_none() {
        return Err(CustomError::NotFound(format!("ID为 {} 的文章未找到", req.post_id)));
    }

    let is_favorited = state.favorite_repo.toggle_favorite(user_id, req.post_id).await?;

    let message = if is_favorited {
        "文章收藏成功".to_string()
    } else {
        "文章已取消收藏".to_string()
    };

    Ok(Json(BaseResponse {
        success: true,
        message: Some(message),
    }))
}

/// 获取当前用户收藏的文章列表 (GET /post_fav/my/list)
/// 需要用户认证。
pub async fn get_my_favorite_posts(
    AuthUser(claims): AuthUser, // 认证用户，获取用户 ID
    State(state): State<AppState>,
    Query(req): Query<crate::handler::idl::PostListRequest>, // 使用 PostListRequest 进行分页
) -> CustomResult<Json<FavoriteListResponse>> {
    let user_id = claims.sub.parse::<i32>()
        .map_err(|_| CustomError::Unauthorized("JWT令牌中的用户ID无效".to_string()))?;

    // 默认页码和每页数量
    let page = req.page.unwrap_or(1);
    let limit = req.limit.unwrap_or(10);

    let (posts_info, total_pages, current_page, total_favorites) =
        state.favorite_repo.get_paginated_user_favorites(user_id, page, limit).await?;

    Ok(Json(FavoriteListResponse {
        success: true,
        favorites: posts_info,
        total_pages,
        current_page,
        total_favorites,
        message: None,
    }))
}


/// 收藏路由配置函数。
/// 此函数将所有文章收藏相关的路由组合起来，方便在 `src/bin/server.rs` 中集成。
pub fn favorite_routes() -> Router<AppState> {
    Router::new()
        // POST /post_fav - 切换收藏状态 (收藏/取消收藏)
        .route("/", post(toggle_post_favorite))
        // GET /post_fav/my/list - 获取当前用户收藏的文章列表
        .route("/my/list", get(get_my_favorite_posts))
}

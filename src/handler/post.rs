//! 文章相关的 HTTP 请求处理函数。
//! 遵循 RESTful API 设计风格。

use axum::{
    extract::{Path, State, Json, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{post, get, put, delete},
    Router,
};
use serde_json::json;
use validator::Validate;
use tracing::{debug, info, error, warn};

// 导入 crate 根目录下的 AppState
use crate::AppState;
// 导入数据库模块的 PostRepository 和文章实体模型
use crate::database::post::{PostRepository, Model as PostModel};
// 导入用户实体模型，用于权限检查时获取作者信息
use crate::database::user::Model as UserModel;
// 导入 IDL 中定义的请求和响应 DTOs
use crate::handler::idl::{
    CreatePostRequest, UpdatePostRequest, PostListRequest, PostInfo,
    PostListResponse, BaseResponse, IdResponse,
};
// 导入自定义错误类型和通用 Result
use crate::error::{CustomError, CustomResult};
// 导入认证提取器
use crate::handler::user::AuthUser; // 从 user 模块引入 AuthUser


/// 将文章 `Model` 转换为 `PostInfo` DTO。
/// 这是一个辅助函数，用于将数据库实体映射为 API 响应的数据结构。
pub fn convert_post_model_to_post_info(model: PostModel) -> PostInfo {
    PostInfo {
        id: model.id,
        title: model.title,
        content_markdown: model.content_markdown,
        category: model.category,
        author_id: model.author_id,
        is_published: model.is_published == 1, // 数据库中 1/0 转换为 bool
        view_count: model.view_count,
        cover_url: model.cover_url,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}


// ======================== 文章相关 API 处理函数 (RESTful 风格) ========================

/// 创建文章 (POST /posts)
/// 需要用户认证。
pub async fn create_post(
    AuthUser(claims): AuthUser, // 认证用户，获取作者ID
    State(state): State<AppState>,
    Json(req): Json<CreatePostRequest>,
) -> CustomResult<Json<IdResponse>> {
    req.validate()?; // 验证请求数据

    let author_id = claims.sub.parse::<i32>()
        .map_err(|_| CustomError::Unauthorized("JWT令牌中的用户ID无效".to_string()))?;

    let created_post = state.post_repo.create_post(req, author_id).await?;

    Ok(Json(IdResponse {
        success: true,
        id: created_post.id,
        message: Some("文章创建成功".to_string()),
    }))
}

/// 获取文章详情 (GET /posts/:id)
///
/// 此接口对所有用户开放，无论是否认证。
/// 同时会增加文章的浏览量。
pub async fn get_post_by_id(
    State(state): State<AppState>,
    Path(post_id): Path<i32>,
) -> CustomResult<Json<PostInfo>> {
    let post_model = state.post_repo.get_post_by_id(post_id).await?
        .ok_or(CustomError::NotFound(format!("ID为 {} 的文章未找到", post_id)))?;

    // 增加浏览量（异步，不阻塞响应）
    // 注意：这里没有处理 increment_view_count 可能返回的错误，生产环境可能需要更细致的日志或错误处理
    if let Err(e) = state.post_repo.increment_view_count(post_id).await {
        warn!("增加文章 {} 浏览量失败: {}", post_id, e);
    }

    Ok(Json(convert_post_model_to_post_info(post_model)))
}

/// 更新文章 (PUT /posts/:id)
/// 只有文章作者或管理员才能更新文章。
pub async fn update_post(
    AuthUser(claims): AuthUser, // 认证用户，获取操作者信息
    State(state): State<AppState>,
    Path(post_id): Path<i32>, // 要更新的文章 ID
    Json(req): Json<UpdatePostRequest>,
) -> CustomResult<Json<BaseResponse>> {
    req.validate()?; // 验证请求数据

    // 检查文章是否存在
    let existing_post = state.post_repo.get_post_by_id(post_id).await?
        .ok_or(CustomError::NotFound(format!("ID为 {} 的文章未找到", post_id)))?;

    let operator_id = claims.sub.parse::<i32>()
        .map_err(|_| CustomError::Unauthorized("JWT令牌中的用户ID无效".to_string()))?;

    // 权限检查：只有文章作者或管理员才能更新
    if existing_post.author_id != operator_id && claims.role != "admin" {
        return Err(CustomError::Forbidden(
            "无权限更新此文章".to_string(),
        ));
    }

    state.post_repo.update_post(post_id, req).await?;

    Ok(Json(BaseResponse {
        success: true,
        message: Some("文章更新成功".to_string()),
    }))
}

/// 删除文章 (DELETE /posts/:id)
/// 只有文章作者或管理员才能删除文章。
pub async fn delete_post(
    AuthUser(claims): AuthUser, // 认证用户，获取操作者信息
    State(state): State<AppState>,
    Path(post_id): Path<i32>, // 要删除的文章 ID
) -> CustomResult<impl IntoResponse> { // 返回 CustomResult<impl IntoResponse> 以便自定义响应
    // 检查文章是否存在
    let existing_post = state.post_repo.get_post_by_id(post_id).await?
        .ok_or(CustomError::NotFound(format!("ID为 {} 的文章未找到", post_id)))?;

    let operator_id = claims.sub.parse::<i32>()
        .map_err(|_| CustomError::Unauthorized("JWT令牌中的用户ID无效".to_string()))?;

    // 权限检查：只有文章作者或管理员才能删除
    if existing_post.author_id != operator_id && claims.role != "admin" {
        return Err(CustomError::Forbidden(
            "无权限删除此文章".to_string(),
        ));
    }

    state.post_repo.delete_post(post_id).await?;

    Ok((
        StatusCode::NO_CONTENT, // 204 No Content，表示成功但无返回体
        Json(BaseResponse {
            success: true,
            message: Some("文章删除成功".to_string()),
        }),
    ))
}

/// 获取文章列表和搜索 (GET /posts)
///
/// 此接口对所有用户开放，无论是否认证。
/// 支持分页、按分类过滤和全文搜索。
pub async fn get_posts_list_and_search(
    auth_user: Option<AuthUser>, // 允许未认证用户访问，但如果认证了则可以检查权限
    State(state): State<AppState>,
    Query(mut req): Query<PostListRequest>, // 使用 Query 提取查询参数
) -> CustomResult<Json<PostListResponse>> {
    // 如果是普通用户访问，确保只获取已发布的文章
    if auth_user.is_none() || (auth_user.is_some() && auth_user.as_ref().unwrap().0.role != "admin") {
        req.published_only = Some(true); // 强制设置为 true
    }
    // 如果是管理员访问，published_only 按照请求参数来决定（默认为 true）

    let (posts_info, total_pages, current_page, total_posts) = 
        state.post_repo.get_paginated_posts(req).await?;

    Ok(Json(PostListResponse {
        success: true,
        posts: posts_info,
        total_pages,
        current_page,
        total_posts,
        message: None,
    }))
}

/// 文章路由配置函数。
/// 此函数将所有文章相关的路由组合起来，方便在 `src/bin/server.rs` 中集成。
pub fn post_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_post))                  // POST /posts (创建文章)
        .route("/:id", get(get_post_by_id))             // GET /posts/:id (获取文章详情)
        .route("/:id", put(update_post))                // PUT /posts/:id (更新文章)
        .route("/:id", delete(delete_post))             // DELETE /posts/:id (删除文章)
        .route("/", get(get_posts_list_and_search))    // GET /posts (获取文章列表/搜索)
        // 注意：/post/edit 被合并到 /posts/:id 的 PUT 请求中，符合 RESTful 风格
        // /post/search 逻辑也被合并到 /posts 的 GET 请求中，通过 Query 参数实现
}

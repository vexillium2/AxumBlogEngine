//! 评论相关的 HTTP 请求处理函数。
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
// 导入数据库模块的 CommentRepository 和评论实体模型
use crate::database::comment::{CommentRepository, Model as CommentModel};
// 导入用户实体模型，用于权限检查
use crate::database::user::Model as UserModel;
// 导入文章实体模型，用于评论与文章的关联
use crate::database::post::Model as PostModel;

// 导入 IDL 中定义的请求和响应 DTOs
use crate::handler::idl::{
    CreateCommentRequest, UpdateCommentRequest, CommentInfo,
    CommentListRequest, CommentListResponse, BaseResponse, IdResponse,
};
// 导入自定义错误类型和通用 Result
use crate::error::{CustomError, CustomResult};
// 导入认证提取器
use crate::handler::user::AuthUser;


/// 将评论 `Model` 转换为 `CommentInfo` DTO。
/// 这是一个辅助函数，用于将数据库实体映射为 API 响应的数据结构。
fn convert_comment_model_to_comment_info(model: CommentModel) -> CommentInfo {
    CommentInfo {
        id: model.id,
        content: model.content,
        post_id: model.post_id,
        user_id: model.user_id,
        parent_id: model.parent_id,
        created_at: model.created_at,
        // 如果 CommentInfo 包含 username，这里需要 JOIN 查询用户表或在 Service 层处理
        // 例如: username: user_model.username,
    }
}


// ======================== 评论相关 API 处理函数 (RESTful 风格) ========================

/// 创建评论 (POST /comments)
/// 需要用户认证。评论内容需要关联到文章 ID 和可选的父评论 ID。
pub async fn create_comment(
    AuthUser(claims): AuthUser, // 认证用户，获取评论者 ID
    State(state): State<AppState>,
    Json(req): Json<CreateCommentRequest>,
) -> CustomResult<Json<IdResponse>> {
    req.validate()?; // 验证请求数据

    let user_id = claims.sub.parse::<i32>()
        .map_err(|_| CustomError::Unauthorized("JWT令牌中的用户ID无效".to_string()))?;

    // 可以在这里添加额外的业务逻辑，例如：
    // - 检查 `req.post_id` 对应的文章是否存在 (调用 state.post_repo.get_post_by_id)
    // - 检查 `req.parent_id` 对应的父评论是否存在且属于同一文章

    let created_comment = state.comment_repo.create_comment(req, user_id).await?;

    Ok(Json(IdResponse {
        success: true,
        id: created_comment.id,
        message: Some("评论创建成功".to_string()),
    }))
}

/// 获取评论详情 (GET /comments/:id)
///
/// 此接口对所有用户开放。
pub async fn get_comment_by_id(
    State(state): State<AppState>,
    Path(comment_id): Path<i32>,
) -> CustomResult<Json<CommentInfo>> {
    let comment_model = state.comment_repo.get_comment_by_id(comment_id).await?
        .ok_or(CustomError::NotFound(format!("ID为 {} 的评论未找到", comment_id)))?;

    Ok(Json(convert_comment_model_to_comment_info(comment_model)))
}

/// 更新评论 (PUT /comments/:id)
/// 只有评论作者或管理员才能更新评论。
pub async fn update_comment(
    AuthUser(claims): AuthUser, // 认证用户，获取操作者信息
    State(state): State<AppState>,
    Path(comment_id): Path<i32>, // 要更新的评论 ID
    Json(req): Json<UpdateCommentRequest>,
) -> CustomResult<Json<BaseResponse>> {
    req.validate()?; // 验证请求数据

    // 检查评论是否存在
    let existing_comment = state.comment_repo.get_comment_by_id(comment_id).await?
        .ok_or(CustomError::NotFound(format!("ID为 {} 的评论未找到", comment_id)))?;

    let operator_id = claims.sub.parse::<i32>()
        .map_err(|_| CustomError::Unauthorized("JWT令牌中的用户ID无效".to_string()))?;

    // 权限检查：只有评论作者或管理员才能更新评论
    if existing_comment.user_id != operator_id && claims.role != "admin" {
        return Err(CustomError::Forbidden(
            "无权限更新此评论".to_string(),
        ));
    }

    state.comment_repo.update_comment(comment_id, req).await?;

    Ok(Json(BaseResponse {
        success: true,
        message: Some("评论更新成功".to_string()),
    }))
}

/// 删除评论 (DELETE /comments/:id)
/// 只有评论作者、文章作者或管理员才能删除评论。
pub async fn delete_comment(
    AuthUser(claims): AuthUser, // 认证用户，获取操作者信息
    State(state): State<AppState>,
    Path(comment_id): Path<i32>, // 要删除的评论 ID
) -> CustomResult<impl IntoResponse> {
    // 检查评论是否存在
    let existing_comment = state.comment_repo.get_comment_by_id(comment_id).await?
        .ok_or(CustomError::NotFound(format!("ID为 {} 的评论未找到", comment_id)))?;

    let operator_id = claims.sub.parse::<i32>()
        .map_err(|_| CustomError::Unauthorized("JWT令牌中的用户ID无效".to_string()))?;

    // 获取文章信息以检查是否是文章作者
    let post_model = state.post_repo.get_post_by_id(existing_comment.post_id).await?
        .ok_or(CustomError::NotFound(format!("评论所属文章 (ID: {}) 未找到", existing_comment.post_id)))?;


    // 权限检查：只有评论作者、文章作者或管理员才能删除评论
    if existing_comment.user_id != operator_id && // 是否是评论作者
       post_model.author_id != operator_id &&    // 是否是文章作者
       claims.role != "admin" {                   // 是否是管理员
        return Err(CustomError::Forbidden(
            "无权限删除此评论".to_string(),
        ));
    }

    state.comment_repo.delete_comment(comment_id).await?;

    Ok((
        StatusCode::NO_CONTENT, // 204 No Content，表示成功但无返回体
        Json(BaseResponse {
            success: true,
            message: Some("评论删除成功".to_string()),
        }),
    ))
}

/// 获取指定文章的评论列表 (GET /posts/:post_id/comments)
///
/// 此接口对所有用户开放。
/// 支持分页。
pub async fn get_comments_by_post_id(
    State(state): State<AppState>,
    Path(post_id): Path<i32>, // 从路径中获取文章 ID
    Query(req): Query<CommentListRequest>, // 使用 Query 提取查询参数
) -> CustomResult<Json<CommentListResponse>> {
    // 默认页码和每页数量
    let page = req.page.unwrap_or(1);
    let limit = req.limit.unwrap_or(10);

    // 检查文章是否存在 (可选，但推荐)
    // 如果文章不存在，评论列表就没意义
    if state.post_repo.get_post_by_id(post_id).await?.is_none() {
        return Err(CustomError::NotFound(format!("ID为 {} 的文章不存在", post_id)));
    }


    let (comment_models, total_pages, current_page, total_comments) = 
        state.comment_repo.get_paginated_comments_by_post_id(post_id, page, limit).await?;

    let comments_info: Vec<CommentInfo> = comment_models
        .into_iter()
        .map(convert_comment_model_to_comment_info)
        .collect();

    Ok(Json(CommentListResponse {
        success: true,
        comments: comments_info,
        total_pages,
        current_page,
        total_comments,
        message: None,
    }))
}


/// 评论路由配置函数。
/// 此函数将所有评论相关的路由组合起来，方便在 `src/bin/server.rs` 中集成。
pub fn comment_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_comment))                   // POST /comments (创建评论)
        .route("/:id", get(get_comment_by_id))              // GET /comments/:id (获取评论详情)
        .route("/:id", put(update_comment))                 // PUT /comments/:id (更新评论)
        .route("/:id", delete(delete_comment))              // DELETE /comments/:id (删除评论)
        // 获取特定文章的评论列表，URL 结构为 /posts/:post_id/comments
        // 这条路由将由 `server.rs` 中的 `nest("/posts", post_routes().nest("/:post_id/comments", comment_routes_for_post()))` 类似方式集成
        // 或者直接在 post 路由中定义 /posts/:post_id/comments 路由，并调用 get_comments_by_post_id
}

// 针对 /posts/:post_id/comments 的路由需要特殊处理，因为它的前缀依赖于 /posts
// 可以在这里定义一个专门用于嵌套在 /posts/:post_id 下的评论路由
pub fn comment_routes_for_post() -> Router<AppState> {
    Router::new()
        .route("/", get(get_comments_by_post_id)) // GET /posts/:post_id/comments
}

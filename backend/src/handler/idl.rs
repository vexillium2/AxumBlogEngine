//! 定义应用程序的请求和响应数据传输对象 (DTOs)。
//! 这些结构体是客户端与服务器之间数据交换的契约。

use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::NaiveDateTime; // 用于时间戳，与数据库中的 TEXT 格式对应
use axum::response::{IntoResponse, Response};
use axum::Json;

// ======================== 用户相关 DTOs ========================

/// 用户注册请求体：POST /users/register
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 20, message = "用户名长度必须在3到20个字符之间"))]
    pub username: String,
    
    #[validate(email(message = "邮箱格式无效"))]
    pub email: String,
    
    #[validate(length(min = 6, message = "密码长度至少为6个字符"))]
    pub password: String, // 原始密码，后端会进行哈希
}

/// 用户注册响应体
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>, // 注册成功后直接返回 JWT Token
    pub user_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>, // 错误或成功消息
}

/// 用户登录请求体：POST /users/login
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "用户名或邮箱不能为空"))]
    pub username_or_email: String, // 用户名或邮箱
    #[validate(length(min = 1, message = "密码不能为空"))]
    pub password: String,          // 原始密码
}

/// 用户登录响应体
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>, // 登录成功后返回 JWT Token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>, // 登录成功后返回用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>, // 错误或成功消息
}

/// 用户信息结构体（用于响应）
/// 用于返回用户的公共信息，不包含敏感数据如密码哈希。
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
    pub created_at: NaiveDateTime, // 使用 NaiveDateTime 以保持类型一致性
}

/// 获取登录用户信息响应体：GET /users/me
#[derive(Debug, Serialize)]
pub struct UserInfoResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// 更新用户个人信息请求体：PUT /users/me
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateMyProfileRequest {
    #[validate(length(min = 3, max = 20, message = "用户名长度必须在3到20个字符之间"))]
    pub username: Option<String>, // 可以选择性更新
    #[validate(email(message = "邮箱格式无效"))]
    pub email: Option<String>,    // 可以选择性更新
    #[validate(length(min = 6, message = "密码长度至少为6个字符"))]
    pub password: Option<String>, // 新密码，后端会进行哈希
}

/// 更新用户（管理员）请求体：PUT /users/:id
/// 管理员可以更新用户的基本信息，包括角色。
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    // user_id 通常通过路径参数传递，这里不需要
    #[validate(length(min = 3, max = 20, message = "用户名长度必须在3到20个字符之间"))]
    pub username: Option<String>,
    #[validate(email(message = "邮箱格式无效"))]
    pub email: Option<String>,
    #[validate(length(min = 6, message = "密码长度至少为6个字符"))]
    pub password: Option<String>,
    #[validate(custom = "validate_role")] // 自定义角色验证
    pub role: Option<String>, // 管理员可以修改角色
}

// 自定义角色验证函数
fn validate_role(role: &str) -> Result<(), validator::ValidationError> {
    if role == "user" || role == "admin" {
        Ok(())
    } else {
        Err(validator::ValidationError::new("角色必须是 'user' 或 'admin'"))
    }
}


/// 创建用户（管理员）请求体：POST /users
/// 管理员创建新用户时使用，必须指定角色。
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserByAdminRequest {
    #[validate(length(min = 3, max = 20, message = "用户名长度必须在3到20个字符之间"))]
    pub username: String,
    #[validate(email(message = "邮箱格式无效"))]
    pub email: String,
    #[validate(length(min = 6, message = "密码长度至少为6个字符"))]
    pub password: String,
    #[validate(custom = "validate_role")]
    pub role: String, // 管理员创建时指定角色
}

/// 删除用户（管理员）请求体：DELETE /users (批量删除)
/// 用于批量删除用户，请求体中包含要删除的用户 ID 列表。
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserRequest {
    pub user_ids: Vec<i32>,
}


// ======================== 文章相关 DTOs ========================

/// 文章信息结构体 (用于响应)
/// 包含文章的元数据和 Markdown 内容，以及作者ID。
#[derive(Debug, Serialize)]
pub struct PostInfo {
    pub id: i32,
    pub title: String,
    pub content_markdown: String, // 存储 Markdown 内容
    pub category: String,         // 文章分类
    pub author_id: i32,
    pub is_published: bool,       // 数据库中 0/1 对应 false/true
    pub view_count: i32,
    pub cover_url: Option<String>, // 封面 URL 可能是 None
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // 可以在这里添加作者的简要信息，例如：
    // pub author_username: String,
}

/// 创建文章请求体：POST /posts
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 255, message = "标题长度必须在1到255个字符之间"))]
    pub title: String,
    #[validate(length(min = 1, message = "文章内容不能为空"))]
    pub content_markdown: String,
    #[validate(length(min = 1, max = 50, message = "分类长度必须在1到50个字符之间"))]
    pub category: String,
    pub is_published: Option<bool>, // 可选，默认为 false (草稿)
    pub cover_url: Option<String>,
}

/// 更新文章请求体：PUT /posts/:id
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePostRequest {
    // 文章ID通过路径参数传递
    #[validate(length(min = 1, max = 255, message = "标题长度必须在1到255个字符之间"))]
    pub title: Option<String>,
    #[validate(length(min = 1, message = "文章内容不能为空"))]
    pub content_markdown: Option<String>,
    #[validate(length(min = 1, max = 50, message = "分类长度必须在1到50个字符之间"))]
    pub category: Option<String>,
    pub is_published: Option<bool>,
    pub cover_url: Option<String>,
}

/// 搜索/分页获取文章列表请求体：GET /posts
#[derive(Debug, Serialize, Deserialize)]
pub struct PostListRequest {
    #[serde(default = "default_page")] // 默认页码 1
    pub page: Option<u64>,
    #[serde(default = "default_limit")] // 默认每页数量 10
    pub limit: Option<u64>,
    pub category: Option<String>, // 按分类过滤
    pub query: Option<String>,   // 搜索关键词
    pub published_only: Option<bool>, // 只看已发布的文章，管理员可能查看所有
    pub author_id: Option<i32>, // 按作者过滤，用于草稿箱功能
}

// 辅助函数：提供默认值
fn default_page() -> Option<u64> { Some(1) }
fn default_limit() -> Option<u64> { Some(10) }

/// 文章列表响应体
#[derive(Debug, Serialize)]
pub struct PostListResponse {
    pub success: bool,
    pub posts: Vec<PostInfo>,
    pub total_pages: u64,
    pub current_page: u64,
    pub total_posts: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}


// ======================== 帖子收藏相关 DTOs ========================

/// 切换收藏状态请求体：POST /post_fav
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ToggleFavoriteRequest {
    #[validate(range(min = 1, message = "文章ID无效"))]
    pub post_id: i32,
}

/// 帖子收藏列表响应体：GET /post_fav/my/list
#[derive(Debug, Serialize)]
pub struct FavoriteListResponse {
    pub success: bool,
    pub favorites: Vec<PostInfo>, // 返回收藏的文章信息
    pub total_pages: u64,
    pub current_page: u64,
    pub total_favorites: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}


// ======================== 评论相关 DTOs ========================

/// 评论信息结构体 (用于响应)
#[derive(Debug, Serialize)]
pub struct CommentInfo {
    pub id: i32,
    pub content: String,
    pub post_id: i32,
    pub user_id: i32,
    pub parent_id: Option<i32>, // 父评论 ID，可选
    pub created_at: NaiveDateTime,
    // 可以在这里添加用户信息，例如：
    // pub username: String,
}

/// 创建评论请求体：POST /comments
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(min = 1, max = 1000, message = "评论内容长度必须在1到1000个字符之间"))]
    pub content: String,
    #[validate(range(min = 1, message = "文章ID无效"))]
    pub post_id: i32,            // 评论的文章 ID
    pub parent_id: Option<i32>,  // 父评论 ID，可选，用于嵌套评论
}

/// 更新评论请求体：PUT /comments/:id
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateCommentRequest {
    #[validate(length(min = 1, max = 1000, message = "评论内容长度必须在1到1000个字符之间"))]
    pub content: String,
    // 评论 ID 通过路径参数传递
}

/// 评论列表请求体：GET /posts/:post_id/comments
/// 必须指定文章 ID 来获取评论列表。
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentListRequest {
    // post_id 通常通过 Path 提取器获得，这里可以移除或作为可选字段
    // pub post_id: i32, // 必须指定文章ID来获取评论
    #[serde(default = "default_page")]
    pub page: Option<u64>,
    #[serde(default = "default_limit")]
    pub limit: Option<u64>,
}

/// 评论列表响应体
#[derive(Debug, Serialize)]
pub struct CommentListResponse {
    pub success: bool,
    pub comments: Vec<CommentInfo>,
    pub total_pages: u64,
    pub current_page: u64,
    pub total_comments: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}


// ======================== 通用响应 DTOs ========================

/// 基本响应结构体，用于表示操作成功或失败，可带可选消息。
#[derive(Debug, Serialize)]
pub struct BaseResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")] // 如果为 None，则不序列化该字段
    pub message: Option<String>,
}

/// 带 ID 的基本响应结构体，通常用于创建资源后返回新资源的 ID。
#[derive(Debug, Serialize)]
pub struct IdResponse {
    pub success: bool,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

// 为 BaseResponse 实现 IntoResponse trait
impl IntoResponse for BaseResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

// 为 IdResponse 实现 IntoResponse trait
impl IntoResponse for IdResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

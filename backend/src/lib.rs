use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{NaiveDateTime, Utc}; // 用于时间戳，与数据库中的 TEXT 格式对应

// Axum Blog Engine 的核心库。
// 定义了应用程序的模块结构、共享状态和通用类型。

// ======================== 模块声明 ========================
// 这些模块包含了应用程序的主要功能逻辑。
// `pub` 关键字使得这些模块可以在 crate 外部（例如 `src/bin/server.rs`）被访问。
pub mod database; // 包含数据库实体和 Repository 实现
pub mod handler;  // 包含 HTTP 请求处理器、DTOs 和认证逻辑
pub mod error;    // 包含自定义错误类型和错误处理


// ======================== 重新导出通用类型 ========================
// 从 `error` 模块重新导出 `CustomError` 和 `CustomResult`，
// 这样在整个 crate 的任何地方都可以直接 `use crate::{CustomError, CustomResult};` 访问。
pub use error::{CustomError, CustomResult};


// ======================== 应用状态和配置 ========================
// 导入 `std::sync::Arc` 用于共享不可变数据。
use std::sync::Arc;
// 导入 SeaORM 的数据库连接类型。
use sea_orm::DatabaseConnection;

// 导入各个 Repository 类型，它们将作为 `AppState` 的字段。
use crate::database::user::UserRepository;
use crate::database::post::PostRepository;
use crate::database::comment::CommentRepository;
use crate::database::favorite::FavoriteRepository;

/// 应用程序的共享配置。
/// 通常用于存储从环境变量读取的密钥、成本因子或其他应用级别的常量。
/// 使用 `#[derive(Clone)]` 允许在整个应用中方便地复制和传递。
#[derive(Clone)]
pub struct Config {
    pub jwt_secret: String, // 用于 JWT 签名和验证的密钥
    pub bcrypt_cost: u32,   // Bcrypt 密码哈希的计算成本
    // 您可以在此添加其他全局配置项，例如 API 密钥、外部服务 URL 等。
    // pub external_api_key: String,
}

/// 应用程序的共享状态。
/// 包含了数据库连接和所有 Repository 实例，
/// 使得各个 Axum 处理函数可以通过 `axum::extract::State` 提取器方便地访问这些资源。
/// 使用 `#[derive(Clone)]` 允许 Axum 在处理请求时复制此状态。
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection, // SeaORM 的数据库连接池
    pub config: Arc<Config>,    // 应用程序配置的共享引用
    pub user_repo: UserRepository,      // 用户数据操作仓库
    pub post_repo: PostRepository,      // 文章数据操作仓库
    pub comment_repo: CommentRepository, // 评论数据操作仓库
    pub favorite_repo: FavoriteRepository, // 收藏数据操作仓库
}

// ======================== 其他可能需要的全局导出 ========================
// 如果您希望 `handler/idl.rs` 中定义的 DTOs 在 crate 的根目录也能直接访问，
// 可以使用 `pub use handler::idl::*;` 进行重新导出。
// 但通常，直接通过 `crate::handler::idl::YourDto` 访问已经足够清晰。
// 例如，`use crate::handler::idl::{RegisterRequest, LoginRequest};`。

// ======================== 用户相关 ========================

// 用户注册请求
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 20), /* 额外校验，如只允许字母数字 */)]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 6), /* 额外校验，如包含特殊字符、数字等 */)]
    pub password: String, // 原始密码，后端会进行哈希
}

// 用户注册响应
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub token: Option<String>, // 注册成功后直接返回JWT Token
    pub user_id: i32,
    pub message: Option<String>, // 错误或成功消息
}

// 用户登录请求
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    pub username_or_email: String, // 用户名或邮箱
    pub password: String,          // 原始密码
}

// 用户登录响应
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub token: Option<String>, // 登录成功后返回JWT Token
    pub user_info: Option<UserInfo>, // 登录成功后返回用户信息
    pub message: Option<String>, // 错误或成功消息
}

// 用户信息结构体（用于响应）
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
    // created_at 是 TEXT 类型在数据库中，chrono::NaiveDateTime 更合适
    // 如果直接在响应中返回字符串，这里可以保持 String
    pub created_at: NaiveDateTime, 
}

// 获取登录用户信息响应 (GET /user/get/me)
#[derive(Debug, Serialize)]
pub struct UserInfoResponse {
    pub success: bool,
    pub user: Option<UserInfo>,
    pub message: Option<String>,
}

// 更新用户个人信息请求 (PUT /user/update/my)
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateMyProfileRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: Option<String>, // 可以选择性更新
    #[validate(email)]
    pub email: Option<String>,    // 可以选择性更新
    #[validate(length(min = 6))]
    pub password: Option<String>, // 新密码，会进行哈希
}

// 更新用户（管理员）请求 (PUT /user/update/:id)
// 注意：管理员可能需要更新用户的角色
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 6))]
    pub password: Option<String>,
    pub role: Option<String>, // 管理员可以修改角色
}

// 创建用户（管理员）请求 (POST /user/create)
// 与 RegisterRequest 类似，但可能由管理员使用
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserByAdminRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub role: String, // 管理员创建时指定角色
}

// 删除用户（管理员）请求 (DELETE /user/delete/:id)
// 通常不需要请求体，通过路径参数传递ID
// 这里仅为示例，如果需要批量删除，可以包含ID列表
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserRequest {
    pub user_ids: Vec<i32>,
}


// ======================== 文章相关 ========================

// 文章信息结构体 (用于响应，包含数据库字段)
#[derive(Debug, Serialize)]
pub struct PostInfo {
    pub id: i32,
    pub title: String,
    pub content_markdown: String, // 存储 Markdown 内容
    pub category: String,         // 文章分类
    pub author_id: i32,
    pub is_published: bool,       // 0/1 对应 false/true
    pub view_count: i32,
    pub cover_url: Option<String>, // 封面URL可能是None
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // 可以在这里添加作者的简要信息，例如：
    // pub author_username: String, 
}

// 创建文章请求 (POST /post/create)
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content_markdown: String,
    #[validate(length(min = 1, max = 50))]
    pub category: String,
    pub is_published: Option<bool>, // 可选，默认为 false (草稿)
    pub cover_url: Option<String>,
}

// 更新文章请求 (PUT /post/update/:id 或 /post/edit/:id)
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePostRequest {
    // 文章ID通常通过路径参数传递
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub content_markdown: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub category: Option<String>,
    pub is_published: Option<bool>,
    pub cover_url: Option<String>,
}

// 搜索/分页获取文章列表请求 (GET /post/search, GET /post/list)
#[derive(Debug, Serialize, Deserialize)]
pub struct PostListRequest {
    pub page: Option<u64>,       // 页码
    pub limit: Option<u64>,      // 每页数量
    pub category: Option<String>, // 按分类过滤
    pub query: Option<String>,   // 搜索关键词 (用于 /post/search)
    pub published_only: Option<bool>, // 只看已发布的文章，管理员可能查看所有
}

// 文章列表响应
#[derive(Debug, Serialize)]
pub struct PostListResponse {
    pub success: bool,
    pub posts: Vec<PostInfo>,
    pub total_pages: u64,
    pub current_page: u64,
    pub total_posts: u64,
}


// ======================== 帖子收藏相关 ========================

// 收藏/取消收藏文章请求 (POST /post_fav)
#[derive(Debug, Serialize, Deserialize)]
pub struct ToggleFavoriteRequest {
    pub post_id: i32,
}

// 帖子收藏列表响应 (GET /post_fav/my/list)
#[derive(Debug, Serialize)]
pub struct FavoriteListResponse {
    pub success: bool,
    pub favorites: Vec<PostInfo>, // 返回收藏的文章信息
    pub total_pages: u64,
    pub current_page: u64,
    pub total_favorites: u64,
}


// ======================== 评论相关 ========================

// 评论信息结构体 (用于响应)
#[derive(Debug, Serialize)]
pub struct CommentInfo {
    pub id: i32,
    pub content: String,
    pub post_id: i32,
    pub user_id: i32,
    pub parent_id: Option<i32>, // 父评论ID，可选
    pub created_at: NaiveDateTime,
    // 可以在这里添加用户信息，例如：
    // pub username: String,
}

// 创建评论请求 (POST /comment/create)
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(min = 1, max = 1000))]
    pub content: String,
    pub post_id: i32,            // 评论的文章ID
    pub parent_id: Option<i32>,  // 父评论ID，可选
}

// 更新评论请求 (PUT /comment/update/:id)
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateCommentRequest {
    #[validate(length(min = 1, max = 1000))]
    pub content: String,
    // 评论ID通过路径参数传递
}

// 评论列表请求 (GET /comment/list)
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentListRequest {
    pub post_id: i32, // 必须指定文章ID来获取评论
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

// 评论列表响应
#[derive(Debug, Serialize)]
pub struct CommentListResponse {
    pub success: bool,
    pub comments: Vec<CommentInfo>,
    pub total_pages: u64,
    pub current_page: u64,
    pub total_comments: u64,
}


// ======================== 通用响应 ========================

// 基本响应结构体，用于表示操作成功或失败
#[derive(Debug, Serialize)]
pub struct BaseResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")] // 如果为None，则不序列化该字段
    pub message: Option<String>,
}

// 带ID的基本响应
#[derive(Debug, Serialize)]
pub struct IdResponse {
    pub success: bool,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

//! 定义应用程序的自定义错误类型及其 Axum 响应转换。

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::error; // 引入 error 宏

/// 定义一个通用结果类型，所有返回 Result 的函数都应使用此类型。
pub type CustomResult<T> = Result<T, CustomError>;

/// 应用程序的自定义错误枚举。
/// 使用 `thiserror` 宏简化错误定义和实现 `std::error::Error` trait。
#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    /// 内部服务器错误，通常表示后端逻辑或系统级别的问题。
    #[error("内部服务器错误: {0}")]
    InternalServerError(String),

    /// 请求无效，通常由客户端发送的请求数据不符合预期引起。
    #[error("请求无效: {0}")]
    BadRequest(String),

    /// 未授权错误，通常表示用户未登录或提供的认证凭据无效。
    #[error("未授权: {0}")]
    Unauthorized(String), // 包含具体未授权原因

    /// 禁止访问错误，表示用户已登录但没有执行特定操作的权限。
    #[error("禁止访问: {0}")]
    Forbidden(String), // 包含具体禁止原因，例如“无权操作”或“非管理员权限”

    /// 资源未找到错误，当请求的资源不存在时返回。
    #[error("未找到: {0}")]
    NotFound(String), // 包含具体未找到的资源信息

    /// 数据库操作失败，封装 SeaORM 的数据库错误。
    #[error("数据库错误: {0}")]
    DbError(#[from] sea_orm::DbErr), // 使用 `#[from]` 实现自动从 `sea_orm::DbErr` 转换

    /// 密码哈希或验证失败。
    #[error("密码处理错误: {0}")]
    BcryptError(String), // 直接存储字符串，因为 bcrypt::BcryptError 不直接实现 Display

    /// JWT 生成、解析或验证失败。
    #[error("认证令牌错误: {0}")]
    JwtError(String), // 直接存储字符串，因为 jsonwebtoken::errors::Error 不直接实现 Display

    /// 数据验证失败，由 `validator` 库生成。
    #[error("数据验证失败: {0}")]
    ValidationError(#[from] validator::ValidationErrors), // 使用 `#[from]` 实现自动从 `validator::ValidationErrors` 转换

    // 您可以根据需要添加更多具体的错误类型，例如：
    // #[error("文件上传失败: {0}")]
    // FileUploadError(String),
}

// 为 `bcrypt::BcryptError` 实现 `From<bcrypt::BcryptError>` 到 `CustomError` 的转换
// 这是因为 `bcrypt::BcryptError` 不直接实现 `std::fmt::Display`，
// 所以我们不能直接使用 `#[from]`，需要手动将其转换为字符串。
impl From<bcrypt::BcryptError> for CustomError {
    fn from(err: bcrypt::BcryptError) -> Self {
        CustomError::BcryptError(err.to_string())
    }
}

// 为 `jsonwebtoken::errors::Error` 实现 `From<jsonwebtoken::errors::Error>` 到 `CustomError` 的转换
// 同上，因为 `jsonwebtoken::errors::Error` 不直接实现 `std::fmt::Display`。
impl From<jsonwebtoken::errors::Error> for CustomError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        CustomError::JwtError(err.to_string())
    }
}


/// 为 `CustomError` 实现 `axum::response::IntoResponse` Trait。
/// 这使得我们可以直接从 Axum 处理函数中返回 `Result<(), CustomError>`，
/// Axum 会自动将 `CustomError` 转换为 HTTP 响应。
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        // 根据 CustomError 的变体，确定 HTTP 状态码和具体的错误信息
        let (status, message) = match self {
            CustomError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            CustomError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            CustomError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            CustomError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            CustomError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            CustomError::DbError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库操作失败: {}", e)),
            CustomError::BcryptError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("密码处理错误: {}", e)),
            CustomError::JwtError(e) => (StatusCode::UNAUTHORIZED, format!("认证令牌错误: {}", e)),
            CustomError::ValidationError(e) => (StatusCode::BAD_REQUEST, format!("数据验证失败: {}", e)),
        };

        // 记录错误日志，便于调试
        error!("API 错误: 状态码={}, 消息='{}'", status, message);

        // 构建 JSON 响应体，遵循 BaseResponse 格式
        let body = Json(json!({
            "success": false,
            "message": message,
        }));

        // 返回 HTTP 响应
        (status, body).into_response()
    }
}

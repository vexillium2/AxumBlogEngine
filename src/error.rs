use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type CustomResult<T> = Result<T, CustomError>;

#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    // ...其他错误...
    
    #[error("认证失败")]
    AuthError,
    
    #[error("无权访问")]
    Forbidden,
    
    #[error("用户不存在")]
    UserNotFound,
    
    #[error("用户已存在")]
    UserAlreadyExists,
    
    #[error("邮箱已存在")]
    EmailAlreadyExists,
    
    #[error("数据库错误: {0}")]
    DbError(#[from] sea_orm::DbErr),
    
    #[error("密码哈希错误: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),
    
    #[error("JWT错误: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    
    #[error("验证错误: {0}")]
    ValidationError(String),
}

impl From<validator::ValidationErrors> for CustomError {
    fn from(err: validator::ValidationErrors) -> Self {
        Self::ValidationError(err.to_string())
    }
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let code: StatusCode = match self {
            CustomError::AuthError => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let reason = self.to_string();
        tracing::error!("{} {}", code, reason);
        (code, reason).into_response()
    }
}

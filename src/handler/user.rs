use axum::{extract::State, Json};
use validator::Validate;

use crate::{
    database::user,
    error::{CustomError, CustomResult},
    handler::{
        auth::{generate_jwt, Claims},
        idl::*,
    },
    AppState,
};

/// 用户注册
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> CustomResult<Json<RegisterResponse>> {
    // 验证输入
    req.validate()?;

    // 检查用户名和邮箱是否已存在
    if user::find_by_username(&req.username).await?.is_some() {
        return Err(CustomError::UserAlreadyExists);
    }
    if user::find_by_email(&req.email).await?.is_some() {
        return Err(CustomError::EmailAlreadyExists);
    }

    // 密码哈希处理
    let password_hash = bcrypt::hash(&req.password, state.config.bcrypt_cost)?;

    // 创建用户
    let user = user::create(&req.username, &req.email, &password_hash, None)
        .await?;

    // 生成JWT
    let token = generate_jwt(&Claims {
        sub: user.id.to_string(),
        username: user.username,
        role: user.role,
        exp: (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as usize,
    })?;

    Ok(Json(RegisterResponse {
        success: true,
        token,
        user_id: user.id,
    }))
}

/// 用户登录
pub async fn login(
    State(_state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> CustomResult<Json<LoginResponse>> {
    req.validate()?;

    // 查找用户（支持用户名/邮箱登录）
    let user = if req.username_or_email.contains('@') {
        user::find_by_email(&req.username_or_email)
            .await?
            .ok_or(CustomError::AuthError)?
    } else {
        user::find_by_username(&req.username_or_email)
            .await?
            .ok_or(CustomError::AuthError)?
    };

    // 验证密码
    if !bcrypt::verify(&req.password, &user.password_hash)? {
        tracing::warn!("密码验证失败: user_id={}", user.id);
        return Err(CustomError::AuthError);
    }

    // 生成JWT - 使用引用或克隆字符串
    let token = generate_jwt(&Claims {
        sub: user.id.to_string(),
        username: user.username.clone(),  // 使用.clone()复制字符串
        role: user.role.clone(),         // 使用.clone()复制字符串
        exp: (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as usize,
    })?;

    Ok(Json(LoginResponse {
        success: true,
        token,
        user_info: UserInfo {
        id: user.id,
        username: user.username,    // 这里可以继续使用原值
        email: user.email,
        role: user.role,            // 这里可以继续使用原值
        created_at: user.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
    },
    }))
}

/// 获取当前用户信息
pub async fn get_current_user(
    claims: Claims,
) -> CustomResult<Json<UserInfoResponse>> {
    let user = user::find_by_username(&claims.username)
        .await?
        .ok_or(CustomError::UserNotFound)?;

    Ok(Json(UserInfoResponse {
        success: true,
        user: UserInfo {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
            created_at: user.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        },
    }))
}

pub async fn update_user(
    claims: Claims,
    State(state): State<AppState>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<BaseResponse>, CustomError> {
    req.validate()?;

    // 权限检查
    if claims.role != "admin" && claims.sub != req.user_id.to_string() {
        return Err(CustomError::Forbidden);
    }

    // 实际更新逻辑
    let mut update_data = user::ActiveModel {
        id: sea_orm::Set(req.user_id),
        ..Default::default()
    };

    if let Some(username) = req.new_username {
        update_data.username = sea_orm::Set(username);
    }
    if let Some(email) = req.new_email {
        update_data.email = sea_orm::Set(email);
    }
    if let Some(password) = req.new_password {
        update_data.password_hash = sea_orm::Set(bcrypt::hash(password, state.config.bcrypt_cost)?);
    }

    user::update(update_data).await?;

    Ok(Json(BaseResponse {
        success: true,
        message: Some("用户信息更新成功".to_string()),
    }))
}
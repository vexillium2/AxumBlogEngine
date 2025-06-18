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
        username: user.username.clone(),
        role: user.role,
    })?;

    Ok(Json(RegisterResponse {
        success: true,
        token,
        user_id: user.id,
    }))
}

/// 用户登录
pub async fn login(
    State(state): State<AppState>,
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

    // 生成JWT
    let token = generate_jwt(&Claims {
        sub: user.id.to_string(),
        username: user.username,
        role: user.role,
    })?;

    Ok(Json(LoginResponse {
        success: true,
        token,
        user_info: UserInfo {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
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

/// 更新用户信息
pub async fn update_user(
    claims: Claims,
    Json(req): Json<UpdateUserRequest>,
) -> CustomResult<Json<BaseResponse>> {
    req.validate()?;

    // 只有管理员或用户本人可以修改
    if claims.role != "admin" && claims.sub != req.user_id.to_string() {
        return Err(CustomError::Forbidden);
    }

    // 实现更新逻辑...
    Ok(Json(BaseResponse { success: true }))
}
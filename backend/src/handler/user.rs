//! 用户相关的 HTTP 请求处理函数。
//! 遵循 RESTful API 设计风格。

use axum::{
    extract::{Path, State, Json, FromRequestParts, FromRef},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
    routing::{post, get, put, delete},
    Router,
};
use serde_json::json;
use validator::Validate;
use bcrypt::{hash, verify, DEFAULT_COST}; // 用于密码哈希和验证
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Algorithm}; // 用于 JWT
use chrono::{Utc, Duration}; // 用于时间戳
use tracing::{debug, info, error, warn}; // 日志

// 导入 crate 根目录下的 AppState 和 Config
use crate::{AppState, Config};
// 导入数据库模块的 UserRepository 和用户实体模型
use crate::database::user::{UserRepository, Model as UserModel}; // 引入用户实体模型和 UserRepository
// 导入 IDL 中定义的请求和响应 DTOs
use crate::handler::idl::{
    RegisterRequest, RegisterResponse, LoginRequest, LoginResponse, UserInfo, UserInfoResponse,
    UpdateMyProfileRequest, UpdateUserRequest, CreateUserByAdminRequest, DeleteUserRequest,
    BaseResponse, IdResponse,
};
// 导入自定义错误类型和通用 Result
use crate::error::{CustomError, CustomResult};
// 导入认证模块中的Claims
use crate::handler::auth::Claims;

/// JWT 认证提取器
/// 这个提取器会尝试从请求头中解析 JWT token，并验证其有效性。
/// 如果认证成功，它将提供 `Claims` 给处理函数。
///
/// 注意：实际应用中，这种提取器通常会放在 `handler/auth.rs` 中，
/// 或者作为 Axum 的 `Layer` (中间件) 来实现更全面的认证流程。
pub struct AuthUser(pub Claims);

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>, // 确保 S 可以转换为 AppState 的引用
    S: Send + Sync,
{
    type Rejection = CustomError; // 定义拒绝类型为自定义错误

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state: AppState = AppState::from_ref(state); // 从 state 转换为 AppState
        let token = parts.headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer ")) // 期望 "Bearer <token>" 格式
            .ok_or(CustomError::Unauthorized("缺少或无效的Authorization头".to_string()))?;

        let decoding_key = DecodingKey::from_secret(app_state.config.jwt_secret.as_bytes());

        // 使用默认验证规则解码 JWT
        let token_data = jsonwebtoken::decode::<Claims>(token, &decoding_key, &jsonwebtoken::Validation::default())
            .map_err(|e| {
                debug!("JWT解码失败: {:?}", e); // 详细日志用于调试
                CustomError::JwtError(format!("JWT令牌无效: {}", e))
            })?;

        Ok(AuthUser(token_data.claims))
    }
}

/// 将用户 `Model` 转换为 `UserInfo` DTO。
/// 这是一个辅助函数，用于将数据库实体映射为 API 响应的数据结构。
fn convert_user_model_to_user_info(model: UserModel) -> UserInfo {
    UserInfo {
        id: model.id,
        username: model.username,
        email: model.email,
        role: model.role,
        created_at: model.created_at, // `NaiveDateTime` 直接映射
    }
}

/// 生成 JWT Token。
/// 此函数负责创建 JWT 字符串，通常会放在 `handler/auth.rs` 模块中。
///
/// # 参数
/// * `sub` - JWT subject，通常是用户ID的字符串形式。
/// * `username` - JWT 中包含的用户名。
/// * `role` - JWT 中包含的用户角色。
/// * `exp_timestamp` - JWT 过期时间戳 (Unix timestamp)。
/// * `jwt_secret` - 用于签名的JWT密钥。
///
/// # 返回
/// 包含生成的 JWT 字符串的 `Result`。
pub fn generate_jwt(
    sub: &str,
    username: &str,
    role: &str,
    exp_timestamp: usize,
    jwt_secret: &str,
) -> Result<String, CustomError> {
    let claims = Claims {
        sub: sub.to_string(),
        username: username.to_string(),
        role: role.to_string(),
        exp: exp_timestamp,
    };

    let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    encode(&Header::new(Algorithm::HS256), &claims, &encoding_key)
        .map_err(|e| CustomError::JwtError(format!("JWT生成失败: {}", e)))
}


// ======================== 用户相关 API 处理函数 (RESTful 风格) ========================

/// 处理用户注册请求 (POST /users/register)
/// 创建一个新用户并返回 JWT Token。
pub async fn register_user(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> CustomResult<Json<RegisterResponse>> {
    req.validate()?; // 验证请求数据

    // 检查用户名是否已存在
    if state.user_repo.get_user_by_username_or_email(&req.username).await?.is_some() {
        return Err(CustomError::BadRequest("用户名已存在".to_string()));
    }
    // 检查邮箱是否已存在
    if state.user_repo.get_user_by_username_or_email(&req.email).await?.is_some() {
        return Err(CustomError::BadRequest("邮箱已存在".to_string()));
    }

    // 哈希密码
    let hashed_password = hash(&req.password, state.config.bcrypt_cost)
        .map_err(|e| CustomError::BcryptError(e.to_string()))?;

    // 创建用户，默认角色为 'user'
    let created_user = state.user_repo.create_user_from_register(req, hashed_password).await?;

    // 生成 JWT Token
    let token = generate_jwt(
        &created_user.id.to_string(),
        &created_user.username,
        &created_user.role,
        (Utc::now() + Duration::days(30)).timestamp() as usize, // Token 有效期 30 天
        &state.config.jwt_secret,
    )?;

    Ok(Json(RegisterResponse {
        success: true,
        token: Some(token), // 成功返回 Some(token)
        user_id: created_user.id,
        message: Some("用户注册成功".to_string()),
    }))
}

/// 处理用户登录请求 (POST /users/login)
/// 验证用户凭据并返回 JWT Token 和用户信息。
pub async fn login_user(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> CustomResult<Json<LoginResponse>> {
    req.validate()?;

    // 查找用户（支持用户名或邮箱登录）
    let user_model = state.user_repo.get_user_by_username_or_email(&req.username_or_email).await?
        .ok_or(CustomError::Unauthorized("用户名或密码错误".to_string()))?;

    // 验证密码
    if !verify(&req.password, &user_model.password_hash).map_err(|e| CustomError::BcryptError(e.to_string()))? {
        warn!("密码验证失败: user_id={}", user_model.id); // 记录警告日志
        return Err(CustomError::Unauthorized("用户名或密码错误".to_string()));
    }

    // 生成 JWT Token
    let token = generate_jwt(
        &user_model.id.to_string(),
        &user_model.username,
        &user_model.role,
        (Utc::now() + Duration::days(30)).timestamp() as usize,
        &state.config.jwt_secret,
    )?;

    Ok(Json(LoginResponse {
        success: true,
        token: Some(token), // 成功返回 Some(token)
        user_info: Some(convert_user_model_to_user_info(user_model)),
        message: Some("登录成功".to_string()),
    }))
}

/// 处理用户登出请求 (POST /users/logout)
/// 理论上，JWT 是无状态的，登出通常意味着客户端删除本地存储的 token。
/// 后端可以实现一个 token 黑名单机制来禁用已登出的 token，但这里只是一个示例占位符。
pub async fn logout_user() -> CustomResult<Json<BaseResponse>> {
    // 实际 JWT 登出通常在客户端进行（删除本地token）
    // 服务端可以实现token黑名单，但超出了本示例范围。
    Ok(Json(BaseResponse {
        success: true,
        message: Some("登出成功".to_string()),
    }))
}


/// 获取当前登录用户信息 (GET /users/me)
/// 接收 `AuthUser` 提取器提供的已认证用户 Claims。
pub async fn get_me(
    AuthUser(claims): AuthUser, // 使用 AuthUser 提取器获取 Claims
    State(state): State<AppState>,
) -> CustomResult<Json<UserInfoResponse>> {
    let user_id = claims.sub.parse::<i32>()
        .map_err(|_| CustomError::Unauthorized("JWT令牌中的用户ID无效".to_string()))?;
    
    let user_model = state.user_repo.get_user_by_id(user_id).await?
        .ok_or(CustomError::NotFound("用户未找到".to_string()))?; // 如果用户已被删除

    Ok(Json(UserInfoResponse {
        success: true,
        user: Some(convert_user_model_to_user_info(user_model)),
        message: None,
    }))
}

/// 更新当前登录用户个人信息 (PUT /users/me)
/// 允许用户更新自己的用户名、邮箱或密码。
pub async fn update_me(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Json(req): Json<UpdateMyProfileRequest>,
) -> CustomResult<Json<BaseResponse>> {
    req.validate()?; // 验证请求数据

    let user_id = claims.sub.parse::<i32>()
        .map_err(|_| CustomError::Unauthorized("JWT令牌中的用户ID无效".to_string()))?;
    
    let mut new_password_hash: Option<String> = None;
    if let Some(password) = req.password.as_ref() {
        new_password_hash = Some(hash(password, state.config.bcrypt_cost)
            .map_err(|e| CustomError::BcryptError(e.to_string()))?);
    }

    state.user_repo.update_my_profile(user_id, req, new_password_hash).await?;

    Ok(Json(BaseResponse {
        success: true,
        message: Some("个人信息更新成功".to_string()),
    }))
}

// ======================== 管理员用户 API 处理函数 (RESTful 风格) ========================

/// 获取指定用户信息 (GET /users/:id) - 需要管理员权限
/// 根据用户 ID 获取其详细信息。
pub async fn get_user_by_id_admin(
    AuthUser(claims): AuthUser, // 认证并获取用户 Claims
    State(state): State<AppState>,
    Path(user_id): Path<i32>, // 从路径中获取用户 ID
) -> CustomResult<Json<UserInfoResponse>> {
    // 权限检查：只有管理员才能访问此接口
    if claims.role != "admin" {
        return Err(CustomError::Forbidden("无权限访问指定用户信息".to_string()));
    }

    let user_model = state.user_repo.get_user_by_id(user_id).await?
        .ok_or(CustomError::NotFound(format!("ID为 {} 的用户未找到", user_id)))?;

    Ok(Json(UserInfoResponse {
        success: true,
        user: Some(convert_user_model_to_user_info(user_model)),
        message: None,
    }))
}

/// 创建用户 (POST /users) - 需要管理员权限
/// 允许管理员创建新用户，并指定角色。
pub async fn create_user_admin(
    AuthUser(claims): AuthUser, // 认证并获取用户 Claims
    State(state): State<AppState>,
    Json(req): Json<CreateUserByAdminRequest>,
) -> CustomResult<Json<IdResponse>> { // 返回 IdResponse，包含新创建用户的 ID
    // 权限检查：只有管理员才能访问此接口
    if claims.role != "admin" {
        return Err(CustomError::Forbidden("无权限创建用户".to_string()));
    }
    req.validate()?; // 验证请求数据

    // 检查用户名是否已存在
    if state.user_repo.get_user_by_username_or_email(&req.username).await?.is_some() {
        return Err(CustomError::BadRequest("用户名已存在".to_string()));
    }
    // 检查邮箱是否已存在
    if state.user_repo.get_user_by_username_or_email(&req.email).await?.is_some() {
        return Err(CustomError::BadRequest("邮箱已存在".to_string()));
    }

    // 哈希密码
    let hashed_password = hash(&req.password, state.config.bcrypt_cost)
        .map_err(|e| CustomError::BcryptError(e.to_string()))?;

    // 创建用户，角色由请求指定
    let created_user = state.user_repo.create_user_by_admin(req, hashed_password).await?;

    Ok(Json(IdResponse {
        success: true,
        id: created_user.id,
        message: Some("用户创建成功".to_string()),
    }))
}

/// 更新用户信息 (PUT /users/:id) - 需要管理员权限
/// 允许管理员更新任何用户的信息，包括角色。
pub async fn update_user_admin(
    AuthUser(claims): AuthUser, // 认证并获取用户 Claims
    State(state): State<AppState>,
    Path(user_id): Path<i32>, // 从路径中获取要更新的用户 ID
    Json(req): Json<UpdateUserRequest>,
) -> CustomResult<Json<BaseResponse>> {
    // 权限检查：只有管理员才能访问此接口
    if claims.role != "admin" {
        return Err(CustomError::Forbidden("无权限更新用户信息".to_string()));
    }
    req.validate()?; // 验证请求数据

    let mut new_password_hash: Option<String> = None;
    if let Some(password) = req.password.as_ref() {
        new_password_hash = Some(hash(password, state.config.bcrypt_cost)
            .map_err(|e| CustomError::BcryptError(e.to_string()))?);
    }
    
    state.user_repo.update_user_by_admin(user_id, req, new_password_hash).await?;

    Ok(Json(BaseResponse {
        success: true,
        message: Some("用户信息更新成功".to_string()),
    }))
}

/// 删除单个用户 (DELETE /users/:id) - 需要管理员权限
/// 根据用户 ID 删除指定用户。
pub async fn delete_user_admin(
    AuthUser(claims): AuthUser, // 认证并获取用户 Claims
    State(state): State<AppState>,
    Path(user_id): Path<i32>, // 从路径中获取要删除的用户 ID
) -> CustomResult<impl IntoResponse> { // 返回 CustomResult<impl IntoResponse> 以便自定义响应
    // 权限检查：只有管理员才能访问此接口
    if claims.role != "admin" {
        return Err(CustomError::Forbidden("无权限删除用户".to_string()));
    }

    state.user_repo.delete_user(user_id).await?;

    Ok((
        StatusCode::NO_CONTENT, // 204 No Content，表示成功但无返回体
        BaseResponse {
            success: true, // 尽管是 204，响应体仍可以指示 success
            message: Some("用户删除成功".to_string()),
        },
    ))
}

/// 批量删除用户 (DELETE /users) - 需要管理员权限
/// 允许管理员一次性删除多个用户。
pub async fn delete_users_batch_admin(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Json(req): Json<DeleteUserRequest>, // 请求体包含要删除的用户 ID 列表
) -> CustomResult<Json<BaseResponse>> {
    // 权限检查：只有管理员才能访问此接口
    if claims.role != "admin" {
        return Err(CustomError::Forbidden("无权限批量删除用户".to_string()));
    }

    let rows_affected = state.user_repo.delete_users_by_ids(req.user_ids).await?;

    Ok(Json(BaseResponse {
        success: true,
        message: Some(format!("成功删除 {} 个用户", rows_affected)),
    }))
}

/// 用户路由配置函数。
/// 此函数将所有用户相关的路由组合起来，方便在 `src/bin/server.rs` 中集成。
pub fn user_routes() -> Router<AppState> {
    Router::new()
        // 公开的用户认证与个人资料管理路由
        .route("/register", post(register_user))        // POST /users/register
        .route("/login", post(login_user))              // POST /users/login
        .route("/logout", post(logout_user))            // POST /users/logout (或 DELETE /sessions)
        .route("/me", get(get_me))                      // GET /users/me
        .route("/me", put(update_me))                   // PUT /users/me

        // 管理员专用路由 (遵循 RESTful /users/{id} 模式)
        .route("/:id", get(get_user_by_id_admin))       // GET /users/:id (获取指定用户)
        .route("/", post(create_user_admin))            // POST /users (创建用户)
        .route("/:id", put(update_user_admin))          // PUT /users/:id (更新指定用户)
        .route("/:id", delete(delete_user_admin))       // DELETE /users/:id (删除单个用户)
        .route("/", delete(delete_users_batch_admin))   // DELETE /users (批量删除)
}

use serde::{Deserialize, Serialize};
use validator::Validate;

// 注册请求
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub token: String,
    pub user_id: i32,
}

// 登录请求
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub token: String,
    pub user_info: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfoResponse {
    pub success: bool,
    pub user: UserInfo,
}

// 更新用户请求
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    pub user_id: i32,
    #[validate(length(min = 3, max = 20))]
    pub new_username: Option<String>,
    #[validate(email)]
    pub new_email: Option<String>,
    #[validate(length(min = 6))]
    pub new_password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BaseResponse {
    pub success: bool,
    pub message: Option<String>, // 可选消息字段
}
// #[derive(Serialize, Deserialize, Debug, Default)]
// #[serde(default)]
// pub struct Book {
//     pub id: i32,
//     pub name: String,
//     pub operator: String,
//     pub created_at: i32,
//     pub updated_at: i32,
// }

// #[derive(Serialize, Deserialize, Debug, Default)]
// #[serde(default)]
// pub struct CreateBookRequest {
//     pub name: String,
// }
// #[derive(Serialize, Deserialize, Debug, Default)]
// #[serde(default)]
// pub struct CreateBookResponse {
//     pub success: bool,
// }

// #[derive(Serialize, Deserialize, Debug, Default)]
// #[serde(default)]
// pub struct SearchBookRequest {
//     pub query: String,
// }
// #[derive(Serialize, Deserialize, Debug, Default)]
// #[serde(default)]
// pub struct SearchBookResponse {
//     pub books: Vec<Book>,
// }

// #[derive(Serialize, Deserialize, Debug, Default)]
// #[serde(default)]
// pub struct UpdateBookRequest {
//     pub id: i32,
//     pub name: String,
// }
// #[derive(Serialize, Deserialize, Debug, Default)]
// #[serde(default)]
// pub struct UpdateBookResponse {
//     pub success: bool,
// }

// #[derive(Serialize, Deserialize, Debug, Default)]
// #[serde(default)]
// pub struct DeleteBookRequest {
//     pub id: i32,
// }
// #[derive(Serialize, Deserialize, Debug, Default)]
// #[serde(default)]
// pub struct DeleteBookResponse {
//     pub success: bool,
// }

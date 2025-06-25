// src/database/user.rs
//! 用户数据库仓库和实体定义。

// --- SeaORM 实体定义 ---
use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;
// 引入 IntoActiveModel trait，以便使用 Model::into_active_model() 方法
use sea_orm::IntoActiveModel;
// 引入 ActiveModelBehavior trait，虽然通常由 prelude 覆盖，但明确引入有时有帮助
use sea_orm::ActiveModelBehavior;


/// `users` 表的实体定义。
///
/// 此结构体直接映射到数据库中 `users` 表的列。
/// `created_at` 和 `updated_at` 字段使用 `chrono::NaiveDateTime` 类型，
/// 以匹配 SQLite 数据库中日期使用的 `TEXT` 格式，并结合 SeaORM 的 `with-chrono` 特性。
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String, // 存储哈希后的密码
    pub role: String, // 'user' 或 'admin'
    pub created_at: NaiveDateTime, // 数据库默认自动填充
    pub updated_at: NaiveDateTime, // 数据库默认自动填充
}

/// 定义 `users` 实体的关系。
/// 注意：这里的 `super::post::Entity` 等路径假设 `post.rs`、`comment.rs`、`favorite.rs` 也直接定义了其实体。
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::post::Entity")] // 引用 `src/database/post.rs` 中的实体
    Post,
    #[sea_orm(has_many = "super::comment::Entity")] // 引用 `src/database/comment.rs` 中的实体
    Comment,
    #[sea_orm(has_many = "super::favorite::Entity")] // 引用 `src/database/favorite.rs` 中的实体
    Favorite,
}

// 为关系实现 Related Trait
// 注意：如果 `super::post::Entity` 等相关实体未定义或未正确导入，这里可能会报错。
// 请确保 `post.rs`, `comment.rs`, `favorite.rs` 文件都已创建并定义了其 `Model` 和 `Entity`。
impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}

impl Related<super::favorite::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Favorite.def()
    }
}

/// 定义 `users` 实体的 `ActiveModel`。
///
/// 这用于在数据库中创建、更新和删除记录。
/// SeaORM 的 `DeriveIntoActiveModel` 和 `DeriveActiveModel` 宏将自动生成所需代码。
/// **修复:** 移除了所有会与 `DeriveActiveModel` 内部实现冲突的 `derive` 属性（如 `Debug`, `PartialEq`, `Eq`）。
/// **修复:** 确保 `pub struct ActiveModel;` 只出现一次。
impl ActiveModelBehavior for ActiveModel {} 


// --- UserRepository (数据库操作) ---
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    PaginatorTrait, QuerySelect
};
// 导入 `anyhow::Result` 已经足够，不需要单独的 `bail`，可以直接用 `?`
use anyhow::Result; // 尽管在 CustomError 中不再直接使用 anyhow::Error，这里仍然可以用于更通用的 Result
use chrono::Utc;

// 从 handler 模块导入 DTO
use crate::handler::idl::{
    CreateUserByAdminRequest, RegisterRequest, UpdateMyProfileRequest, UpdateUserRequest, UserInfo,
};
// 导入自定义错误类型
use crate::error::CustomError;


/// `UserRepository` 提供与 `users` 表交互的方法。
/// 它持有数据库连接并执行 CRUD 操作。
#[derive(Clone)]
pub struct UserRepository {
    db: DatabaseConnection,
}

impl UserRepository {
    /// 创建一个新的 `UserRepository` 实例。
    pub fn new(db: DatabaseConnection) -> Self {
        UserRepository { db }
    }

    /// 基于 `RegisterRequest` 创建新用户。
    ///
    /// 密码在此方法调用前**必须**已被哈希处理。
    ///
    /// # 参数
    /// * `req` - 包含用户详细信息的 `RegisterRequest`。
    /// * `password_hash` - 预先哈希好的密码字符串。
    ///
    /// # 返回
    /// 包含创建的用户 `Model` 或 `CustomError` 的 `Result`。
    pub async fn create_user_from_register(
        &self,
        req: RegisterRequest,
        password_hash: String,
    ) -> Result<Model, CustomError> { // 使用 CustomError
        let now = Utc::now().naive_utc();
        let active_model = ActiveModel {
            username: Set(req.username),
            email: Set(req.email),
            password_hash: Set(password_hash),
            role: Set("user".to_owned()), // 注册用户的默认角色
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default() // 填充其他字段的默认值
        };

        let user = active_model.insert(&self.db).await?;
        Ok(user)
    }

    /// 基于 `CreateUserByAdminRequest` 创建新用户。
    ///
    /// 密码在此方法调用前**必须**已被哈希处理。
    ///
    /// # 参数
    /// * `req` - 包含用户详细信息的 `CreateUserByAdminRequest`。
    /// * `password_hash` - 预先哈希好的密码字符串。
    ///
    /// # 返回
    /// 包含创建的用户 `Model` 或 `CustomError` 的 `Result`。
    pub async fn create_user_by_admin(
        &self,
        req: CreateUserByAdminRequest,
        password_hash: String,
    ) -> Result<Model, CustomError> { // 使用 CustomError
        let now = Utc::now().naive_utc();
        let active_model = ActiveModel {
            username: Set(req.username),
            email: Set(req.email),
            password_hash: Set(password_hash),
            role: Set(req.role), // 管理员可以指定角色
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let user = active_model.insert(&self.db).await?;
        Ok(user)
    }

    /// 根据用户 ID 检索用户。
    ///
    /// # 参数
    /// * `user_id` - 要检索的用户 ID。
    ///
    /// # 返回
    /// 包含 `Option<Model>` 或 `CustomError` 的 `Result`。
    pub async fn get_user_by_id(&self, user_id: i32) -> Result<Option<Model>, CustomError> { // 使用 CustomError
        let user = Entity::find_by_id(user_id).one(&self.db).await?;
        Ok(user)
    }

    /// 根据用户名或邮箱检索用户。
    /// 主要用于登录。
    ///
    /// # 参数
    /// * `username_or_email` - 用户的用户名或邮箱。
    ///
    /// # 返回
    /// 包含 `Option<Model>` 或 `CustomError` 的 `Result`。
    pub async fn get_user_by_username_or_email(
        &self,
        username_or_email: &str,
    ) -> Result<Option<Model>, CustomError> { // 使用 CustomError
        let user = Entity::find()
            .filter(
                Column::Username
                    .eq(username_or_email)
                    .or(Column::Email.eq(username_or_email)),
            )
            .one(&self.db)
            .await?;
        Ok(user)
    }

    /// 根据 `UpdateMyProfileRequest` 更新用户个人资料。
    ///
    /// # 参数
    /// * `user_id` - 要更新的用户 ID。
    /// * `req` - 包含可选更新字段的 `UpdateMyProfileRequest`。
    /// * `new_password_hash` - 如果密码需要更新，则为可选的新密码哈希。
    ///
    /// # 返回
    /// 包含更新后的用户 `Model` 或 `CustomError` 的 `Result`。
    pub async fn update_my_profile(
        &self,
        user_id: i32,
        req: UpdateMyProfileRequest,
        new_password_hash: Option<String>,
    ) -> Result<Model, CustomError> { // 使用 CustomError
        let user = Entity::find_by_id(user_id).one(&self.db).await?;
        let mut user: ActiveModel = match user {
            Some(u) => u.into_active_model(),
            None => return Err(CustomError::NotFound("用户未找到".to_string())), // 使用 CustomError::NotFound
        };

        if let Some(username) = req.username {
            user.username = Set(username);
        }
        if let Some(email) = req.email {
            user.email = Set(email);
        }
        if let Some(password_hash) = new_password_hash {
            user.password_hash = Set(password_hash);
        }
        user.updated_at = Set(Utc::now().naive_utc()); // 更新时间戳

        let updated_user = user.update(&self.db).await?;
        Ok(updated_user)
    }

    /// 由管理员根据 `UpdateUserRequest` 更新用户信息。
    ///
    /// # 参数
    /// * `user_id` - 要更新的用户 ID。
    /// * `req` - 包含可选更新字段的 `UpdateUserRequest`。
    /// * `new_password_hash` - 如果密码需要更新，则为可选的新密码哈希。
    ///
    /// # 返回
    /// 包含更新后的用户 `Model` 或 `CustomError` 的 `Result`。
    pub async fn update_user_by_admin(
        &self,
        user_id: i32,
        req: UpdateUserRequest,
        new_password_hash: Option<String>,
    ) -> Result<Model, CustomError> { // 使用 CustomError
        let user = Entity::find_by_id(user_id).one(&self.db).await?;
        let mut user: ActiveModel = match user {
            Some(u) => u.into_active_model(),
            None => return Err(CustomError::NotFound("用户未找到".to_string())), // 使用 CustomError::NotFound
        };

        if let Some(username) = req.username {
            user.username = Set(username);
        }
        if let Some(email) = req.email {
            user.email = Set(email);
        }
        if let Some(password_hash) = new_password_hash {
            user.password_hash = Set(password_hash);
        }
        if let Some(role) = req.role {
            // 基本角色验证：确保是 'user' 或 'admin'
            if role != "user" && role != "admin" {
                return Err(CustomError::BadRequest("指定的角色无效。必须是 'user' 或 'admin'。".to_string())); // 使用 CustomError::BadRequest
            }
            user.role = Set(role);
        }
        user.updated_at = Set(Utc::now().naive_utc());

        let updated_user = user.update(&self.db).await?;
        Ok(updated_user)
    }

    /// 根据用户 ID 删除用户。
    ///
    /// # 参数
    /// * `user_id` - 要删除的用户 ID。
    ///
    /// # 返回
    /// 表示成功或失败的 `Result`。
    pub async fn delete_user(&self, user_id: i32) -> Result<(), CustomError> { // 使用 CustomError
        let result = Entity::delete_by_id(user_id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(CustomError::NotFound("用户未找到或已被删除".to_string())); // 使用 CustomError::NotFound
        }
        Ok(())
    }

    /// 根据用户 ID 列表删除多个用户。
    ///
    /// # 参数
    /// * `user_ids` - 要删除的用户 ID 向量。
    ///
    /// # 返回
    /// 表示成功删除行数的 `Result`。
    pub async fn delete_users_by_ids(&self, user_ids: Vec<i32>) -> Result<u64, CustomError> { // 使用 CustomError
        let result = Entity::delete_many()
            .filter(Column::Id.is_in(user_ids))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    /// 检索用户分页列表。（通常仅限管理员访问）
    ///
    /// # 参数
    /// * `page` - 当前页码（1-索引）。
    /// * `page_size` - 每页的用户数量。
    ///
    /// # 返回
    /// 包含 `(Vec<UserInfo>, total_pages, current_page, total_users)` 元组或 `CustomError` 的 `Result`。
    pub async fn get_all_users(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<UserInfo>, u64, u64, u64), CustomError> { // 使用 CustomError
        let paginator = Entity::find()
            .paginate(&self.db, page_size);

        let total_users = paginator.num_items().await?;
        let total_pages = paginator.num_pages().await?;

        // 调整页码为 0 索引以适应 SeaORM 的 `fetch_page`
        let users = paginator.fetch_page(page.saturating_sub(1)).await?
            .into_iter()
            .map(|user_model| UserInfo {
                id: user_model.id,
                username: user_model.username,
                email: user_model.email,
                role: user_model.role,
                created_at: user_model.created_at,
            })
            .collect();

        Ok((users, total_pages, page, total_users))
    }
}

// src/database/favorite.rs
//! 收藏数据库仓库和实体定义。

// --- SeaORM 实体定义 ---
use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;

/// `favorites` 表的实体定义。
///
/// 此结构体直接映射到数据库中 `favorites` 表的列。
/// 注意：`user_id` 和 `post_id` 组成联合主键。
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "favorites")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    #[sea_orm(primary_key)]
    pub post_id: i32,
    pub created_at: NaiveDateTime,
}

/// 定义 `favorites` 实体的关系。
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User, // 收藏记录属于一个用户

    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::PostId",
        to = "super::post::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Post, // 收藏记录关联一篇文章
}

impl Related<super::user::Entity> for Entity {
    fn to_entity_relation() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::post::Entity> for Entity {
    fn to_entity_relation() -> RelationDef {
        Relation::Post.def()
    }
}

/// 定义 `favorites` 实体的 `ActiveModel`。
/// 用于在数据库中创建、更新和删除记录。
#[derive(Debug, PartialEq, Eq, DeriveIntoActiveModel, DeriveActiveModel)]
pub struct ActiveModel;


// --- FavoriteRepository (数据库操作) ---
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    PaginatorTrait, QuerySelect, RelationTrait
};
use chrono::Utc;

// 导入 DTOs (如果需要，例如 PostInfo)
use crate::handler::idl::PostInfo;
// 导入自定义错误类型
use crate::error::CustomError;
// 导入文章实体，用于联接查询
use crate::database::post::Entity as PostEntity;
use crate::database::post::Model as PostModel; // 导入文章 Model


/// `FavoriteRepository` 提供与 `favorites` 表交互的方法。
/// 它持有数据库连接并执行 CRUD 操作。
#[derive(Clone)]
pub struct FavoriteRepository {
    db: DatabaseConnection,
}

impl FavoriteRepository {
    /// 创建一个新的 `FavoriteRepository` 实例。
    pub fn new(db: DatabaseConnection) -> Self {
        FavoriteRepository { db }
    }

    /// 切换收藏状态 (收藏/取消收藏)。
    /// 如果用户已收藏某文章，则取消收藏；否则，添加收藏。
    ///
    /// # 参数
    /// * `user_id` - 执行操作的用户 ID。
    /// * `post_id` - 被收藏/取消收藏的文章 ID。
    ///
    /// # 返回
    /// `Result<bool, CustomError>`，`true` 表示已收藏，`false` 表示已取消收藏。
    pub async fn toggle_favorite(
        &self,
        user_id: i32,
        post_id: i32,
    ) -> Result<bool, CustomError> {
        // 检查收藏记录是否已存在
        let existing_favorite = Entity::find()
            .filter(Column::UserId.eq(user_id))
            .filter(Column::PostId.eq(post_id))
            .one(&self.db)
            .await?;

        if let Some(fav) = existing_favorite {
            // 如果已存在，则删除 (取消收藏)
            fav.into_active_model().delete(&self.db).await?;
            Ok(false) // 已取消收藏
        } else {
            // 如果不存在，则创建 (添加收藏)
            let now = Utc::now().naive_utc();
            let active_model = ActiveModel {
                user_id: Set(user_id),
                post_id: Set(post_id),
                created_at: Set(now),
                ..Default::default()
            };
            active_model.insert(&self.db).await?;
            Ok(true) // 已收藏
        }
    }

    /// 检查指定文章是否已被用户收藏。
    ///
    /// # 参数
    /// * `user_id` - 用户 ID。
    /// * `post_id` - 文章 ID。
    ///
    /// # 返回
    /// `Result<bool, CustomError>`，`true` 表示已收藏，`false` 表示未收藏。
    pub async fn is_favorited(&self, user_id: i32, post_id: i32) -> Result<bool, CustomError> {
        let count = Entity::find()
            .filter(Column::UserId.eq(user_id))
            .filter(Column::PostId.eq(post_id))
            .count(&self.db)
            .await?;
        Ok(count > 0)
    }

    /// 检索用户收藏的文章分页列表。
    /// 此方法会联接 `posts` 表以获取文章的详细信息。
    ///
    /// # 参数
    /// * `user_id` - 要获取收藏列表的用户 ID。
    /// * `page` - 当前页码（1-索引）。
    /// * `page_size` - 每页的文章数量。
    ///
    /// # 返回
    /// 包含 `(Vec<PostInfo>, total_pages, current_page, total_favorites)` 元组或 `CustomError` 的 `Result`。
    pub async fn get_paginated_user_favorites(
        &self,
        user_id: i32,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<PostInfo>, u64, u64, u64), CustomError> {
        let paginator = Entity::find()
            .filter(Column::UserId.eq(user_id))
            .inner_join(PostEntity) // 联接文章表
            .select_only() // 只选择 PostEntity 的列
            .columns_as_models(PostEntity::included_columns().to_vec()) // 将联接的 PostEntity 列映射为 PostModel
            .order_by_desc(Column::CreatedAt) // 按收藏时间倒序
            .paginate(&self.db, page_size);

        let total_favorites = paginator.num_items().await?; // 收藏记录总数
        let total_pages = paginator.num_pages().await?;

        // `fetch_page` 返回的是 `Vec<Model>` (Favorite Model)，但我们需要 PostModel
        // SeaORM `columns_as_models` 配合 `fetch_page` 会将结果映射到 PostModel
        let favorited_posts_models: Vec<PostModel> = paginator.fetch_page(page.saturating_sub(1)).await?;

        let posts_info: Vec<PostInfo> = favorited_posts_models
            .into_iter()
            .map(|post_model| PostInfo {
                id: post_model.id,
                title: post_model.title,
                content_markdown: post_model.content_markdown,
                category: post_model.category,
                author_id: post_model.author_id,
                is_published: post_model.is_published == 1,
                view_count: post_model.view_count,
                cover_url: post_model.cover_url,
                created_at: post_model.created_at, // 这是文章的创建时间
                updated_at: post_model.updated_at, // 这是文章的更新时间
                // 注意：这里没有收藏记录的 `created_at`。如果需要，需要额外处理，
                // 例如通过 `select_as_pair` 来获取 Favorite 的 created_at
            })
            .collect();

        Ok((posts_info, total_pages, page, total_favorites))
    }
}

// src/database/comment.rs
//! 评论数据库仓库和实体定义。

// --- SeaORM 实体定义 ---
use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;

/// `comments` 表的实体定义。
///
/// 此结构体直接映射到数据库中 `comments` 表的列。
/// `created_at` 字段使用 `chrono::NaiveDateTime` 类型。
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "comments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub content: String,
    pub post_id: i32,
    pub user_id: i32,
    pub parent_id: Option<i32>, // 可选，用于嵌套评论
    pub created_at: NaiveDateTime,
}

/// 定义 `comments` 实体的关系。
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::PostId",
        to = "super::post::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Post, // 评论属于一篇文章

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User, // 评论属于一个用户

    #[sea_orm(
        belongs_to = "Entity", // 自引用关系，表示父评论
        from = "Column::ParentId",
        to = "Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade" // 删除父评论时，子评论也删除
    )]
    Parent, // 父评论

    #[sea_orm(has_many = "Entity")] // 一条评论可以有多个子评论
    Children, // 子评论
}

impl Related<super::post::Entity> for Entity {
    fn to_entity_relation() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to_entity_relation() -> RelationDef {
        Relation::User.def()
    }
}

// 注意：对于自引用关系，通常需要为父级和子级分别实现 Related
// impl Related<Entity> for Entity {
//     fn to_entity_relation() -> RelationDef {
//         Relation::Children.def() // 或 Relation::Parent.def()
//     }
// }


/// 定义 `comments` 实体的 `ActiveModel`。
/// 用于在数据库中创建、更新和删除记录。
#[derive(Debug, PartialEq, Eq, DeriveIntoActiveModel, DeriveActiveModel)]
pub struct ActiveModel;

// --- CommentRepository (数据库操作) ---
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    PaginatorTrait, QuerySelect
};
use chrono::Utc;

// 导入 DTOs
use crate::handler::idl::{
    CreateCommentRequest, UpdateCommentRequest, CommentInfo
};
// 导入自定义错误类型
use crate::error::CustomError;


/// `CommentRepository` 提供与 `comments` 表交互的方法。
/// 它持有数据库连接并执行 CRUD 操作。
#[derive(Clone)]
pub struct CommentRepository {
    db: DatabaseConnection,
}

impl CommentRepository {
    /// 创建一个新的 `CommentRepository` 实例。
    pub fn new(db: DatabaseConnection) -> Self {
        CommentRepository { db }
    }

    /// 创建一条新评论。
    ///
    /// # 参数
    /// * `req` - 包含评论详细信息的 `CreateCommentRequest`。
    /// * `user_id` - 评论用户的 ID。
    ///
    /// # 返回
    /// 包含创建的评论 `Model` 或 `CustomError` 的 `Result`。
    pub async fn create_comment(
        &self,
        req: CreateCommentRequest,
        user_id: i32,
    ) -> Result<Model, CustomError> {
        let now = Utc::now().naive_utc();

        let active_model = ActiveModel {
            content: Set(req.content),
            post_id: Set(req.post_id),
            user_id: Set(user_id),
            parent_id: Set(req.parent_id), // 可以为 None
            created_at: Set(now),
            ..Default::default()
        };

        let comment = active_model.insert(&self.db).await?;
        Ok(comment)
    }

    /// 根据评论 ID 检索评论。
    ///
    /// # 参数
    /// * `comment_id` - 要检索的评论 ID。
    ///
    /// # 返回
    /// 包含 `Option<Model>` 或 `CustomError` 的 `Result`。
    pub async fn get_comment_by_id(&self, comment_id: i32) -> Result<Option<Model>, CustomError> {
        let comment = Entity::find_by_id(comment_id).one(&self.db).await?;
        Ok(comment)
    }

    /// 更新评论内容。
    ///
    /// # 参数
    /// * `comment_id` - 要更新的评论 ID。
    /// * `req` - 包含新评论内容的 `UpdateCommentRequest`。
    ///
    /// # 返回
    /// 包含更新后的评论 `Model` 或 `CustomError` 的 `Result`。
    pub async fn update_comment(
        &self,
        comment_id: i32,
        req: UpdateCommentRequest,
    ) -> Result<Model, CustomError> {
        let comment = Entity::find_by_id(comment_id).one(&self.db).await?;
        let mut comment: ActiveModel = match comment {
            Some(c) => c.into_active_model(),
            None => return Err(CustomError::NotFound(format!("ID为 {} 的评论未找到", comment_id))),
        };

        comment.content = Set(req.content);
        // 通常不允许修改 post_id, user_id, parent_id

        let updated_comment = comment.update(&self.db).await?;
        Ok(updated_comment)
    }

    /// 删除评论。
    ///
    /// # 参数
    /// * `comment_id` - 要删除的评论 ID。
    ///
    /// # 返回
    /// 表示成功或失败的 `Result`。
    pub async fn delete_comment(&self, comment_id: i32) -> Result<(), CustomError> {
        let result = Entity::delete_by_id(comment_id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(CustomError::NotFound(format!("ID为 {} 的评论未找到或已被删除", comment_id)));
        }
        Ok(())
    }

    /// 检索指定文章的评论分页列表。
    ///
    /// # 参数
    /// * `post_id` - 要获取评论的文章 ID。
    /// * `page` - 当前页码（1-索引）。
    /// * `page_size` - 每页的评论数量。
    ///
    /// # 返回
    /// 包含 `(Vec<Model>, total_pages, current_page, total_comments)` 元组或 `CustomError` 的 `Result`。
    pub async fn get_paginated_comments_by_post_id(
        &self,
        post_id: i32,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<Model>, u64, u64, u64), CustomError> {
        let paginator = Entity::find()
            .filter(Column::PostId.eq(post_id))
            .order_by_desc(Column::CreatedAt) // 通常按时间倒序
            .paginate(&self.db, page_size);

        let total_comments = paginator.num_items().await?;
        let total_pages = paginator.num_pages().await?;

        let comments = paginator.fetch_page(page.saturating_sub(1)).await?;

        Ok((comments, total_pages, page, total_comments))
    }
}

// src/database/post.rs
//! 文章数据库仓库和实体定义。

// --- SeaORM 实体定义 ---
use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;
// 引入 IntoActiveModel trait，以便使用 Model::into_active_model() 方法
use sea_orm::{IntoActiveModel, ActiveModelBehavior};

/// `posts` 表的实体定义。
///
/// 此结构体直接映射到数据库中 `posts` 表的列。
/// `created_at` 和 `updated_at` 字段使用 `chrono::NaiveDateTime` 类型。
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub content_markdown: String,
    pub category: String,
    pub author_id: i32,
    pub is_published: i32, // 数据库中是 INTEGER (0/1), Rust 中会转换为 bool
    pub view_count: i32,
    pub cover_url: Option<String>, // 可选字段
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// 定义 `posts` 实体的关系。
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity", // 文章属于一个用户 (作者)
        from = "Column::AuthorId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User, 

    #[sea_orm(has_many = "super::comment::Entity")] // 一篇文章可以有多条评论
    Comment, 

    #[sea_orm(has_many = "super::favorite::Entity")] // 一篇文章可以被多个用户收藏
    Favorite, 
}

// 为关系实现 Related Trait
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
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

/// 定义 `posts` 实体的 `ActiveModel`。
/// 用于在数据库中创建、更新和删除记录。
/// 修复：移除了冗余的 `Debug`、`PartialEq`、`Eq` derive，因为 `DeriveActiveModel` 会自动包含它们。
impl ActiveModelBehavior for ActiveModel {}

// --- PostRepository (数据库操作) ---
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    PaginatorTrait, QuerySelect, RelationTrait, QueryOrder
};
use chrono::Utc;

// 导入 DTOs
use crate::handler::idl::{
    CreatePostRequest, UpdatePostRequest, PostListRequest, PostInfo
};
// 导入自定义错误类型
use crate::error::CustomError;


/// `PostRepository` 提供与 `posts` 表交互的方法。
/// 它持有数据库连接并执行 CRUD 操作。
#[derive(Clone)]
pub struct PostRepository {
    db: DatabaseConnection,
}

impl PostRepository {
    /// 创建一个新的 `PostRepository` 实例。
    pub fn new(db: DatabaseConnection) -> Self {
        PostRepository { db }
    }

    /// 创建一篇新文章。
    ///
    /// # 参数
    /// * `req` - 包含文章详细信息的 `CreatePostRequest`。
    /// * `author_id` - 文章作者的用户 ID。
    ///
    /// # 返回
    /// 包含创建的文章 `Model` 或 `CustomError` 的 `Result`。
    pub async fn create_post(
        &self,
        req: CreatePostRequest,
        author_id: i32,
    ) -> Result<Model, CustomError> {
        let now = Utc::now().naive_utc();
        // 将 Option<bool> 转换为 i32 (0 或 1)
        let is_published_int = if req.is_published.unwrap_or(false) { 1 } else { 0 };

        let active_model = ActiveModel {
            title: Set(req.title),
            content_markdown: Set(req.content_markdown),
            category: Set(req.category),
            author_id: Set(author_id),
            is_published: Set(is_published_int),
            view_count: Set(0), // 新文章初始浏览量为0
            cover_url: Set(req.cover_url), // Option<String> 直接 Set
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let post = active_model.insert(&self.db).await?;
        Ok(post)
    }

    /// 根据文章 ID 检索文章。
    ///
    /// # 参数
    /// * `post_id` - 要检索的文章 ID。
    ///
    /// # 返回
    /// 包含 `Option<Model>` 或 `CustomError` 的 `Result`。
    pub async fn get_post_by_id(&self, post_id: i32) -> Result<Option<Model>, CustomError> {
        let post = Entity::find_by_id(post_id).one(&self.db).await?;
        Ok(post)
    }

    /// 更新文章信息。
    ///
    /// # 参数
    /// * `post_id` - 要更新的文章 ID。
    /// * `req` - 包含可选更新字段的 `UpdatePostRequest`。
    ///
    /// # 返回
    /// 包含更新后的文章 `Model` 或 `CustomError` 的 `Result`。
    pub async fn update_post(
        &self,
        post_id: i32,
        req: UpdatePostRequest,
    ) -> Result<Model, CustomError> {
        let post = Entity::find_by_id(post_id).one(&self.db).await?;
        let mut post: ActiveModel = match post {
            Some(p) => p.into_active_model(),
            None => return Err(CustomError::NotFound(format!("ID为 {} 的文章未找到", post_id))),
        };

        if let Some(title) = req.title {
            post.title = Set(title);
        }
        if let Some(content_markdown) = req.content_markdown {
            post.content_markdown = Set(content_markdown);
        }
        if let Some(category) = req.category {
            post.category = Set(category);
        }
        if let Some(is_published) = req.is_published {
            post.is_published = Set(if is_published { 1 } else { 0 });
        }
        // 对于 Option<String> 类型的 cover_url，如果 req.cover_url 是 Some(value)，则更新为 Some(value)；
        // 如果 req.cover_url 是 None，则表示不更新该字段，保持数据库现有值。
        // 如果想显式设置为 NULL，需要客户端发送 Some(None) 这样的结构，
        // 或者提供一个 `clear_cover_url: bool` 字段。这里简单处理为 Some(String) 才更新。
        if let Some(cover_url_val) = req.cover_url {
            post.cover_url = Set(Some(cover_url_val));
        }
        // 如果 req.cover_url 为 None，则不更新 cover_url 字段，保持数据库现有值。

        post.updated_at = Set(Utc::now().naive_utc());

        let updated_post = post.update(&self.db).await?;
        Ok(updated_post)
    }

    /// 删除文章。
    ///
    /// # 参数
    /// * `post_id` - 要删除的文章 ID。
    ///
    /// # 返回
    /// 表示成功或失败的 `Result`。
    pub async fn delete_post(&self, post_id: i32) -> Result<(), CustomError> {
        let result = Entity::delete_by_id(post_id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(CustomError::NotFound(format!("ID为 {} 的文章未找到或已被删除", post_id)));
        }
        Ok(())
    }

    /// 批量删除文章。
    ///
    /// # 参数
    /// * `post_ids` - 要删除的文章 ID 向量。
    ///
    /// # 返回
    /// 表示成功删除行数的 `Result`。
    pub async fn delete_posts_by_ids(&self, post_ids: Vec<i32>) -> Result<u64, CustomError> {
        let result = Entity::delete_many()
            .filter(Column::Id.is_in(post_ids))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    /// 检索文章分页列表（带筛选和搜索功能）。
    ///
    /// # 参数
    /// * `req` - `PostListRequest`，包含分页、分类和搜索查询参数。
    ///
    /// # 返回
    /// 包含 `(Vec<PostInfo>, total_pages, current_page, total_posts)` 元组或 `CustomError` 的 `Result`。
    pub async fn get_paginated_posts(
        &self,
        req: PostListRequest,
    ) -> Result<(Vec<PostInfo>, u64, u64, u64), CustomError> {
        // req.page 和 req.limit 已经在 DTO 中处理了默认值，所以这里直接 unwrap
        let page = req.page.unwrap_or(1);
        let page_size = req.limit.unwrap_or(10);

        let mut select = Entity::find();

        // 根据分类过滤
        if let Some(category) = req.category {
            select = select.filter(Column::Category.eq(category));
        }

        // 根据搜索查询过滤
        if let Some(query) = req.query {
            // 使用 LIKE 进行模糊匹配，可以同时搜索标题和内容
            select = select.filter(
                Column::Title.like(format!("%{}%", query))
                    .or(Column::ContentMarkdown.like(format!("%{}%", query)))
            );
        }

        // 仅获取已发布的文章 (如果指定)
        // req.published_only 默认值在 DTO 中已经处理，这里直接 unwrap
        if req.published_only.unwrap_or(true) { 
            select = select.filter(Column::IsPublished.eq(1));
        }

        // 按创建时间倒序排序
        select = select.order_by_desc(Column::CreatedAt);

        let paginator = select.paginate(&self.db, page_size);

        let total_posts = paginator.num_items().await?;
        let total_pages = paginator.num_pages().await?;

        // `fetch_page` 接收 0-indexed 的页码，所以需要 `saturating_sub(1)`
        let posts = paginator.fetch_page(page.saturating_sub(1)).await?
            .into_iter()
            .map(|post_model| PostInfo {
                id: post_model.id,
                title: post_model.title,
                content_markdown: post_model.content_markdown,
                category: post_model.category,
                author_id: post_model.author_id,
                is_published: post_model.is_published == 1, // 数据库中 1/0 转换为 bool
                view_count: post_model.view_count,
                cover_url: post_model.cover_url,
                created_at: post_model.created_at,
                updated_at: post_model.updated_at,
            })
            .collect();

        Ok((posts, total_pages, page, total_posts))
    }

    /// 增加文章的浏览量。
    ///
    /// # 参数
    /// * `post_id` - 要增加浏览量的文章 ID。
    ///
    /// # 返回
    /// 表示操作成功或失败的 `Result`。
    pub async fn increment_view_count(&self, post_id: i32) -> Result<(), CustomError> {
        let post = Entity::find_by_id(post_id).one(&self.db).await?;
        if let Some(model) = post {
            let mut active_model: ActiveModel = model.into_active_model();
            // 确保 view_count 是 Option<i32> 或 i32。数据库是 i32，所以这里直接加 1
            active_model.view_count = Set(active_model.view_count.unwrap() + 1);
            active_model.update(&self.db).await?;
            Ok(())
        } else {
            Err(CustomError::NotFound(format!("ID为 {} 的文章未找到", post_id)))
        }
    }
}

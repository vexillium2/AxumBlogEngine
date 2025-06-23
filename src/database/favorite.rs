use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, 
    IntoActiveModel, QueryFilter, QuerySelect, Set,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Favorite {
    pub user_id: i32,
    pub article_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

/// 添加收藏
pub async fn create(
    db: &DatabaseConnection,
    user_id: i32,
    article_id: i32,
) -> Result<(), sea_orm::DbErr> {
    // 检查是否已收藏
    if exists(db, user_id, article_id).await? {
        return Ok(()); // 已存在则静默成功
    }

    entity::ActiveModel {
        user_id: Set(user_id),
        article_id: Set(article_id),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(())
}

/// 取消收藏
pub async fn delete(
    db: &DatabaseConnection,
    user_id: i32,
    article_id: i32,
) -> Result<(), sea_orm::DbErr> {
    entity::Entity::delete_many()
        .filter(entity::Column::UserId.eq(user_id))
        .filter(entity::Column::ArticleId.eq(article_id))
        .exec(db)
        .await?;
    Ok(())
}

/// 检查收藏状态
pub async fn exists(
    db: &DatabaseConnection,
    user_id: i32,
    article_id: i32,
) -> Result<bool, sea_orm::DbErr> {
    let count = entity::Entity::find()
        .filter(entity::Column::UserId.eq(user_id))
        .filter(entity::Column::ArticleId.eq(article_id))
        .count(db)
        .await?;
    Ok(count > 0)
}

/// 获取用户收藏列表
pub async fn list_by_user(
    db: &DatabaseConnection,
    user_id: i32,
    page: u32,
    per_page: u32,
) -> Result<(Vec<Favorite>, u64), sea_orm::DbErr> {
    // 获取总数
    let total = entity::Entity::find()
        .filter(entity::Column::UserId.eq(user_id))
        .count(db)
        .await?;

    // 分页查询
    let favorites = entity::Entity::find()
        .filter(entity::Column::UserId.eq(user_id))
        .order_by_desc(entity::Column::CreatedAt)
        .offset((page - 1) * per_page as u64)
        .limit(per_page as u64)
        .into_model::<Favorite>()
        .all(db)
        .await?;

    Ok((favorites, total))
}

/// 获取文章收藏数
pub async fn count_by_article(
    db: &DatabaseConnection,
    article_id: i32,
) -> Result<u64, sea_orm::DbErr> {
    entity::Entity::find()
        .filter(entity::Column::ArticleId.eq(article_id))
        .count(db)
        .await
}
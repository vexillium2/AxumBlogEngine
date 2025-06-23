#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_update_user() {
        // 测试前初始化数据库
        crate::database::orm::init_db("sqlite::memory:").await.unwrap();
        
        // 创建测试用户
        let user = user::create(/* 参数 */).await.unwrap();
        
        // 执行更新测试
        let update_data = user::ActiveModel {
            id: sea_orm::Set(user.id),
            username: sea_orm::Set("new_username".to_string()),
            ..Default::default()
        };
        
        let updated = user::update(update_data).await.unwrap();
        assert_eq!(updated.username, "new_username");
    }
}
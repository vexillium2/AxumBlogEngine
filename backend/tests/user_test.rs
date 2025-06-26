//! 用户模块测试用例
//! 测试 UserRepository 的各种数据库操作功能

#[cfg(test)]
mod tests {
    use sea_orm::{Database, DatabaseConnection, ConnectionTrait};
    use chrono::Utc;
    use bcrypt::{hash, DEFAULT_COST};
    
    // 导入项目模块
    use axum_blog_engine::database::user::{UserRepository, Model};
    use axum_blog_engine::handler::idl::{
        RegisterRequest, CreateUserByAdminRequest, UpdateMyProfileRequest, UpdateUserRequest
    };
    use axum_blog_engine::error::CustomError;

    /// 创建内存数据库连接用于测试
    async fn setup_test_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to test database");
        
        // 创建用户表
        let sql = r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                role TEXT NOT NULL DEFAULT 'user',
                created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now')),
                updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now'))
            );
        "#;
        
        db.execute_unprepared(sql).await.expect("Failed to create users table");
        
        db
    }

    /// 创建测试用户数据
    async fn create_test_user(repo: &UserRepository) -> Result<Model, CustomError> {
        let register_req = RegisterRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        
        let password_hash = hash(&register_req.password, DEFAULT_COST)
            .map_err(|e| CustomError::BcryptError(e.to_string()))?;
        
        repo.create_user_from_register(register_req, password_hash).await
    }

    #[tokio::test]
    async fn test_create_user_from_register() {
        let db = setup_test_db().await;
        let repo = UserRepository::new(db);
        
        let register_req = RegisterRequest {
            username: "newuser".to_string(),
            email: "newuser@example.com".to_string(),
            password: "securepassword".to_string(),
        };
        
        let password_hash = hash(&register_req.password, DEFAULT_COST).unwrap();
        let result = repo.create_user_from_register(register_req, password_hash).await;
        
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.username, "newuser");
        assert_eq!(user.email, "newuser@example.com");
        assert_eq!(user.role, "user"); // 默认角色
        assert!(user.id > 0);
    }

    #[tokio::test]
    async fn test_create_user_by_admin() {
        let db = setup_test_db().await;
        let repo = UserRepository::new(db);
        
        let admin_req = CreateUserByAdminRequest {
            username: "adminuser".to_string(),
            email: "admin@example.com".to_string(),
            password: "adminpassword".to_string(),
            role: "admin".to_string(),
        };
        
        let password_hash = hash(&admin_req.password, DEFAULT_COST).unwrap();
        let result = repo.create_user_by_admin(admin_req, password_hash).await;
        
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.username, "adminuser");
        assert_eq!(user.email, "admin@example.com");
        assert_eq!(user.role, "admin"); // 管理员指定的角色
    }

    #[tokio::test]
    async fn test_get_user_by_id() {
        let db = setup_test_db().await;
        let repo = UserRepository::new(db);
        
        // 先创建一个用户
        let created_user = create_test_user(&repo).await.unwrap();
        
        // 通过ID查找用户
        let result = repo.get_user_by_id(created_user.id).await;
        assert!(result.is_ok());
        
        let found_user = result.unwrap();
        assert!(found_user.is_some());
        
        let user = found_user.unwrap();
        assert_eq!(user.id, created_user.id);
        assert_eq!(user.username, created_user.username);
        assert_eq!(user.email, created_user.email);
    }

    #[tokio::test]
    async fn test_get_user_by_id_not_found() {
        let db = setup_test_db().await;
        let repo = UserRepository::new(db);
        
        // 查找不存在的用户ID
        let result = repo.get_user_by_id(999).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_user_by_username_or_email() {
        let db = setup_test_db().await;
        let repo = UserRepository::new(db);
        
        // 先创建一个用户
        let created_user = create_test_user(&repo).await.unwrap();
        
        // 通过用户名查找
        let result_by_username = repo.get_user_by_username_or_email(&created_user.username).await;
        assert!(result_by_username.is_ok());
        assert!(result_by_username.unwrap().is_some());
        
        // 通过邮箱查找
        let result_by_email = repo.get_user_by_username_or_email(&created_user.email).await;
        assert!(result_by_email.is_ok());
        assert!(result_by_email.unwrap().is_some());
        
        // 查找不存在的用户
        let result_not_found = repo.get_user_by_username_or_email("nonexistent").await;
        assert!(result_not_found.is_ok());
        assert!(result_not_found.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_update_my_profile() {
        let db = setup_test_db().await;
        let repo = UserRepository::new(db);
        
        // 先创建一个用户
        let created_user = create_test_user(&repo).await.unwrap();
        
        // 更新用户信息
        let update_req = UpdateMyProfileRequest {
            username: Some("updateduser".to_string()),
            email: Some("updated@example.com".to_string()),
            password: Some("newpassword123".to_string()),
        };
        
        let new_password_hash = Some(hash("newpassword123", DEFAULT_COST).unwrap());
        let result = repo.update_my_profile(created_user.id, update_req, new_password_hash).await;
        
        assert!(result.is_ok());
        let updated_user = result.unwrap();
        assert_eq!(updated_user.username, "updateduser");
        assert_eq!(updated_user.email, "updated@example.com");
        assert_ne!(updated_user.password_hash, created_user.password_hash); // 密码已更新
    }

    #[tokio::test]
    async fn test_update_my_profile_partial() {
        let db = setup_test_db().await;
        let repo = UserRepository::new(db);
        
        // 先创建一个用户
        let created_user = create_test_user(&repo).await.unwrap();
        
        // 只更新用户名
        let update_req = UpdateMyProfileRequest {
            username: Some("partialupdateuser".to_string()),
            email: None,
            password: None,
        };
        
        let result = repo.update_my_profile(created_user.id, update_req, None).await;
        
        assert!(result.is_ok());
        let updated_user = result.unwrap();
        assert_eq!(updated_user.username, "partialupdateuser");
        assert_eq!(updated_user.email, created_user.email); // 邮箱未变
        assert_eq!(updated_user.password_hash, created_user.password_hash); // 密码未变
    }

    #[tokio::test]
    async fn test_update_user_by_admin() {
        let db = setup_test_db().await;
        let repo = UserRepository::new(db);
        
        // 先创建一个用户
        let created_user = create_test_user(&repo).await.unwrap();
        
        // 管理员更新用户信息（包括角色）
        let update_req = UpdateUserRequest {
            username: Some("adminupdateduser".to_string()),
            email: Some("adminupdated@example.com".to_string()),
            password: Some("adminpassword123".to_string()),
            role: Some("admin".to_string()),
        };
        
        let new_password_hash = Some(hash("adminpassword123", DEFAULT_COST).unwrap());
        let result = repo.update_user_by_admin(created_user.id, update_req, new_password_hash).await;
        
        assert!(result.is_ok());
        let updated_user = result.unwrap();
        assert_eq!(updated_user.username, "adminupdateduser");
        assert_eq!(updated_user.email, "adminupdated@example.com");
        assert_eq!(updated_user.role, "admin"); // 角色已更新为管理员
    }

    #[tokio::test]
    async fn test_update_user_by_admin_invalid_role() {
        let db = setup_test_db().await;
        let repo = UserRepository::new(db);
        
        // 先创建一个用户
        let created_user = create_test_user(&repo).await.unwrap();
        
        // 尝试设置无效角色
        let update_req = UpdateUserRequest {
            username: None,
            email: None,
            password: None,
            role: Some("invalid_role".to_string()),
        };
        
        let result = repo.update_user_by_admin(created_user.id, update_req, None).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            CustomError::BadRequest(msg) => {
                assert!(msg.contains("指定的角色无效"));
            }
            _ => panic!("Expected BadRequest error"),
        }
    }

    #[tokio::test]
    async fn test_update_user_not_found() {
        let db = setup_test_db().await;
        let repo = UserRepository::new(db);
        
        let update_req = UpdateMyProfileRequest {
            username: Some("nonexistentuser".to_string()),
            email: None,
            password: None,
        };
        
        let result = repo.update_my_profile(999, update_req, None).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            CustomError::NotFound(msg) => {
                assert!(msg.contains("用户未找到"));
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_delete_user() {
        let db = setup_test_db().await;
        let repo = UserRepository::new(db);
        
        // 先创建一个用户
        let created_user = create_test_user(&repo).await.unwrap();
        
        // 删除用户
        let result = repo.delete_user(created_user.id).await;
        assert!(result.is_ok());
        
        // 验证用户已被删除
        let find_result = repo.get_user_by_id(created_user.id).await;
        assert!(find_result.is_ok());
        assert!(find_result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_delete_user_not_found() {
        let db = setup_test_db().await;
        let repo = UserRepository::new(db);
        
        // 尝试删除不存在的用户
        let result = repo.delete_user(999).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            CustomError::NotFound(msg) => {
                assert!(msg.contains("用户未找到或已被删除"));
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_duplicate_username() {
        let db = setup_test_db().await;
        let repo = UserRepository::new(db);
        
        // 创建第一个用户
        let _first_user = create_test_user(&repo).await.unwrap();
        
        // 尝试创建具有相同用户名的用户
        let duplicate_req = RegisterRequest {
            username: "testuser".to_string(), // 相同的用户名
            email: "different@example.com".to_string(),
            password: "password123".to_string(),
        };
        
        let password_hash = hash(&duplicate_req.password, DEFAULT_COST).unwrap();
        let result = repo.create_user_from_register(duplicate_req, password_hash).await;
        
        // 应该失败，因为用户名重复
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_duplicate_email() {
        let db = setup_test_db().await;
        let repo = UserRepository::new(db);
        
        // 创建第一个用户
        let _first_user = create_test_user(&repo).await.unwrap();
        
        // 尝试创建具有相同邮箱的用户
        let duplicate_req = RegisterRequest {
            username: "differentuser".to_string(),
            email: "test@example.com".to_string(), // 相同的邮箱
            password: "password123".to_string(),
        };
        
        let password_hash = hash(&duplicate_req.password, DEFAULT_COST).unwrap();
        let result = repo.create_user_from_register(duplicate_req, password_hash).await;
        
        // 应该失败，因为邮箱重复
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_user_timestamps() {
        let db = setup_test_db().await;
        let repo = UserRepository::new(db);
        
        let before_creation = Utc::now().naive_utc();
        
        // 创建用户
        let created_user = create_test_user(&repo).await.unwrap();
        
        let after_creation = Utc::now().naive_utc();
        
        // 验证时间戳
        assert!(created_user.created_at >= before_creation);
        assert!(created_user.created_at <= after_creation);
        assert!(created_user.updated_at >= before_creation);
        assert!(created_user.updated_at <= after_creation);
        
        // 等待一小段时间后更新用户
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        let update_req = UpdateMyProfileRequest {
            username: Some("timestamptest".to_string()),
            email: None,
            password: None,
        };
        
        let before_update = Utc::now().naive_utc();
        let updated_user = repo.update_my_profile(created_user.id, update_req, None).await.unwrap();
        let after_update = Utc::now().naive_utc();
        
        // 验证更新时间戳
        assert_eq!(updated_user.created_at, created_user.created_at); // 创建时间不变
        assert!(updated_user.updated_at >= before_update); // 更新时间应该更新
        assert!(updated_user.updated_at <= after_update);
        assert!(updated_user.updated_at > created_user.updated_at); // 更新时间应该晚于创建时间
    }
}
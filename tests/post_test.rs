//! 文章模块测试用例
//! 测试 PostRepository 的各种数据库操作功能

#[cfg(test)]
mod tests {
    use sea_orm::{Database, DatabaseConnection, ConnectionTrait};
    use chrono::Utc;
    
    // 导入项目模块
    use axum_blog_engine::database::post::{PostRepository, Model};
    use axum_blog_engine::handler::idl::{
        CreatePostRequest, UpdatePostRequest, PostListRequest
    };
    use axum_blog_engine::error::CustomError;

    /// 创建内存数据库连接用于测试
    async fn setup_test_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to test database");
        
        // 创建文章表
        let sql = r#"
            CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content_markdown TEXT NOT NULL,
                category TEXT NOT NULL,
                author_id INTEGER NOT NULL,
                is_published INTEGER NOT NULL DEFAULT 0,
                view_count INTEGER NOT NULL DEFAULT 0,
                cover_url TEXT,
                created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now')),
                updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now'))
            );
        "#;
        
        db.execute_unprepared(sql).await.expect("Failed to create posts table");
        
        db
    }

    /// 创建测试文章数据
    async fn create_test_post(repo: &PostRepository, author_id: i32) -> Result<Model, CustomError> {
        let create_req = CreatePostRequest {
            title: "测试文章标题".to_string(),
            content_markdown: "# 测试文章内容\n\n这是一篇测试文章的内容。".to_string(),
            category: "技术".to_string(),
            is_published: Some(true),
            cover_url: Some("https://example.com/cover.jpg".to_string()),
        };
        
        repo.create_post(create_req, author_id).await
    }

    #[tokio::test]
    async fn test_create_post() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        let create_req = CreatePostRequest {
            title: "新文章标题".to_string(),
            content_markdown: "# 新文章\n\n这是新文章的内容。".to_string(),
            category: "生活".to_string(),
            is_published: Some(false), // 草稿状态
            cover_url: None,
        };
        
        let result = repo.create_post(create_req, 1).await;
        
        assert!(result.is_ok());
        let post = result.unwrap();
        assert_eq!(post.title, "新文章标题");
        assert_eq!(post.category, "生活");
        assert_eq!(post.author_id, 1);
        assert_eq!(post.is_published, 0); // 草稿状态
        assert_eq!(post.view_count, 0); // 初始浏览量为0
        assert!(post.cover_url.is_none());
        assert!(post.id > 0);
    }

    #[tokio::test]
    async fn test_create_post_published() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        let create_req = CreatePostRequest {
            title: "已发布文章".to_string(),
            content_markdown: "已发布的文章内容".to_string(),
            category: "技术".to_string(),
            is_published: Some(true), // 发布状态
            cover_url: Some("https://example.com/published.jpg".to_string()),
        };
        
        let result = repo.create_post(create_req, 2).await;
        
        assert!(result.is_ok());
        let post = result.unwrap();
        assert_eq!(post.is_published, 1); // 发布状态
        assert_eq!(post.cover_url, Some("https://example.com/published.jpg".to_string()));
    }

    #[tokio::test]
    async fn test_get_post_by_id() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        // 先创建一篇文章
        let created_post = create_test_post(&repo, 1).await.unwrap();
        
        // 通过ID查找文章
        let result = repo.get_post_by_id(created_post.id).await;
        assert!(result.is_ok());
        
        let found_post = result.unwrap();
        assert!(found_post.is_some());
        
        let post = found_post.unwrap();
        assert_eq!(post.id, created_post.id);
        assert_eq!(post.title, created_post.title);
        assert_eq!(post.content_markdown, created_post.content_markdown);
    }

    #[tokio::test]
    async fn test_get_post_by_id_not_found() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        // 查找不存在的文章ID
        let result = repo.get_post_by_id(999).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_update_post() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        // 先创建一篇文章
        let created_post = create_test_post(&repo, 1).await.unwrap();
        
        // 更新文章信息
        let update_req = UpdatePostRequest {
            title: Some("更新后的标题".to_string()),
            content_markdown: Some("# 更新后的内容\n\n这是更新后的文章内容。".to_string()),
            category: Some("更新分类".to_string()),
            is_published: Some(false), // 改为草稿
            cover_url: Some("https://example.com/updated.jpg".to_string()),
        };
        
        let result = repo.update_post(created_post.id, update_req).await;
        
        assert!(result.is_ok());
        let updated_post = result.unwrap();
        assert_eq!(updated_post.title, "更新后的标题");
        assert_eq!(updated_post.category, "更新分类");
        assert_eq!(updated_post.is_published, 0); // 草稿状态
        assert_eq!(updated_post.cover_url, Some("https://example.com/updated.jpg".to_string()));
        assert!(updated_post.updated_at > created_post.updated_at); // 更新时间应该更新
    }

    #[tokio::test]
    async fn test_update_post_partial() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        // 先创建一篇文章
        let created_post = create_test_post(&repo, 1).await.unwrap();
        
        // 只更新标题
        let update_req = UpdatePostRequest {
            title: Some("部分更新标题".to_string()),
            content_markdown: None,
            category: None,
            is_published: None,
            cover_url: None,
        };
        
        let result = repo.update_post(created_post.id, update_req).await;
        
        assert!(result.is_ok());
        let updated_post = result.unwrap();
        assert_eq!(updated_post.title, "部分更新标题");
        assert_eq!(updated_post.content_markdown, created_post.content_markdown); // 内容未变
        assert_eq!(updated_post.category, created_post.category); // 分类未变
        assert_eq!(updated_post.is_published, created_post.is_published); // 发布状态未变
    }

    #[tokio::test]
    async fn test_update_post_not_found() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        let update_req = UpdatePostRequest {
            title: Some("不存在的文章".to_string()),
            content_markdown: None,
            category: None,
            is_published: None,
            cover_url: None,
        };
        
        let result = repo.update_post(999, update_req).await;
        assert!(result.is_err());
        
        if let Err(CustomError::NotFound(msg)) = result {
            assert!(msg.contains("999"));
            assert!(msg.contains("未找到"));
        } else {
            panic!("Expected NotFound error");
        }
    }

    #[tokio::test]
    async fn test_delete_post() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        // 先创建一篇文章
        let created_post = create_test_post(&repo, 1).await.unwrap();
        
        // 删除文章
        let result = repo.delete_post(created_post.id).await;
        assert!(result.is_ok());
        
        // 验证文章已被删除
        let found_post = repo.get_post_by_id(created_post.id).await.unwrap();
        assert!(found_post.is_none());
    }

    #[tokio::test]
    async fn test_delete_post_not_found() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        // 删除不存在的文章
        let result = repo.delete_post(999).await;
        assert!(result.is_err());
        
        if let Err(CustomError::NotFound(msg)) = result {
            assert!(msg.contains("999"));
            assert!(msg.contains("未找到或已被删除"));
        } else {
            panic!("Expected NotFound error");
        }
    }

    #[tokio::test]
    async fn test_delete_posts_by_ids() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        // 创建多篇文章
        let post1 = create_test_post(&repo, 1).await.unwrap();
        let post2 = create_test_post(&repo, 2).await.unwrap();
        let post3 = create_test_post(&repo, 3).await.unwrap();
        
        // 批量删除文章
        let post_ids = vec![post1.id, post2.id];
        let result = repo.delete_posts_by_ids(post_ids).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2); // 删除了2篇文章
        
        // 验证文章已被删除
        assert!(repo.get_post_by_id(post1.id).await.unwrap().is_none());
        assert!(repo.get_post_by_id(post2.id).await.unwrap().is_none());
        assert!(repo.get_post_by_id(post3.id).await.unwrap().is_some()); // post3未被删除
    }

    #[tokio::test]
    async fn test_get_paginated_posts() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        // 创建多篇文章
        for i in 1..=5 {
            let create_req = CreatePostRequest {
                title: format!("文章{}", i),
                content_markdown: format!("内容{}", i),
                category: if i % 2 == 0 { "技术".to_string() } else { "生活".to_string() },
                is_published: Some(true),
                cover_url: None,
            };
            repo.create_post(create_req, i).await.unwrap();
        }
        
        // 测试分页查询
        let list_req = PostListRequest {
            page: Some(1),
            limit: Some(3),
            category: None,
            query: None,
            published_only: Some(true),
        };
        
        let result = repo.get_paginated_posts(list_req).await;
        assert!(result.is_ok());
        
        let (posts, total_pages, current_page, total_posts) = result.unwrap();
        assert_eq!(posts.len(), 3); // 每页3篇文章
        assert_eq!(current_page, 1);
        assert_eq!(total_posts, 5);
        assert_eq!(total_pages, 2); // 5篇文章分2页
    }

    #[tokio::test]
    async fn test_get_paginated_posts_with_category_filter() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        // 创建不同分类的文章
        for i in 1..=4 {
            let create_req = CreatePostRequest {
                title: format!("文章{}", i),
                content_markdown: format!("内容{}", i),
                category: if i <= 2 { "技术".to_string() } else { "生活".to_string() },
                is_published: Some(true),
                cover_url: None,
            };
            repo.create_post(create_req, i).await.unwrap();
        }
        
        // 按分类过滤
        let list_req = PostListRequest {
            page: Some(1),
            limit: Some(10),
            category: Some("技术".to_string()),
            query: None,
            published_only: Some(true),
        };
        
        let result = repo.get_paginated_posts(list_req).await;
        assert!(result.is_ok());
        
        let (posts, _, _, total_posts) = result.unwrap();
        assert_eq!(posts.len(), 2); // 只有2篇技术文章
        assert_eq!(total_posts, 2);
        
        // 验证所有返回的文章都是技术分类
        for post in posts {
            assert_eq!(post.category, "技术");
        }
    }

    #[tokio::test]
    async fn test_get_paginated_posts_with_search_query() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        // 创建包含特定关键词的文章
        let create_req1 = CreatePostRequest {
            title: "Rust编程语言".to_string(),
            content_markdown: "Rust是一门系统编程语言".to_string(),
            category: "技术".to_string(),
            is_published: Some(true),
            cover_url: None,
        };
        repo.create_post(create_req1, 1).await.unwrap();
        
        let create_req2 = CreatePostRequest {
            title: "Python入门".to_string(),
            content_markdown: "Python是一门简单易学的语言".to_string(),
            category: "技术".to_string(),
            is_published: Some(true),
            cover_url: None,
        };
        repo.create_post(create_req2, 2).await.unwrap();
        
        // 搜索包含"Rust"的文章
        let list_req = PostListRequest {
            page: Some(1),
            limit: Some(10),
            category: None,
            query: Some("Rust".to_string()),
            published_only: Some(true),
        };
        
        let result = repo.get_paginated_posts(list_req).await;
        assert!(result.is_ok());
        
        let (posts, _, _, total_posts) = result.unwrap();
        assert_eq!(posts.len(), 1); // 只有1篇包含Rust的文章
        assert_eq!(total_posts, 1);
        assert!(posts[0].title.contains("Rust"));
    }

    #[tokio::test]
    async fn test_increment_view_count() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        // 先创建一篇文章
        let created_post = create_test_post(&repo, 1).await.unwrap();
        assert_eq!(created_post.view_count, 0);
        
        // 增加浏览量
        let result = repo.increment_view_count(created_post.id).await;
        assert!(result.is_ok());
        
        // 验证浏览量已增加
        let updated_post = repo.get_post_by_id(created_post.id).await.unwrap().unwrap();
        assert_eq!(updated_post.view_count, 1);
        
        // 再次增加浏览量
        repo.increment_view_count(created_post.id).await.unwrap();
        let updated_post = repo.get_post_by_id(created_post.id).await.unwrap().unwrap();
        assert_eq!(updated_post.view_count, 2);
    }

    #[tokio::test]
    async fn test_increment_view_count_not_found() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        // 对不存在的文章增加浏览量
        let result = repo.increment_view_count(999).await;
        assert!(result.is_err());
        
        if let Err(CustomError::NotFound(msg)) = result {
            assert!(msg.contains("999"));
            assert!(msg.contains("未找到"));
        } else {
            panic!("Expected NotFound error");
        }
    }

    #[tokio::test]
    async fn test_post_timestamps() {
        let db = setup_test_db().await;
        let repo = PostRepository::new(db);
        
        let before_creation = Utc::now().naive_utc();
        
        // 创建文章
        let created_post = create_test_post(&repo, 1).await.unwrap();
        
        let after_creation = Utc::now().naive_utc();
        
        // 验证时间戳
        assert!(created_post.created_at >= before_creation);
        assert!(created_post.created_at <= after_creation);
        assert!(created_post.updated_at >= before_creation);
        assert!(created_post.updated_at <= after_creation);
        
        // 等待一小段时间后更新文章
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        let update_req = UpdatePostRequest {
            title: Some("时间戳测试更新".to_string()),
            content_markdown: None,
            category: None,
            is_published: None,
            cover_url: None,
        };
        
        let before_update = Utc::now().naive_utc();
        let updated_post = repo.update_post(created_post.id, update_req).await.unwrap();
        let after_update = Utc::now().naive_utc();
        
        // 验证更新时间戳
        assert_eq!(updated_post.created_at, created_post.created_at); // 创建时间不变
        assert!(updated_post.updated_at >= before_update); // 更新时间应该更新
        assert!(updated_post.updated_at <= after_update);
        assert!(updated_post.updated_at > created_post.updated_at); // 更新时间应该晚于创建时间
    }
}
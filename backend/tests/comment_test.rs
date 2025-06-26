//! 评论模块测试用例
//! 测试 CommentRepository 的各种数据库操作功能

#[cfg(test)]
mod tests {
    use sea_orm::{Database, DatabaseConnection, ConnectionTrait};
    use chrono::Utc;
    
    // 导入项目模块
    use axum_blog_engine::database::comment::{CommentRepository, Model};
    use axum_blog_engine::handler::idl::{
        CreateCommentRequest, UpdateCommentRequest
    };
    use axum_blog_engine::error::CustomError;

    /// 创建内存数据库连接用于测试
    async fn setup_test_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to test database");
        
        // 创建评论表
        let sql = r#"
            CREATE TABLE IF NOT EXISTS comments (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL,
                post_id INTEGER NOT NULL,
                user_id INTEGER NOT NULL,
                parent_id INTEGER,
                created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now'))
            );
        "#;
        
        db.execute_unprepared(sql).await.expect("Failed to create comments table");
        
        db
    }

    /// 创建测试评论数据
    async fn create_test_comment(repo: &CommentRepository, post_id: i32, user_id: i32) -> Result<Model, CustomError> {
        let create_req = CreateCommentRequest {
            content: "这是一条测试评论".to_string(),
            post_id,
            parent_id: None,
        };
        
        repo.create_comment(create_req, user_id).await
    }

    /// 创建子评论
    async fn create_child_comment(repo: &CommentRepository, post_id: i32, user_id: i32, parent_id: i32) -> Result<Model, CustomError> {
        let create_req = CreateCommentRequest {
            content: "这是一条子评论".to_string(),
            post_id,
            parent_id: Some(parent_id),
        };
        
        repo.create_comment(create_req, user_id).await
    }

    #[tokio::test]
    async fn test_create_comment() {
        let db = setup_test_db().await;
        let repo = CommentRepository::new(db);
        
        let create_req = CreateCommentRequest {
            content: "这是一条新评论".to_string(),
            post_id: 1,
            parent_id: None,
        };
        
        let result = repo.create_comment(create_req, 1).await;
        
        assert!(result.is_ok());
        let comment = result.unwrap();
        assert_eq!(comment.content, "这是一条新评论");
        assert_eq!(comment.post_id, 1);
        assert_eq!(comment.user_id, 1);
        assert!(comment.parent_id.is_none());
        assert!(comment.id > 0);
    }

    #[tokio::test]
    async fn test_create_nested_comment() {
        let db = setup_test_db().await;
        let repo = CommentRepository::new(db);
        
        // 先创建父评论
        let parent_comment = create_test_comment(&repo, 1, 1).await.unwrap();
        
        // 创建子评论
        let create_req = CreateCommentRequest {
            content: "这是一条回复评论".to_string(),
            post_id: 1,
            parent_id: Some(parent_comment.id),
        };
        
        let result = repo.create_comment(create_req, 2).await;
        
        assert!(result.is_ok());
        let child_comment = result.unwrap();
        assert_eq!(child_comment.content, "这是一条回复评论");
        assert_eq!(child_comment.post_id, 1);
        assert_eq!(child_comment.user_id, 2);
        assert_eq!(child_comment.parent_id, Some(parent_comment.id));
    }

    #[tokio::test]
    async fn test_get_comment_by_id() {
        let db = setup_test_db().await;
        let repo = CommentRepository::new(db);
        
        // 先创建一条评论
        let created_comment = create_test_comment(&repo, 1, 1).await.unwrap();
        
        // 通过ID查找评论
        let result = repo.get_comment_by_id(created_comment.id).await;
        assert!(result.is_ok());
        
        let found_comment = result.unwrap();
        assert!(found_comment.is_some());
        
        let comment = found_comment.unwrap();
        assert_eq!(comment.id, created_comment.id);
        assert_eq!(comment.content, created_comment.content);
        assert_eq!(comment.post_id, created_comment.post_id);
        assert_eq!(comment.user_id, created_comment.user_id);
    }

    #[tokio::test]
    async fn test_get_comment_by_id_not_found() {
        let db = setup_test_db().await;
        let repo = CommentRepository::new(db);
        
        // 查找不存在的评论ID
        let result = repo.get_comment_by_id(999).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_update_comment() {
        let db = setup_test_db().await;
        let repo = CommentRepository::new(db);
        
        // 先创建一条评论
        let created_comment = create_test_comment(&repo, 1, 1).await.unwrap();
        
        // 更新评论内容
        let update_req = UpdateCommentRequest {
            content: "这是更新后的评论内容".to_string(),
        };
        
        let result = repo.update_comment(created_comment.id, update_req).await;
        
        assert!(result.is_ok());
        let updated_comment = result.unwrap();
        assert_eq!(updated_comment.content, "这是更新后的评论内容");
        assert_eq!(updated_comment.post_id, created_comment.post_id); // 其他字段不变
        assert_eq!(updated_comment.user_id, created_comment.user_id);
        assert_eq!(updated_comment.parent_id, created_comment.parent_id);
    }

    #[tokio::test]
    async fn test_update_comment_not_found() {
        let db = setup_test_db().await;
        let repo = CommentRepository::new(db);
        
        let update_req = UpdateCommentRequest {
            content: "不存在的评论".to_string(),
        };
        
        let result = repo.update_comment(999, update_req).await;
        assert!(result.is_err());
        
        if let Err(CustomError::NotFound(msg)) = result {
            assert!(msg.contains("999"));
            assert!(msg.contains("未找到"));
        } else {
            panic!("Expected NotFound error");
        }
    }

    #[tokio::test]
    async fn test_delete_comment() {
        let db = setup_test_db().await;
        let repo = CommentRepository::new(db);
        
        // 先创建一条评论
        let created_comment = create_test_comment(&repo, 1, 1).await.unwrap();
        
        // 删除评论
        let result = repo.delete_comment(created_comment.id).await;
        assert!(result.is_ok());
        
        // 验证评论已被删除
        let found_comment = repo.get_comment_by_id(created_comment.id).await.unwrap();
        assert!(found_comment.is_none());
    }

    #[tokio::test]
    async fn test_delete_comment_not_found() {
        let db = setup_test_db().await;
        let repo = CommentRepository::new(db);
        
        // 删除不存在的评论
        let result = repo.delete_comment(999).await;
        assert!(result.is_err());
        
        if let Err(CustomError::NotFound(msg)) = result {
            assert!(msg.contains("999"));
            assert!(msg.contains("未找到或已被删除"));
        } else {
            panic!("Expected NotFound error");
        }
    }

    #[tokio::test]
    async fn test_delete_parent_comment_with_children() {
        let db = setup_test_db().await;
        let repo = CommentRepository::new(db);
        
        // 创建父评论
        let parent_comment = create_test_comment(&repo, 1, 1).await.unwrap();
        
        // 创建子评论
        let child_comment = create_child_comment(&repo, 1, 2, parent_comment.id).await.unwrap();
        
        // 删除父评论
        let result = repo.delete_comment(parent_comment.id).await;
        assert!(result.is_ok());
        
        // 验证父评论已被删除
        let found_parent = repo.get_comment_by_id(parent_comment.id).await.unwrap();
        assert!(found_parent.is_none());
        
        // 根据数据库设计，子评论可能也会被级联删除（取决于外键约束）
        // 这里我们测试子评论的状态
        let found_child = repo.get_comment_by_id(child_comment.id).await.unwrap();
        // 如果设置了级联删除，子评论应该也被删除
        // 如果没有设置，子评论仍然存在但parent_id指向已删除的评论
        // 这里我们假设子评论仍然存在（具体行为取决于数据库约束设置）
    }

    #[tokio::test]
    async fn test_get_paginated_comments_by_post_id() {
        let db = setup_test_db().await;
        let repo = CommentRepository::new(db);
        
        let post_id = 1;
        
        // 为同一篇文章创建多条评论
        for i in 1..=5 {
            let create_req = CreateCommentRequest {
                content: format!("评论{}", i),
                post_id,
                parent_id: None,
            };
            repo.create_comment(create_req, i).await.unwrap();
        }
        
        // 为另一篇文章创建评论（不应该被查询到）
        let create_req = CreateCommentRequest {
            content: "其他文章的评论".to_string(),
            post_id: 2,
            parent_id: None,
        };
        repo.create_comment(create_req, 6).await.unwrap();
        
        // 测试分页查询
        let result = repo.get_paginated_comments_by_post_id(post_id, 1, 3).await;
        assert!(result.is_ok());
        
        let (comments, total_pages, current_page, total_comments) = result.unwrap();
        assert_eq!(comments.len(), 3); // 每页3条评论
        assert_eq!(current_page, 1);
        assert_eq!(total_comments, 5); // 总共5条评论（不包括其他文章的评论）
        assert_eq!(total_pages, 2); // 5条评论分2页
        
        // 验证所有返回的评论都属于指定文章
        for comment in comments {
            assert_eq!(comment.post_id, post_id);
        }
    }

    #[tokio::test]
    async fn test_get_paginated_comments_second_page() {
        let db = setup_test_db().await;
        let repo = CommentRepository::new(db);
        
        let post_id = 1;
        
        // 创建5条评论
        for i in 1..=5 {
            let create_req = CreateCommentRequest {
                content: format!("评论{}", i),
                post_id,
                parent_id: None,
            };
            repo.create_comment(create_req, i).await.unwrap();
        }
        
        // 查询第二页
        let result = repo.get_paginated_comments_by_post_id(post_id, 2, 3).await;
        assert!(result.is_ok());
        
        let (comments, total_pages, current_page, total_comments) = result.unwrap();
        assert_eq!(comments.len(), 2); // 第二页只有2条评论
        assert_eq!(current_page, 2);
        assert_eq!(total_comments, 5);
        assert_eq!(total_pages, 2);
    }

    #[tokio::test]
    async fn test_get_paginated_comments_empty_post() {
        let db = setup_test_db().await;
        let repo = CommentRepository::new(db);
        
        // 查询没有评论的文章
        let result = repo.get_paginated_comments_by_post_id(999, 1, 10).await;
        assert!(result.is_ok());
        
        let (comments, total_pages, current_page, total_comments) = result.unwrap();
        assert_eq!(comments.len(), 0);
        assert_eq!(current_page, 1);
        assert_eq!(total_comments, 0);
        assert_eq!(total_pages, 0);
    }

    #[tokio::test]
    async fn test_comment_timestamps() {
        let db = setup_test_db().await;
        let repo = CommentRepository::new(db);
        
        let before_creation = Utc::now().naive_utc();
        
        // 创建评论
        let created_comment = create_test_comment(&repo, 1, 1).await.unwrap();
        
        let after_creation = Utc::now().naive_utc();
        
        // 验证时间戳
        assert!(created_comment.created_at >= before_creation);
        assert!(created_comment.created_at <= after_creation);
    }

    #[tokio::test]
    async fn test_comment_hierarchy() {
        let db = setup_test_db().await;
        let repo = CommentRepository::new(db);
        
        // 创建父评论
        let parent_comment = create_test_comment(&repo, 1, 1).await.unwrap();
        
        // 创建多个子评论
        let child1 = create_child_comment(&repo, 1, 2, parent_comment.id).await.unwrap();
        let child2 = create_child_comment(&repo, 1, 3, parent_comment.id).await.unwrap();
        
        // 创建孙评论（子评论的子评论）
        let grandchild = create_child_comment(&repo, 1, 4, child1.id).await.unwrap();
        
        // 验证层级关系
        assert!(parent_comment.parent_id.is_none()); // 父评论没有父级
        assert_eq!(child1.parent_id, Some(parent_comment.id));
        assert_eq!(child2.parent_id, Some(parent_comment.id));
        assert_eq!(grandchild.parent_id, Some(child1.id)); // 孙评论的父级是child1
        
        // 验证所有评论都属于同一篇文章
        assert_eq!(parent_comment.post_id, 1);
        assert_eq!(child1.post_id, 1);
        assert_eq!(child2.post_id, 1);
        assert_eq!(grandchild.post_id, 1);
    }

    #[tokio::test]
    async fn test_comments_ordering() {
        let db = setup_test_db().await;
        let repo = CommentRepository::new(db);
        
        let post_id = 1;
        
        // 创建评论时添加延迟以确保时间戳不同
        let create_req1 = CreateCommentRequest {
            content: "第一条评论".to_string(),
            post_id,
            parent_id: None,
        };
        let comment1 = repo.create_comment(create_req1, 1).await.unwrap();
        
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        let create_req2 = CreateCommentRequest {
            content: "第二条评论".to_string(),
            post_id,
            parent_id: None,
        };
        let comment2 = repo.create_comment(create_req2, 2).await.unwrap();
        
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        let create_req3 = CreateCommentRequest {
            content: "第三条评论".to_string(),
            post_id,
            parent_id: None,
        };
        let comment3 = repo.create_comment(create_req3, 3).await.unwrap();
        
        // 获取评论列表（应该按创建时间倒序）
        let result = repo.get_paginated_comments_by_post_id(post_id, 1, 10).await;
        assert!(result.is_ok());
        
        let (comments, _, _, _) = result.unwrap();
        assert_eq!(comments.len(), 3);
        
        // 验证排序（最新的评论在前）
        assert_eq!(comments[0].content, "第三条评论");
        assert_eq!(comments[1].content, "第二条评论");
        assert_eq!(comments[2].content, "第一条评论");
        
        // 验证时间戳顺序
        assert!(comments[0].created_at >= comments[1].created_at);
        assert!(comments[1].created_at >= comments[2].created_at);
    }
}
// src/bin/server.rs
use axum::{Router, routing::post};
use tokio::net::TcpListener;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 删除可能冲突的环境变量
    std::env::remove_var("DATABASE_URL");
    std::env::remove_var("JWT_SECRET");

    let env_path = std::path::Path::new("./.env");
    dotenv::from_path(env_path).expect("无法加载.env文件");

    // 打印验证环境变量
    println!("Database URL: {}", std::env::var("DATABASE_URL").unwrap());

    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:blogdb.db?mode=rwc".into());
    
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default_secret".into());
    
    println!("JWT Secret: {}", std::env::var("JWT_SECRET").unwrap());
    println!("Current dir: {:?}", std::env::current_dir().unwrap());

    // 1. 初始化数据库
    let db = sea_orm::Database::connect(&db_url)
        .await
        .unwrap_or_else(|e| {
            panic!("数据库连接失败 ({}): {}", db_url, e);
        });

    // 2. 初始化应用状态
    let state = axum_blog_engine::AppState {
        db,
        config: std::sync::Arc::new(axum_blog_engine::Config {
            jwt_secret,
            bcrypt_cost: 12, // 默认值
        }),
    };

    // 3. 创建路由
    let app = Router::new()
        .nest(
            "/user",
            Router::new()
                .route("/register", post(axum_blog_engine::handler::user::register))
                .route("/login", post(axum_blog_engine::handler::user::login)),
        )
        .with_state(state);

    // 4. 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    
    println!("Server running on {}", addr);
    axum::serve(listener, app)
        .await
        .unwrap();
}
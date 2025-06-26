//! Axum Blog Engine 的主入口点。
//! 负责应用程序的初始化、路由聚合和服务器启动。

use axum::Router;
use tokio::net::TcpListener;
use std::{net::SocketAddr, sync::Arc, env, time::Duration};
use tracing::{debug, info, error, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sea_orm::{DatabaseConnection, ConnectOptions};
use anyhow::Result; // 用于 main 函数的错误处理
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;

// 从 axum_blog_engine 库导入 AppState 和 Config
use axum_blog_engine::{AppState, Config};

// 导入数据库 Repositories
use axum_blog_engine::database::user::UserRepository;
use axum_blog_engine::database::post::PostRepository;
use axum_blog_engine::database::comment::CommentRepository;
use axum_blog_engine::database::favorite::FavoriteRepository;

// 导入路由配置函数
use axum_blog_engine::handler::user::user_routes;
use axum_blog_engine::handler::post::post_routes;
use axum_blog_engine::handler::comment::comment_routes_for_post as post_comments_routes; // 为 /posts/:id/comments 导入一个别名
use axum_blog_engine::handler::comment::comment_routes;
use axum_blog_engine::handler::favorite::favorite_routes;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 初始化日志系统
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_blog_engine=debug,tower_http=debug,sea_orm=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("应用程序启动中...");

    // 2. 加载环境变量
    let env_path = std::path::Path::new("./.env");
    match dotenv::from_path(env_path) {
        Ok(_) => info!("已成功加载 .env 文件"),
        Err(e) => error!("无法加载 .env 文件: {}", e), // Non-fatal, as defaults might be used
    };

    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            info!("未找到 DATABASE_URL 环境变量，使用默认值：sqlite:blogdb.db");
            "sqlite:blogdb.db?mode=rwc".into() // 确保 SQLite URL 格式正确，rwc表示读写创建
        });
    
    let jwt_secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| {
            warn!("未找到 JWT_SECRET 环境变量，使用默认值：default_jwt_secret。生产环境请务必设置！");
            "default_jwt_secret".into() // 生产环境请务必使用强密钥
        });
    
    let bcrypt_cost_str = env::var("BCRYPT_COST")
        .unwrap_or_else(|_| "12".into());
    let bcrypt_cost = bcrypt_cost_str.parse::<u32>()
        .unwrap_or_else(|_| {
            warn!("无法解析 BCRYPT_COST 环境变量，使用默认值 12。");
            12
        });

    debug!("数据库 URL: {}", db_url);
    debug!("当前工作目录: {:?}", env::current_dir().unwrap_or_default());

    // 3. 初始化数据库连接
    let mut connect_options = ConnectOptions::new(db_url.clone());
    connect_options
        .max_connections(10)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true) // 开启 SQLX 日志
        .sqlx_logging_level(tracing::log::LevelFilter::Info); // 设置日志级别

    let db_connection: DatabaseConnection = sea_orm::Database::connect(connect_options)
        .await
        .map_err(|e| anyhow::anyhow!("数据库连接失败: {}", e))?;
    
    info!("数据库连接成功");

    // 运行数据库迁移 (如果使用 SeaORM 的迁移工具)
    // 假设你有一个独立的 SeaORM 迁移项目或通过 SeaORM CLI 管理迁移
    // 如果你还没有设置，可以暂时注释掉这行。
    // 例如：`sea_orm_migration::Migrator::run(&db_connection, Config::default()).await?;`

    // 4. 初始化应用状态
    let app_config = Arc::new(Config {
        jwt_secret,
        bcrypt_cost,
    });

    let app_state = AppState {
        db: db_connection.clone(),
        config: app_config.clone(),
        user_repo: UserRepository::new(db_connection.clone()),
        post_repo: PostRepository::new(db_connection.clone()),
        comment_repo: CommentRepository::new(db_connection.clone()),
        favorite_repo: FavoriteRepository::new(db_connection.clone()),
    };

    info!("应用状态初始化完成");

    // 5. 构建 Axum 路由 (遵循 RESTful 风格)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any);

    let app = Router::new()
        // API 路由前缀
        .nest("/api", Router::new()
            // 用户相关路由
            .nest("/user", user_routes())
            // 帖子相关路由
            .nest("/post", post_routes())
            // 文章评论路由 (嵌套在文章路由下)
            .nest("/post/:post_id/comments", post_comments_routes())
            // 独立的评论资源路由 (例如 POST /comments, GET /comments/:id)
            .nest("/comment", comment_routes())
            // 收藏相关路由
            .nest("/post_fav", favorite_routes()) // 你的 API 路径是 /post_fav
        )
        // 添加 CORS 中间件
        .layer(cors)
        // 可以添加其他全局中间件，例如 TraceLayer 用于请求日志
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(app_state); // 注入应用程序状态

    debug!("路由配置完成");

    // 6. 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000)); // 监听所有网络接口的 3000 端口
    let listener = TcpListener::bind(addr).await
        .map_err(|e| anyhow::anyhow!("无法绑定监听地址 {}: {}", addr, e))?;
    
    info!("服务器正在监听 {}", addr);
    axum::serve(listener, app)
        .await
        .map_err(|e| anyhow::anyhow!("服务器启动失败: {}", e))?;

    info!("服务器已关闭");
    Ok(())
}

mod database;
pub mod error;
pub mod handler;

// src/lib.rs
pub mod app_state {
    use std::sync::Arc;
    use sea_orm::DatabaseConnection;

    #[derive(Clone)]
    pub struct AppState {
        pub db: DatabaseConnection,
        pub config: Arc<Config>,
    }

    pub struct Config {
        pub jwt_secret: String,
        pub bcrypt_cost: u32,
        // 其他配置...
    }
}

// 导出主要类型
pub use app_state::{AppState, Config};
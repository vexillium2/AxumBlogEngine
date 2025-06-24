// 声明 orm 模块，通常包含数据库连接初始化逻辑
mod orm;

// 声明并公开 user 模块，包含 UserRepository 和用户实体定义
pub(crate) mod user;

// 声明并公开 post 模块，包含 PostRepository 和文章实体定义
pub(crate) mod post;

// 声明并公开 comment 模块，包含 CommentRepository 和评论实体定义
pub(crate) mod comment;

// 声明并公开 favorite 模块，包含 FavoriteRepository 和收藏实体定义
pub(crate) mod favorite;

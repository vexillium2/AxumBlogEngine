# 博客引擎

[English](README.md) | 中文

## 环境
- Rust
- Sqlite 

## 使用方法
1. 创建数据库并执行`sqlite3.exe blogdb.db ".read init.sql"`创建数据库表。
2. 在 `.env` 设置环境变量 `DATABASE_URL` 和 `JWT_SECRET`。
3. 执行 `cargo run --bin server`。


## API

后端接口使用/api/开头，例如`/api/user`。

### 用户
- /user/register        用户注册
- /user/login           用户登录
- /user/logout          用户登出
- /user/get/me       获取登录用户信息
- /user/update/me       更新个人信息
- /user/get             获取用户信息（管理员）
- /user/create          添加用户（管理员）
- /user/update          更新用户信息（管理员）
- /user/delete          删除用户信息（管理员）

### 帖子
> 需要在 header 中设置 JWT `Authorization: Bearer <JWT>`
- /post/create   创建文章
- /post/search   搜索文章
- /post/update   更新文章
- /post/delete   删除文章
- /post/get      获取文章（根据id）
- /post/list     分页获取文章列表
- /post/edit     编辑文章

### 帖子收藏
- /post_fav             收藏/取消收藏文章
- /post_fav/my/list     获取本用户收藏的文章信息

### 评论
- /comment/create   创建评论
- /comment/update   更新评论
- /comment/delete   删除评论
- /comment/get      获取评论（根据id）
- /comment/list     分页获取评论列表

## 后端框架
- 数据层 database
- 业务层 handler
- 路由层 /bin/server.rs

依赖位于 Cargo.toml 中。

## 开发帮助文档
- [rust](https://photino.gitbooks.io/rust-notes/content/memory-safety.html)
- [axum](https://docs.rs/axum/latest/axum/)
- [seaorm](https://www.sea-ql.org/SeaORM/docs/introduction/tutorial/)

## 用 Redis 作为缓存
1. 添加 [redis](https://github.com/redis-rs/redis-rs) 到 Cargo.toml，需要开启 feature `tokio-comp`。
> 必须使用 `async`，因为如果不用 `async`，**系统线程**会在这一行代码执行时阻塞，在执行完成前不会执行其他任务。
2. 在 src/error.rs 添加 `redis::RedisError` 。
```rust
    #[error("redis_error: {0}")]
    Redis(#[from] redis::RedisError),
```
> 如果加了 `#[from]`，[thiserror](https://github.com/dtolnay/thiserror) 会自动生成 `impl From<redis::RedisError> for CustomError`。在错误类型是`CustomResult<T>`时，你可以用 `?` 或 `.map_err(Into::into)?` 返回错误。  
> 如果不加 `#[from]`，你需要自己转换错误类型 `.map_err(|e| CustomError::Redis(e))` 或 `.map_err(CustomError::Redis)`。
3. 阅读代码 [redis/examples](https://github.com/redis-rs/redis-rs/blob/main/redis/examples/async-await.rs)。
4. 编写缓存代码。

### 全局 404 处理器
1. 在 `src/bin/server.rs` 中，添加 [Global-404-handler](https://github.com/tokio-rs/axum/tree/main/examples/global-404-handler)。

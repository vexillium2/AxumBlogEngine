// src/handler/mod.rs
//! 处理程序模块的顶层文件，声明并公开其子模块。

pub mod auth; // 认证相关逻辑
pub mod idl;  // 数据传输对象 (DTOs) - 修复：现在是 public
pub mod user; // 用户相关的 HTTP 请求处理函数
pub mod post; // 文章相关的 HTTP 请求处理函数
pub mod comment; // 评论相关的 HTTP 请求处理函数
pub mod favorite; // 收藏相关的 HTTP 请求处理函数

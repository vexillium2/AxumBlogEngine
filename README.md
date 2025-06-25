# Axum Blog Engine

一个基于 Rust Axum 框架构建的现代化轻量级博客引擎，提供完整的博客功能和 RESTful API。

[English](README_EN.md) | **中文**

## ✨ 特性

- 🚀 **高性能**: 基于 Rust 和 Axum 框架，提供卓越的性能和并发处理能力
- 🔐 **安全认证**: JWT 令牌认证，bcrypt 密码加密
- 📝 **完整博客功能**: 文章创建、编辑、分类、搜索
- 💬 **评论系统**: 支持嵌套评论和回复
- ⭐ **收藏功能**: 用户可以收藏喜欢的文章
- 👥 **用户管理**: 用户注册、登录、权限管理
- 🗄️ **轻量数据库**: 使用 SQLite，易于部署和维护
- 📊 **分页查询**: 所有列表接口支持分页
- 🧪 **完整测试**: 60+ 测试用例，覆盖所有核心功能
- 📖 **RESTful API**: 遵循 REST 设计原则的 API 接口

## 🛠️ 技术栈

- **后端框架**: [Axum](https://github.com/tokio-rs/axum) - 现代化的 Rust Web 框架
- **数据库**: SQLite - 轻量级嵌入式数据库
- **ORM**: [SeaORM](https://www.sea-ql.org/SeaORM/) - Rust 异步 ORM
- **认证**: JWT (JSON Web Tokens) + bcrypt 密码哈希
- **序列化**: Serde - JSON 序列化/反序列化
- **日志**: Tracing - 结构化日志记录
- **验证**: Validator - 请求数据验证
- **测试**: Tokio Test - 异步测试框架

## 📋 环境要求

- **Rust**: 1.70+ (推荐使用最新稳定版)
- **SQLite**: 3.35+
- **操作系统**: Linux, macOS, Windows

## 🚀 快速开始

### 1. 克隆项目

```bash
git clone <repository-url>
cd AxumBlogEngine
```

### 2. 环境配置

创建 `.env` 文件并配置环境变量：

```env
# 数据库连接字符串
DATABASE_URL=sqlite:blogdb.db?mode=rwc

# JWT 密钥（生产环境请使用强密钥）
JWT_SECRET=your_super_secret_jwt_key_here

# bcrypt 成本因子（推荐 12-15）
BCRYPT_COST=12
```

### 3. 初始化数据库

```bash
# 创建数据库并执行初始化脚本
sqlite3 blogdb.db ".read init.sql"
```

### 4. 运行项目

```bash
# 开发模式运行
cargo run --bin server

# 或者构建后运行
cargo build --release
./target/release/server
```

服务器将在 `http://localhost:3000` 启动。

### 5. 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test user_test
cargo test post_test
cargo test comment_test
cargo test favorite_test
```

## 📚 API 文档

### 认证

大部分 API 需要在请求头中包含 JWT 令牌：

```
Authorization: Bearer <your_jwt_token>
```

### 用户管理

| 方法 | 路径 | 描述 | 认证 |
|------|------|------|------|
| POST | `/users/register` | 用户注册 | ❌ |
| POST | `/users/login` | 用户登录 | ❌ |
| POST | `/users/logout` | 用户登出 | ✅ |
| GET | `/users/me` | 获取当前用户信息 | ✅ |
| PUT | `/users/me` | 更新个人信息 | ✅ |
| GET | `/users/:id` | 获取用户信息（管理员） | ✅ |
| POST | `/users` | 创建用户（管理员） | ✅ |
| PUT | `/users/:id` | 更新用户（管理员） | ✅ |
| DELETE | `/users/:id` | 删除用户（管理员） | ✅ |

### 文章管理

| 方法 | 路径 | 描述 | 认证 |
|------|------|------|------|
| POST | `/posts` | 创建文章 | ✅ |
| GET | `/posts` | 获取文章列表（支持搜索和分页） | ❌ |
| GET | `/posts/:id` | 获取文章详情 | ❌ |
| PUT | `/posts/:id` | 更新文章 | ✅ |
| DELETE | `/posts/:id` | 删除文章 | ✅ |

### 评论管理

| 方法 | 路径 | 描述 | 认证 |
|------|------|------|------|
| POST | `/comments` | 创建评论 | ✅ |
| GET | `/comments/:id` | 获取评论详情 | ❌ |
| PUT | `/comments/:id` | 更新评论 | ✅ |
| DELETE | `/comments/:id` | 删除评论 | ✅ |
| GET | `/posts/:post_id/comments` | 获取文章评论列表 | ❌ |

### 收藏管理

| 方法 | 路径 | 描述 | 认证 |
|------|------|------|------|
| POST | `/post_fav` | 收藏/取消收藏文章 | ✅ |
| GET | `/post_fav/my/list` | 获取我的收藏列表 | ✅ |

## 🏗️ 项目结构

```
AxumBlogEngine/
├── src/
│   ├── bin/
│   │   └── server.rs          # 应用程序入口点
│   ├── database/              # 数据层
│   │   ├── mod.rs
│   │   ├── user.rs           # 用户数据模型和仓库
│   │   ├── post.rs           # 文章数据模型和仓库
│   │   ├── comment.rs        # 评论数据模型和仓库
│   │   └── favorite.rs       # 收藏数据模型和仓库
│   ├── handler/               # 业务层
│   │   ├── mod.rs
│   │   ├── auth.rs           # 认证相关
│   │   ├── user.rs           # 用户处理器
│   │   ├── post.rs           # 文章处理器
│   │   ├── comment.rs        # 评论处理器
│   │   ├── favorite.rs       # 收藏处理器
│   │   └── idl.rs            # 数据传输对象定义
│   ├── error.rs              # 错误处理
│   └── lib.rs                # 库入口
├── tests/                     # 测试文件
│   ├── user_test.rs          # 用户模块测试
│   ├── post_test.rs          # 文章模块测试
│   ├── comment_test.rs       # 评论模块测试
│   ├── favorite_test.rs      # 收藏模块测试
│   └── TESTING_GUIDE.md      # 测试指南
├── init.sql                   # 数据库初始化脚本
├── Cargo.toml                # 项目配置和依赖
└── README.md                 # 项目说明文档
```

## 🗄️ 数据库设计

### 核心表结构

- **users**: 用户信息表
- **posts**: 文章信息表
- **comments**: 评论信息表（支持嵌套）
- **favorites**: 用户收藏表

### 关系设计

- 用户与文章：一对多关系
- 用户与评论：一对多关系
- 文章与评论：一对多关系
- 用户与收藏：多对多关系（通过 favorites 表）
- 评论支持自引用（parent_id）实现嵌套回复

## 🧪 测试

项目包含完整的测试套件，覆盖所有核心功能：

- **用户模块**: 15个测试用例
- **文章模块**: 16个测试用例
- **评论模块**: 15个测试用例
- **收藏模块**: 14个测试用例

总计 **60个测试用例**，确保代码质量和功能稳定性。

详细测试指南请参考：[tests/TESTING_GUIDE.md](tests/TESTING_GUIDE.md)

## 🔧 开发指南

### 添加新功能

1. 在 `src/database/` 中定义数据模型和仓库
2. 在 `src/handler/idl.rs` 中定义 DTO
3. 在 `src/handler/` 中实现业务逻辑
4. 在 `src/bin/server.rs` 中注册路由
5. 在 `tests/` 中添加测试用例

### 代码规范

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 Rust 官方命名规范
- 为公共 API 编写文档注释

## 🚀 部署

### Docker 部署（推荐）

```dockerfile
# Dockerfile 示例
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y sqlite3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/server .
COPY init.sql .
EXPOSE 3000
CMD ["./server"]
```

### 传统部署

1. 编译项目：`cargo build --release`
2. 复制二进制文件到服务器
3. 配置环境变量
4. 初始化数据库
5. 启动服务

## 🔮 未来计划

- [ ] Redis 缓存支持
- [ ] 全局 404 处理器
- [ ] 文件上传功能
- [ ] 邮件通知系统
- [ ] 文章标签系统
- [ ] 搜索功能增强
- [ ] API 限流
- [ ] 管理后台界面

## 📖 参考文档

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Axum 框架文档](https://docs.rs/axum/latest/axum/)
- [SeaORM 文档](https://www.sea-ql.org/SeaORM/docs/introduction/tutorial/)
- [SQLite 文档](https://www.sqlite.org/docs.html)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本项目
2. 创建特性分支：`git checkout -b feature/amazing-feature`
3. 提交更改：`git commit -m 'Add some amazing feature'`
4. 推送分支：`git push origin feature/amazing-feature`
5. 提交 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 👨‍💻 作者

Axum Blog Engine 开发团队

---

⭐ 如果这个项目对你有帮助，请给它一个星标！

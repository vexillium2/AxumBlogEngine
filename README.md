# 🚀 Axum Blog Engine

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Vue.js](https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vue.js&logoColor=4FC08D)
![SQLite](https://img.shields.io/badge/SQLite-07405E?style=for-the-badge&logo=sqlite&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-000000?style=for-the-badge&logo=rust&logoColor=white)

**一个基于 Axum 框架构建的现代化轻量级博客引擎**

[English](README_EN.md) | 中文

</div>

## ✨ 特性

- 🔐 **用户认证系统** - JWT 令牌认证，支持用户注册、登录、权限管理
- 📝 **文章管理** - 支持 Markdown 编辑，文章分类，草稿/发布状态
- 💬 **评论系统** - 支持嵌套评论，实时互动
- ⭐ **收藏功能** - 用户可收藏喜欢的文章
- 🔍 **搜索功能** - 全文搜索文章内容
- 👑 **管理员面板** - 用户管理，内容审核
- 📱 **响应式设计** - 适配各种设备屏幕
- ⚡ **高性能** - 基于 Rust 异步框架，性能卓越

## 🛠️ 技术栈

### 后端
- **框架**: [Axum](https://github.com/tokio-rs/axum) - 现代化的 Rust Web 框架
- **数据库**: SQLite - 轻量级嵌入式数据库
- **ORM**: [SeaORM](https://www.sea-ql.org/SeaORM/) - Rust 异步 ORM
- **认证**: JWT (JSON Web Tokens)
- **密码加密**: bcrypt
- **日志**: tracing + tracing-subscriber
- **序列化**: serde + serde_json
- **验证**: validator

### 前端
- **框架**: [Vue 3](https://vuejs.org/) - 渐进式 JavaScript 框架
- **UI 库**: [Element Plus](https://element-plus.org/) - 基于 Vue 3 的组件库
- **构建工具**: [Vite](https://vitejs.dev/) - 下一代前端构建工具
- **路由**: Vue Router 4
- **样式**: Less

## 📋 系统要求

- **Rust**: 1.70.0 或更高版本
- **Node.js**: 16.0.0 或更高版本
- **SQLite**: 3.35.0 或更高版本

## 🚀 快速开始

### 1. 克隆项目

```bash
git clone https://github.com/your-username/AxumBlogEngine.git
cd AxumBlogEngine
```

### 2. 后端设置

#### 安装依赖
```bash
cd backend
cargo build
```

#### 数据库初始化
```bash
# 创建数据库文件
sqlite3 blogdb.db ".read init.sql"
```

#### 环境配置
创建 `.env` 文件：
```env
DATABASE_URL=sqlite:blogdb.db
JWT_SECRET=your-super-secret-jwt-key-here
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
```

#### 启动后端服务
```bash
cargo run --bin server
```

服务将在 `http://localhost:3000` 启动

### 3. 前端设置

```bash
cd frontend
npm install
npm run dev
```

前端将在 `http://localhost:5173` 启动

### 4. 默认管理员账户

- **用户名**: `admin`
- **邮箱**: `admin@example.com`
- **密码**: `admin123`

> ⚠️ **安全提示**: 请在生产环境中立即修改默认密码！

## 📚 API 文档

### 认证相关

| 方法 | 端点 | 描述 | 认证 |
|------|------|------|------|
| POST | `/api/user/register` | 用户注册 | ❌ |
| POST | `/api/user/login` | 用户登录 | ❌ |
| POST | `/api/user/logout` | 用户登出 | ✅ |
| GET | `/api/user/get/me` | 获取当前用户信息 | ✅ |
| PUT | `/api/user/update/me` | 更新个人信息 | ✅ |

### 文章管理

| 方法 | 端点 | 描述 | 认证 |
|------|------|------|------|
| POST | `/api/post/create` | 创建文章 | ✅ |
| GET | `/api/post/list` | 获取文章列表 | ❌ |
| GET | `/api/post/get/:id` | 获取文章详情 | ❌ |
| PUT | `/api/post/update/:id` | 更新文章 | ✅ |
| DELETE | `/api/post/delete/:id` | 删除文章 | ✅ |
| GET | `/api/post/search` | 搜索文章 | ❌ |

### 评论系统

| 方法 | 端点 | 描述 | 认证 |
|------|------|------|------|
| POST | `/api/comment/create` | 创建评论 | ✅ |
| GET | `/api/comment/list/:post_id` | 获取文章评论 | ❌ |
| PUT | `/api/comment/update/:id` | 更新评论 | ✅ |
| DELETE | `/api/comment/delete/:id` | 删除评论 | ✅ |

### 收藏功能

| 方法 | 端点 | 描述 | 认证 |
|------|------|------|------|
| POST | `/api/post_fav` | 收藏/取消收藏文章 | ✅ |
| GET | `/api/post_fav/my/list` | 获取我的收藏列表 | ✅ |

### 管理员功能

| 方法 | 端点 | 描述 | 认证 |
|------|------|------|------|
| GET | `/api/user/get/:id` | 获取用户信息 | 👑 |
| POST | `/api/user/create` | 创建用户 | 👑 |
| PUT | `/api/user/update/:id` | 更新用户信息 | 👑 |
| DELETE | `/api/user/delete/:id` | 删除用户 | 👑 |

> 📝 **认证说明**:
> - ❌ 无需认证
> - ✅ 需要 JWT 令牌
> - 👑 需要管理员权限
>
> **请求头格式**: `Authorization: Bearer <JWT_TOKEN>`

## 🏗️ 项目结构

```
AxumBlogEngine/
├── backend/                 # 后端代码
│   ├── src/
│   │   ├── bin/
│   │   │   └── server.rs   # 服务器入口
│   │   ├── database/       # 数据层
│   │   │   ├── user.rs
│   │   │   ├── post.rs
│   │   │   ├── comment.rs
│   │   │   └── favorite.rs
│   │   ├── handler/        # 业务逻辑层
│   │   │   ├── user.rs
│   │   │   ├── post.rs
│   │   │   ├── comment.rs
│   │   │   └── favorite.rs
│   │   ├── error.rs        # 错误处理
│   │   └── lib.rs          # 库入口
│   ├── tests/              # 测试文件
│   ├── init.sql            # 数据库初始化脚本
│   ├── Cargo.toml          # Rust 依赖配置
│   └── .env                # 环境变量
├── frontend/               # 前端代码
│   ├── src/
│   │   ├── view/           # 页面组件
│   │   ├── router/         # 路由配置
│   │   ├── assets/         # 静态资源
│   │   ├── App.vue         # 根组件
│   │   └── main.js         # 入口文件
│   ├── package.json        # Node.js 依赖配置
│   └── vite.config.js      # Vite 配置
└── README.md               # 项目说明
```

## 🧪 测试

### 运行所有测试
```bash
cd backend
cargo test
```

### 运行特定模块测试
```bash
# 用户模块测试
cargo test user_test

# 文章模块测试
cargo test post_test

# 评论模块测试
cargo test comment_test

# 收藏模块测试
cargo test favorite_test
```

### 运行单个测试
```bash
cargo test test_create_user_from_register
```

## 🔧 开发指南

### 添加新功能

1. **数据层**: 在 `backend/src/database/` 中添加数据模型
2. **业务层**: 在 `backend/src/handler/` 中添加业务逻辑
3. **路由层**: 在 `backend/src/bin/server.rs` 中注册路由
4. **前端**: 在 `frontend/src/view/` 中添加页面组件

### 代码规范

- 使用 `cargo fmt` 格式化 Rust 代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 RESTful API 设计原则
- 编写单元测试和集成测试

## 🚀 部署

### 生产环境部署

1. **构建后端**
```bash
cd backend
cargo build --release
```

2. **构建前端**
```bash
cd frontend
npm run build
```

3. **配置环境变量**
```env
DATABASE_URL=sqlite:blogdb.db
JWT_SECRET=your-production-secret
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
```

4. **启动服务**
```bash
./target/release/server
```

### Docker 部署 (可选)

```dockerfile
# Dockerfile 示例
FROM rust:1.70 as builder
WORKDIR /app
COPY backend/ .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y sqlite3
COPY --from=builder /app/target/release/server /usr/local/bin/server
COPY backend/init.sql /app/
EXPOSE 3000
CMD ["server"]
```

## 🔮 未来计划

- [ ] 🔄 Redis 缓存支持
- [ ] 📧 邮件通知系统
- [ ] 🖼️ 图片上传功能
- [ ] 🏷️ 标签系统
- [ ] 📊 数据统计面板
- [ ] 🌐 多语言支持
- [ ] 📱 移动端 App
- [ ] 🔌 插件系统

## 📖 参考文档

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Axum 框架文档](https://docs.rs/axum/latest/axum/)
- [SeaORM 文档](https://www.sea-ql.org/SeaORM/docs/introduction/tutorial/)
- [Vue 3 文档](https://vuejs.org/guide/)
- [Element Plus 文档](https://element-plus.org/)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 👥 作者

- **Axum Blog Engine 开发团队**

## 🙏 致谢

感谢所有为这个项目做出贡献的开发者！

---

<div align="center">

**如果这个项目对你有帮助，请给它一个 ⭐ Star！**

</div>

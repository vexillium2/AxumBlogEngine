# 测试指南

本文档提供了 Axum Blog Engine 项目的完整测试覆盖情况和测试运行指南。

## 测试概览

我们为博客引擎的核心数据库模块编写了全面的测试用例，确保所有关键功能都经过充分验证。

### 测试模块

| 模块 | 测试文件 | 测试数量 | 覆盖功能 |
|------|----------|----------|----------|
| 用户模块 | `user_test.rs` | 15个 | 用户CRUD、认证、权限管理 |
| 文章模块 | `post_test.rs` | 16个 | 文章CRUD、分页、搜索、浏览量 |
| 评论模块 | `comment_test.rs` | 15个 | 评论CRUD、嵌套评论、分页 |
| 收藏模块 | `favorite_test.rs` | 14个 | 收藏切换、分页查询、用户关联 |

**总计：60个测试用例**

## 测试环境

- **数据库**: SQLite 内存数据库 (`sqlite::memory:`)
- **测试框架**: Tokio 异步测试
- **ORM**: SeaORM
- **测试隔离**: 每个测试使用独立的内存数据库实例

## 运行测试

### 运行所有测试
```bash
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

### 显示测试输出
```bash
cargo test -- --nocapture
```

### 运行单个测试
```bash
cargo test test_create_user_from_register
```

## 测试覆盖详情

### 1. 用户模块 (`user_test.rs`)

**核心功能测试：**
- ✅ 用户注册（普通用户和管理员创建）
- ✅ 用户查询（按ID、用户名、邮箱）
- ✅ 用户信息更新（个人资料、管理员操作）
- ✅ 用户删除
- ✅ 数据完整性验证

**边界条件测试：**
- ✅ 重复用户名/邮箱处理
- ✅ 不存在用户的操作
- ✅ 无效角色验证
- ✅ 时间戳验证

### 2. 文章模块 (`post_test.rs`)

**核心功能测试：**
- ✅ 文章创建（草稿和发布状态）
- ✅ 文章查询（按ID）
- ✅ 文章更新（部分更新支持）
- ✅ 文章删除（单个和批量）
- ✅ 分页查询（支持分类过滤和搜索）
- ✅ 浏览量统计

**边界条件测试：**
- ✅ 不存在文章的操作
- ✅ 空搜索结果
- ✅ 分页边界情况
- ✅ 时间戳验证

### 3. 评论模块 (`comment_test.rs`)

**核心功能测试：**
- ✅ 评论创建（顶级和嵌套评论）
- ✅ 评论查询（按ID）
- ✅ 评论更新
- ✅ 评论删除
- ✅ 分页查询（按文章ID）

**特殊功能测试：**
- ✅ 嵌套评论层级关系
- ✅ 父评论删除对子评论的影响
- ✅ 评论排序（按创建时间）
- ✅ 空文章评论列表

### 4. 收藏模块 (`favorite_test.rs`)

**核心功能测试：**
- ✅ 收藏状态切换（添加/取消）
- ✅ 收藏状态查询
- ✅ 用户收藏列表分页查询

**复杂场景测试：**
- ✅ 多用户收藏同一文章
- ✅ 单用户收藏多篇文章
- ✅ 收藏状态的多次切换
- ✅ 不同用户的收藏隔离
- ✅ 边界条件（不存在的用户/文章）

## 测试数据设计

### 数据库表结构

每个测试模块都会创建相应的数据库表：

```sql
-- 用户表
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'user',
    avatar_url TEXT,
    bio TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- 文章表
CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content_markdown TEXT NOT NULL,
    category TEXT NOT NULL,
    author_id INTEGER NOT NULL,
    is_published INTEGER NOT NULL DEFAULT 0,
    view_count INTEGER NOT NULL DEFAULT 0,
    cover_url TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- 评论表
CREATE TABLE comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,
    post_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    parent_id INTEGER,
    created_at TEXT NOT NULL
);

-- 收藏表
CREATE TABLE favorites (
    user_id INTEGER NOT NULL,
    post_id INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    PRIMARY KEY (user_id, post_id)
);
```

### 测试数据特点

- **隔离性**: 每个测试使用独立的内存数据库
- **真实性**: 模拟真实的业务场景和数据
- **完整性**: 覆盖正常流程和异常情况
- **时间性**: 验证时间戳的正确性

## 测试最佳实践

### 1. 测试命名规范
- 使用描述性的测试函数名
- 格式：`test_<功能>_<场景>`
- 例如：`test_create_user_from_register`

### 2. 测试结构
```rust
#[tokio::test]
async fn test_function_name() {
    // 1. 准备测试环境
    let db = setup_test_db().await;
    let repo = Repository::new(db);
    
    // 2. 执行测试操作
    let result = repo.some_operation().await;
    
    // 3. 验证结果
    assert!(result.is_ok());
    assert_eq!(expected, actual);
}
```

### 3. 断言策略
- 验证操作成功性：`assert!(result.is_ok())`
- 验证返回值：`assert_eq!(expected, actual)`
- 验证状态变化：查询数据库确认
- 验证边界条件：测试极端情况

## 持续集成

### GitHub Actions 配置示例
```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --verbose
```

## 性能考虑

- **内存数据库**: 测试速度快，无需清理
- **并行执行**: Cargo 默认并行运行测试
- **资源隔离**: 每个测试独立，避免相互影响

## 未来扩展

### 计划添加的测试
1. **集成测试**: 测试 HTTP API 端点
2. **性能测试**: 大数据量下的性能表现
3. **并发测试**: 多用户并发操作
4. **安全测试**: SQL注入、XSS等安全漏洞

### 测试工具升级
1. **测试覆盖率**: 使用 `cargo-tarpaulin`
2. **基准测试**: 使用 `criterion`
3. **模糊测试**: 使用 `cargo-fuzz`

## 故障排除

### 常见问题

1. **编译错误**
   ```bash
   cargo check
   ```

2. **测试失败**
   ```bash
   cargo test -- --nocapture
   ```

3. **依赖问题**
   ```bash
   cargo update
   ```

### 调试技巧

- 使用 `println!` 或 `dbg!` 宏输出调试信息
- 设置 `RUST_BACKTRACE=1` 查看详细错误堆栈
- 使用 `cargo test -- --test-threads=1` 串行运行测试

## 贡献指南

### 添加新测试
1. 在相应的测试文件中添加测试函数
2. 遵循现有的命名和结构规范
3. 确保测试的独立性和可重复性
4. 添加必要的文档注释

### 测试审查清单
- [ ] 测试名称清晰描述功能
- [ ] 覆盖正常和异常情况
- [ ] 使用适当的断言
- [ ] 测试独立且可重复
- [ ] 添加必要的注释

---

**最后更新**: 2024年
**维护者**: Axum Blog Engine 开发团队
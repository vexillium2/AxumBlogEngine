PRAGMA foreign_keys = ON;  -- 启用外键约束，确保数据完整性
PRAGMA journal_mode = WAL; -- 提高并发性能，尤其是在高写入负载下

-- 用户表（对应/database/user.rs）
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL CHECK(length(username) >= 3), -- 用户名唯一且长度至少为3
    email TEXT UNIQUE NOT NULL CHECK(email LIKE '%@%.%'),      -- 邮箱唯一且格式校验
    password_hash TEXT NOT NULL CHECK(length(password_hash) > 0), -- 密码哈希不能为空
    role TEXT NOT NULL DEFAULT 'user' CHECK(role IN ('user', 'admin')), -- 用户角色，只能是 'user' 或 'admin'
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now')), -- 创建时间
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now'))  -- 更新时间
);

-- 文章表（对应/database/post.rs）
-- 存储文章的元数据和Markdown内容
CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,                -- 文章标题
    content_markdown TEXT NOT NULL,     -- 存储文章的 Markdown 内容
    category TEXT NOT NULL DEFAULT '未分类', -- 文章分类，作为文本直接存储
    author_id INTEGER NOT NULL,         -- 作者的用户ID
    is_published INTEGER NOT NULL DEFAULT 0, -- 是否已发布 (0: 草稿, 1: 已发布)
    view_count INTEGER NOT NULL DEFAULT 0, -- 浏览量
    cover_url TEXT,                     -- 文章封面图片URL (可选)
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now')), -- 创建时间
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now')), -- 更新时间
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE -- 作者删除时，其所有文章也删除
);

-- 评论表（对应/database/comment.rs）
-- 存储用户对文章的评论
CREATE TABLE IF NOT EXISTS comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,              -- 评论内容
    post_id INTEGER NOT NULL,           -- 评论所属的文章ID (已更名为 post_id)
    user_id INTEGER NOT NULL,           -- 评论用户的ID
    parent_id INTEGER,                  -- 父评论ID，用于实现嵌套评论 (可选，NULL表示顶级评论)
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now')), -- 创建时间
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,  -- 文章删除时，其所有评论也删除
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,  -- 用户删除时，其所有评论也删除
    FOREIGN KEY (parent_id) REFERENCES comments(id) ON DELETE CASCADE -- 父评论删除时，子评论也删除
);

-- 收藏表（对应/database/favorite.rs）
-- 存储用户收藏的文章
CREATE TABLE IF NOT EXISTS favorites (
    user_id INTEGER NOT NULL,           -- 收藏该文章的用户ID
    post_id INTEGER NOT NULL,           -- 被收藏的文章ID (已更名为 post_id)
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now')), -- 收藏时间
    PRIMARY KEY (user_id, post_id),     -- 联合主键，确保一个用户不能重复收藏同一篇文章
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,  -- 用户删除时，其收藏记录也删除
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE   -- 文章删除时，所有收藏该文章的记录也删除
);

-- 创建索引，提高查询性能
CREATE INDEX idx_posts_author ON posts(author_id);   -- 按作者查询文章
CREATE INDEX idx_posts_category ON posts(category);   -- 按分类查询文章
CREATE INDEX idx_comments_post ON comments(post_id);  -- 按文章查询评论
CREATE INDEX idx_comments_user ON comments(user_id);  -- 按用户查询评论
CREATE INDEX idx_favorites_user ON favorites(user_id); -- 按用户查询收藏

-- 添加管理员用户（初始数据）
INSERT OR IGNORE INTO users (username, email, password_hash, role)
VALUES (
    'admin',
    'admin@example.com',
    -- 密码是 "admin123" 的 bcrypt hash (安全提示：在生产环境中请勿硬编码密码)
    '$2b$12$Yj7ZULN2V9gKGDKkC5ZQe.BQ9X9XZ3JZ8X9JXvX9JXvX9JXvX9JXv',
    'admin'
);

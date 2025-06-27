<template>
  <div>
    <button class="btn btn-secondary mb-3 mt-3" @click="$emit('back-to-home')">
      返回首页
    </button>
    <div class="card" v-if="currentPost">
      <div class="card-header">
        <h2 class="card-title" style="margin: 0 auto">
          {{ currentPost.title }}
        </h2>
        <div v-if="isAuthor(currentPost.author_id)">
          <button class="btn btn-secondary" @click="editPost(currentPost.id)">
            编辑
          </button>
          <button class="btn btn-danger" @click="deletePost(currentPost.id)">
            删除
          </button>
        </div>
      </div>

      <div class="post-meta mb-3">
        <span>作者: {{ currentPost.author }}</span>
        <span>发布于: {{ formatDate(currentPost.created_at) }}</span>
        <span v-if="currentPost.updated_at !== currentPost.created_at">
          更新于: {{ formatDate(currentPost.updated_at) }}
        </span>
      </div>

      <div class="post-tags mb-3">
        <span class="tag" v-for="tag in currentPost.tags" :key="tag">
          #{{ tag }}
        </span>
      </div>

      <div class="post-content" v-html="currentPost.content"></div>

      <div class="post-actions mt-3">
        <button
          class="btn btn-secondary"
          @click="toggleFavorite(currentPost.id)"
          v-if="isAuthenticated"
        >
          <i
            class="far fa-heart"
            :class="{ fas: isFavorite(currentPost.id) }"
          ></i>
          {{ isFavorite(currentPost.id) ? "取消收藏" : "收藏" }}
        </button>
      </div>
    </div>

    <div v-else>
      <p>文章不存在或已删除</p>
    </div>

    <!-- 评论区域 -->
    <div class="card mt-3">
      <h3 class="card-title">评论</h3>

      <div class="comment-list">
        <div
          class="comment-item"
          v-for="comment in testComments"
          :key="comment.id"
        >
          <div class="d-flex justify-between">
            <div>
              <div class="comment-author">{{ comment.author }}</div>
              <div class="comment-date">
                {{ formatDate(comment.created_at) }}
              </div>
            </div>
            <div v-if="isAuthor(comment.author_id)">
              <button
                class="btn btn-sm btn-secondary"
                @click="editComment(comment)"
              >
                编辑
              </button>
              <button
                class="btn btn-sm btn-danger"
                @click="deleteComment(comment.id)"
              >
                删除
              </button>
            </div>
          </div>
          <div class="comment-content">{{ comment.content }}</div>
        </div>
      </div>

      <form @submit.prevent="handleComment" v-if="isAuthenticated" class="mt-3">
        <div class="form-group">
          <label class="form-label">发表评论</label>
          <textarea
            class="form-control"
            v-model="commentForm.content"
            required
          ></textarea>
        </div>
        <button type="submit" class="btn btn-primary">提交评论</button>
      </form>

      <div v-else class="text-center mt-3">
        <p>
          请<a href="#" @click.prevent="currentView = 'login'">登录</a
          >后发表评论
        </p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
const isAuthenticated = ref(false);

const props = defineProps({
  postId: {
    type: Number,
    required: true,
  },
});

const currentPost = ref(null);

// 模拟从API获取文章详情
const fetchPost = (id) => {
  currentPost.value = testPosts.find((post) => post.id === id);
};

// 工具函数
const formatDate = (dateString) => {
  const options = { year: "numeric", month: "long", day: "numeric" };
  return new Date(dateString).toLocaleDateString("zh-CN", options);
};

const isAuthor = (authorId) => {
  return isAuthenticated.value && user.value.id === authorId;
};

const isFavorite = (postId) => {
  const post = testPosts.find((p) => p.id === postId);
  return (
    post &&
    post.favorites &&
    post.favorites.some((f) => f.user_id === user.value.id)
  );
};

// 评论数据
const testComments = [
  {
    id: 1,
    post_id: 1,
    author: "tester",
    author_id: 2,
    tags: ["前端", "Vue", "教程"],
    content: "非常好的入门教程，对我帮助很大！",
    created_at: "2023-05-16T10:20:00Z",
  },
  {
    id: 2,
    post_id: 1,
    author: "writer",
    author_id: 3,
    tags: ["前端", "工具链", "Webpack"],
    content: "建议增加一些实际项目的例子，这样更容易理解。",
    created_at: "2023-05-17T14:30:00Z",
  },
  {
    id: 3,
    post_id: 2,
    author: "admin",
    author_id: 1,
    content:
      "工具链的选择确实很重要，我们团队最近从webpack迁移到了vite，构建速度提升明显。",
    created_at: "2023-06-03T09:15:00Z",
  },
  {
    id: 4,
    post_id: 4,
    author: "tester",
    author_id: 2,
    content: "响应式设计在移动优先的时代越来越重要了，感谢分享这些实用技巧！",
    created_at: "2023-06-19T11:40:00Z",
  },
];

const testPosts = [
  {
    id: 1,
    title: "Vue.js入门指南",
    excerpt: "本文介绍Vue.js的基本概念和使用方法，适合初学者快速上手。",
    content:
      "<p>Vue.js是一套用于构建用户界面的渐进式框架。与其它大型框架不同的是，Vue被设计为可以自底向上逐层应用。</p><p>Vue的核心库只关注视图层，不仅易于上手，还便于与第三方库或既有项目整合。</p>",
    author: "admin",
    author_id: 1,
    created_at: "2023-05-15T09:30:00Z",
    updated_at: "2023-05-15T09:30:00Z",
    published: true,
    favorites: [{ user_id: 2 }, { user_id: 3 }],
  },
  {
    id: 2,
    title: "现代前端开发工具链",
    excerpt: "探讨现代前端开发中常用的工具和技术栈。",
    content:
      "<p>现代前端开发已经离不开各种工具的支持。从包管理工具npm/yarn，到构建工具webpack/vite，再到各种框架和库，工具链的选择直接影响开发效率。</p><p>本文将介绍这些工具的基本用法和最佳实践。</p>",
    author: "tester",
    author_id: 2,
    created_at: "2023-06-02T14:15:00Z",
    updated_at: "2023-06-05T10:20:00Z",
    published: true,
    favorites: [{ user_id: 1 }],
  },
  {
    id: 3,
    title: "RESTful API设计原则",
    excerpt: "分享RESTful API设计的最佳实践和常见模式。",
    content:
      "<p>RESTful API已成为现代Web应用的标准接口设计风格。良好的API设计可以提高开发效率，降低维护成本。</p><p>本文将讨论资源命名、HTTP方法使用、状态码选择等关键设计原则。</p>",
    author: "writer",
    author_id: 3,
    created_at: "2023-06-10T11:00:00Z",
    updated_at: "2023-06-10T11:00:00Z",
    published: true,
    favorites: [],
  },
  {
    id: 4,
    title: "响应式设计技巧",
    excerpt: "如何创建适应各种设备的响应式网页设计。",
    content:
      "<p>在移动设备普及的今天，响应式设计已成为网页开发的基本要求。本文将介绍媒体查询、弹性布局、视口单位等关键技术。</p>",
    author: "admin",
    author_id: 1,
    created_at: "2023-06-18T16:45:00Z",
    updated_at: "2023-06-20T09:10:00Z",
    published: true,
    favorites: [{ user_id: 2 }, { user_id: 3 }],
  },
];

onMounted(() => {
  fetchPost(props.postId);
});
</script>

<style lang="less" scoped>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
}

:root {
  --primary: #4facfe;
  --secondary: #a777e3;
  --success: #43e97b;
  --danger: #ff5858;
  --warning: #f09819;
  --dark: #2c3e50;
  --light: #f8f9fa;
  --gray: #6c757d;
}

body {
  background-color: #f5f7fa;
  color: var(--dark);
  line-height: 1.6;
}
/* 按钮样式 */
.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s;
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.btn-primary {
  background-color: var(--primary);
  color: white;
}

.btn-secondary {
  background-color: var(--secondary);
  color: white;
}

.btn-success {
  background-color: var(--success);
  color: white;
}

.btn-danger {
  background-color: var(--danger);
  color: white;
}

.btn-outline {
  background-color: transparent;
  border: 1px solid white;
  color: white;
}

.btn:hover {
  opacity: 0.9;
  transform: translateY(-2px);
}

/* 表单样式 */
.form-group {
  margin-bottom: 15px;
}

.form-label {
  display: block;
  margin-bottom: 5px;
  font-weight: 500;
}

.form-control {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
}

textarea.form-control {
  min-height: 120px;
  resize: vertical;
}
/* 卡片样式 */
.card {
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
  padding: 20px;
  margin-bottom: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
  padding-bottom: 10px;
  border-bottom: 1px solid #eee;
}

.card-title {
  font-size: 1.4rem;
  font-weight: 600;
  color: var(--dark);
}
/* 评论样式 */
.comment-list {
  margin-top: 20px;
}

.comment-item {
  padding: 15px;
  border-radius: 8px;
  margin-bottom: 15px;
}

.comment-author {
  font-weight: 600;
  margin-bottom: 5px;
}

.comment-date {
  color: var(--gray);
  font-size: 0.8rem;
}

.comment-content {
  margin-top: 5px;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .navbar-container {
    flex-direction: column;
    gap: 15px;
  }

  .nav-links {
    flex-direction: column;
    align-items: center;
    gap: 10px;
  }

  .auth-buttons {
    margin-top: 10px;
  }
}

/* 动画效果 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* 工具类 */
.text-center {
  text-align: center;
}

.mt-3 {
  margin-top: 15px;
}

.mb-3 {
  margin-bottom: 15px;
}

/* 新增标签样式 */
.post-tags {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  margin-bottom: 15px;
}

.tag {
  background-color: #f0f0f0;
  color: #555;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.9rem;
}
</style>
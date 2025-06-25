<template>
  <main class="container" style="padding: 20px 0">
    <!-- 首页视图 -->
    <div>
      <div class="card">
        <h2 class="card-title text-center">最新文章</h2>

        <div class="post-list">
          <div class="card post-item" v-for="post in testPosts" :key="post.id">
            <h3 class="post-title">{{ post.title }}</h3>
            <div class="post-meta">
              <span>作者: {{ post.author }}</span>
              <span>发布于: {{ formatDate(post.created_at) }}</span>
            </div>
            <!-- 新增关键词标签区域 -->
            <div class="post-tags">
              <span class="tag" v-for="tag in post.tags" :key="tag">
                #{{ tag }}
              </span>
            </div>
            <p class="post-excerpt">{{ post.excerpt }}</p>
            <div class="post-actions">
              <button class="btn btn-primary" @click="viewPost(post.id)">
                阅读更多
              </button>
              <button
                class="btn btn-secondary"
                @click="toggleFavorite(post.id)"
                v-if="isAuthenticated"
              >
                <i
                  class="far fa-heart"
                  :class="{ fas: isFavorite(post.id) }"
                ></i>
              </button>
            </div>
          </div>
        </div>

        <div class="d-flex justify-between mt-3">
          <button class="btn" :disabled="currentPage === 1" @click="prevPage">
            上一页
          </button>
          <span>第 {{ currentPage }} 页</span>
          <button
            class="btn"
            :disabled="testPosts.length < perPage"
            @click="nextPage"
          >
            下一页
          </button>
        </div>
      </div>
    </div>
  </main>
</template>

<script setup>
import { ref, defineEmits } from "vue";
const currentPage = ref(1);
const perPage = ref(10);

const nextPage = () => {
  currentPage.value++;
};

const prevPage = () => {
  if (currentPage.value > 1) {
    currentPage.value--;
  }
};

// 工具函数
const formatDate = (dateString) => {
  const options = { year: "numeric", month: "long", day: "numeric" };
  return new Date(dateString).toLocaleDateString("zh-CN", options);
};

const emit = defineEmits(["view-post"]);

const viewPost = (postId) => {
  emit("view-post", postId);
};
// 文章数据
const testPosts = [
  {
    id: 1,
    title: "Vue.js入门指南",
    excerpt: "本文介绍Vue.js的基本概念和使用方法，适合初学者快速上手。",
    content:
      "<p>Vue.js是一套用于构建用户界面的渐进式框架。与其它大型框架不同的是，Vue被设计为可以自底向上逐层应用。</p><p>Vue的核心库只关注视图层，不仅易于上手，还便于与第三方库或既有项目整合。</p>",
    author: "admin",
    author_id: 1,
    tags: ["前端", "Vue", "教程"],
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
    tags: ["前端", "工具链", "Webpack", "Vite"],
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
    tags: ["后端", "API设计", "RESTful"],
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
    tags: ["前端", "响应式设计", "CSS"],
    created_at: "2023-06-18T16:45:00Z",
    updated_at: "2023-06-20T09:10:00Z",
    published: true,
    favorites: [{ user_id: 2 }, { user_id: 3 }],
  },
];
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
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
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
/* 文章列表 */
.post-list {
  display: grid;
  gap: 20px;
}

.post-item {
  padding: 15px;
  border-radius: 8px;
  transition: transform 0.3s;
}

.post-item:hover {
  transform: translateY(-3px);
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
}

.post-title {
  font-size: 1.3rem;
  font-weight: 600;
  margin-bottom: 10px;
  color: var(--dark);
}

.post-meta {
  display: flex;
  gap: 15px;
  color: var(--gray);
  font-size: 0.9rem;
  margin-bottom: 10px;
}

.post-excerpt {
  color: #555;
  margin-bottom: 10px;
}

.post-actions {
  display: flex;
  gap: 10px;
}

.mt-3 {
  margin-top: 15px;
}

.mb-3 {
  margin-bottom: 15px;
}

.d-flex {
  display: flex;
}

.justify-between {
  justify-content: space-between;
}
/* 新增标签样式 */
.post-tags {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  margin: 10px 0;
}

.tag {
  background-color: #f0f0f0;
  color: #555;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.9rem;
}
</style>
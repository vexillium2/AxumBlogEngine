<template>
  <main class="container" style="padding: 20px 0">
    <!-- 首页视图 -->
    <div>
      <div class="card">
        <h2 class="card-title text-center">最新文章</h2>

        <div class="post-list" v-if="!isLoading">
          <div class="card post-item" v-for="post in posts" :key="post.id">
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
                class="btn favorite-btn"
                :class="isAuthenticated ? 'btn-secondary' : 'btn-disabled'"
                @click="handleFavoriteClick(post.id)"
                :disabled="!isAuthenticated"
              >
                <i
                  class="far fa-heart"
                  :class="{ fas: isAuthenticated && isFavorite(post.id) }"
                ></i>
                {{ isAuthenticated ? (isFavorite(post.id) ? '已收藏' : '收藏') : '收藏' }}
              </button>
            </div>
          </div>
        </div>

        <div class="loading-container" v-if="isLoading">
          <p>加载中...</p>
        </div>
        
        <div class="d-flex justify-between mt-3" v-if="!isLoading">
          <button class="btn" :disabled="currentPage === 1" @click="prevPage">
            上一页
          </button>
          <span>第 {{ currentPage }} 页 (共 {{ totalPosts }} 篇文章)</span>
          <button
            class="btn"
            :disabled="posts.length < perPage"
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
import { ref, defineEmits, onMounted } from "vue";
import { postAPI, favoriteAPI, getToken } from "../api/index.js";

const currentPage = ref(1);
const perPage = ref(10);
const posts = ref([]);
const isLoading = ref(false);
const totalPosts = ref(0);
const isAuthenticated = ref(false);
const favoritePostIds = ref(new Set());

// 获取文章列表
const fetchPosts = async () => {
  isLoading.value = true;
  try {
    const response = await postAPI.list({
      page: currentPage.value,
      limit: perPage.value,
    });
    
    posts.value = response.posts || [];
    totalPosts.value = response.total_posts || posts.value.length;
    
    // 检查用户认证状态
    isAuthenticated.value = !!getToken();
    
    // 如果用户已登录，获取收藏列表
    if (isAuthenticated.value) {
      await fetchFavorites();
    }
  } catch (error) {
    console.error('获取文章列表失败:', error);
    // 如果API调用失败，使用测试数据
    posts.value = testPosts;
  } finally {
    isLoading.value = false;
  }
};

// 获取用户收藏列表
const fetchFavorites = async () => {
  try {
    const response = await favoriteAPI.list();
    const favorites = response.favorites || [];
    favoritePostIds.value = new Set(favorites.map(post => post.id));
  } catch (error) {
    console.error('获取收藏列表失败:', error);
  }
};

// 处理收藏按钮点击
const handleFavoriteClick = (postId) => {
  if (!isAuthenticated.value) {
    alert('请先登录后再收藏文章');
    return;
  }
  toggleFavorite(postId);
};

// 切换收藏状态
const toggleFavorite = async (postId) => {
  try {
    await favoriteAPI.toggle(postId);
    
    // 更新本地收藏状态
    if (favoritePostIds.value.has(postId)) {
      favoritePostIds.value.delete(postId);
    } else {
      favoritePostIds.value.add(postId);
    }
  } catch (error) {
    console.error('切换收藏状态失败:', error);
    alert('操作失败: ' + error.message);
  }
};

// 检查是否已收藏
const isFavorite = (postId) => {
  return favoritePostIds.value.has(postId);
};

const nextPage = async () => {
  currentPage.value++;
  await fetchPosts();
};

const prevPage = async () => {
  if (currentPage.value > 1) {
    currentPage.value--;
    await fetchPosts();
  }
};

// 组件挂载时获取数据
onMounted(() => {
  fetchPosts();
});

// 工具函数
const formatDate = (dateString) => {
  const options = { year: "numeric", month: "long", day: "numeric" };
  return new Date(dateString).toLocaleDateString("zh-CN", options);
};

const emit = defineEmits(["view-post"]);

const viewPost = (postId) => {
  emit("view-post", postId);
};

// 暴露方法给模板使用
defineExpose({
  toggleFavorite,
  isFavorite,
});
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
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
}

:root {
  --primary: #ff5722;
  --primary-light: #ff7043;
  --primary-dark: #d84315;
  --secondary: #2196f3;
  --secondary-light: #42a5f5;
  --success: #4caf50;
  --danger: #f44336;
  --warning: #ff9800;
  --dark: #212121;
  --text-primary: #333333;
  --text-secondary: #666666;
  --text-light: #999999;
  --border-color: #e0e0e0;
  --bg-color: #fafafa;
  --card-bg: #ffffff;
  --shadow-light: rgba(0, 0, 0, 0.04);
  --shadow-medium: rgba(0, 0, 0, 0.08);
  --shadow-heavy: rgba(0, 0, 0, 0.12);
}

body {
  background-color: var(--bg-color);
  color: var(--text-primary);
  line-height: 1.6;
  min-height: 100vh;
}
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.loading-container {
  text-align: center;
  padding: 60px 20px;
  color: var(--text-light);
  font-size: 1rem;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .container {
    padding: 16px;
  }
  
  .post-item {
    padding: 16px;
  }
  
  .post-title {
    font-size: 1.125rem;
  }
  
  .post-actions {
    flex-direction: column;
    gap: 8px;
  }
  
  .btn {
    width: 100%;
    justify-content: center;
  }
  
  .post-meta {
    flex-direction: column;
    gap: 6px;
  }
  
  .d-flex.justify-between {
    flex-direction: column;
    gap: 12px;
    text-align: center;
  }
}
/* 卡片样式 */
.card {
  background: var(--card-bg);
  border-radius: 8px;
  box-shadow: 0 2px 8px var(--shadow-light);
  padding: 24px;
  margin-bottom: 24px;
  border: 1px solid var(--border-color);
  transition: box-shadow 0.2s ease;
}

.card:hover {
  box-shadow: 0 4px 16px var(--shadow-medium);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--border-color);
}

.card-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}
/* 文章列表 */
.post-list {
  display: grid;
  gap: 16px;
}

.post-item {
  background: var(--card-bg);
  padding: 20px;
  border-radius: 8px;
  border: 1px solid var(--border-color);
  transition: all 0.2s ease;
  position: relative;
}

.post-item:hover {
  box-shadow: 0 4px 12px var(--shadow-medium);
  border-color: var(--primary);
}

.post-title {
  font-size: 1.25rem;
  font-weight: 600;
  margin-bottom: 12px;
  color: var(--text-primary);
  line-height: 1.4;
  transition: color 0.2s ease;
}

.post-item:hover .post-title {
  color: var(--primary);
}

.post-meta {
  display: flex;
  gap: 16px;
  color: var(--text-light);
  font-size: 0.875rem;
  margin-bottom: 12px;
}

.post-meta span {
  display: flex;
  align-items: center;
  gap: 4px;
}

.post-excerpt {
  color: var(--text-secondary);
  margin-bottom: 16px;
  line-height: 1.6;
  font-size: 0.9rem;
}

.post-actions {
  display: flex;
  gap: 12px;
  align-items: center;
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
  align-items: center;
}

/* 分页样式 */
.d-flex.justify-between {
  background: var(--card-bg);
  padding: 16px;
  border-radius: 8px;
  border: 1px solid var(--border-color);
  box-shadow: 0 2px 4px var(--shadow-light);
}

.d-flex.justify-between span {
  font-weight: 500;
  color: var(--text-primary);
  font-size: 0.875rem;
}

.d-flex.justify-between .btn {
  min-width: 80px;
}

.d-flex.justify-between .btn:disabled {
  background: #f5f5f5;
  color: var(--text-light);
  border-color: var(--border-color);
  cursor: not-allowed;
  opacity: 0.6;
}

.d-flex.justify-between .btn:disabled:hover {
  box-shadow: none;
}

/* 按钮基础样式 */
.btn {
  padding: 8px 16px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  display: inline-flex;
  align-items: center;
  gap: 6px;
  text-decoration: none;
  background: var(--card-bg);
  color: var(--text-primary);
}

.btn-primary {
  background: var(--primary);
  color: white;
  border-color: var(--primary);
}

.btn-primary:hover {
  background: var(--primary-dark);
  border-color: var(--primary-dark);
}

.btn-secondary {
  background: var(--secondary);
  color: white;
  border-color: var(--secondary);
}

.btn-secondary:hover {
  background: var(--secondary-light);
  border-color: var(--secondary-light);
}

.btn:hover {
  box-shadow: 0 2px 4px var(--shadow-medium);
}
/* 标签样式 */
.post-tags {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  margin: 12px 0;
}

.tag {
  background: #f5f5f5;
  color: var(--text-secondary);
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.75rem;
  font-weight: 500;
  border: 1px solid var(--border-color);
  transition: all 0.2s ease;
}

.tag:hover {
  background: var(--primary);
  color: white;
  border-color: var(--primary);
}

/* 收藏按钮样式 */
.favorite-btn {
  background: #e91e63 !important;
  color: white !important;
  border-color: #e91e63 !important;
}

.favorite-btn:hover {
  background: #d81b60 !important;
  border-color: #d81b60 !important;
}

.favorite-btn i {
  font-size: 0.875rem;
}

/* 禁用状态的收藏按钮 */
.btn-disabled {
  background: #f5f5f5 !important;
  color: var(--text-light) !important;
  border-color: var(--border-color) !important;
  cursor: not-allowed !important;
  opacity: 0.6;
}

.btn-disabled:hover {
  background: #f5f5f5 !important;
  color: var(--text-light) !important;
  border-color: var(--border-color) !important;
  box-shadow: none !important;
  opacity: 0.6;
}
</style>
<template>
  <div>
    <button class="btn btn-secondary mb-3" @click="$emit('back-to-home')">
      返回首页
    </button>

    <div class="card">
      <div class="d-flex justify-between align-items-center">
        <h2 class="card-title">我的收藏</h2>
      </div>

      <div class="post-list">
        <div
          class="card post-item"
          v-for="post in favoritePosts"
          :key="post.id"
        >
          <div class="d-flex justify-between align-items-center">
            <h3 class="post-title">{{ post.title }}</h3>
          </div>
          <div class="post-meta">
            <span>作者: {{ post.author }}</span>
            <span>发布于: {{ formatDate(post.created_at) }}</span>
          </div>
          <p class="post-excerpt">{{ post.excerpt }}</p>
          <div class="post-actions d-flex justify-end">
            <button class="btn btn-primary" @click="viewPost(post.id)">
              阅读更多
            </button>
            <button class="btn btn-danger" @click="toggleFavorite(post.id)">
              取消收藏
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { favoriteAPI, getToken } from "../api/index.js";

const props = defineProps({
  userId: {
    type: Number,
    required: false,
  },
});

const emit = defineEmits(["back-to-home", "view-post"]);

// 模拟收藏文章数据
const favoritePosts = ref([]);

// 获取用户收藏
const fetchFavorites = async () => {
  try {
    // 检查用户认证状态
    if (!getToken()) {
      console.warn('用户未登录，无法获取收藏列表');
      favoritePosts.value = [];
      return;
    }
    
    const response = await favoriteAPI.list();
    favoritePosts.value = response.favorites || [];
  } catch (error) {
    console.error('获取收藏列表失败:', error);
    alert('获取收藏列表失败: ' + error.message);
    favoritePosts.value = [];
  }
};

// 查看文章
const viewPost = (postId) => {
  // 通知父组件切换到详情页
  emit('view-post', postId);
};

// 取消收藏
const toggleFavorite = async (postId) => {
  if (confirm("确定要取消收藏吗？")) {
    try {
      await favoriteAPI.toggle(postId);
      // 重新获取收藏列表
      await fetchFavorites();
    } catch (error) {
      console.error('取消收藏失败:', error);
      alert('取消收藏失败: ' + error.message);
    }
  }
};

// 格式化日期
const formatDate = (dateString) => {
  const options = { year: "numeric", month: "long", day: "numeric" };
  return new Date(dateString).toLocaleDateString("zh-CN", options);
};

onMounted(() => {
  fetchFavorites();
});
</script>

<style scoped>
/* 使用和Post.vue相同的样式 */
.post-list {
  display: grid;
  gap: 15px;
  margin-top: 20px;
}

.post-item {
  padding: 15px;
  transition: all 0.3s;
}

.post-item:hover {
  transform: translateY(-3px);
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
}

.post-title {
  font-size: 1.2rem;
  margin-bottom: 10px;
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
  margin-top: 10px;
}

.justify-end {
  justify-content: flex-end;
}

.d-flex {
  display: flex;
}

.justify-between {
  justify-content: space-between;
}

.align-items-center {
  align-items: center;
}

.mb-3 {
  margin-bottom: 15px;
}
</style>
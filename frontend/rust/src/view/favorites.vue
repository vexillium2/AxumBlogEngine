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

const props = defineProps({
  userId: {
    type: Number,
    required: true,
  },
});

const emit = defineEmits(["back-to-home"]);

// 模拟收藏文章数据
const favoritePosts = ref([]);

// 获取用户收藏
const fetchFavorites = () => {
  // 这里应该是API调用，我们暂时使用测试数据
  const allPosts = [
    {
      id: 1,
      title: "Vue.js入门指南",
      author: "admin",
      excerpt: "本文介绍Vue.js的基本概念和使用方法...",
      created_at: "2023-05-15T09:30:00Z",
      favorites: [{ user_id: 2 }, { user_id: 3 }],
    },
    {
      id: 2,
      title: "现代前端开发工具链",
      author: "tester",
      excerpt: "探讨现代前端开发中常用的工具和技术栈...",
      created_at: "2023-06-02T14:15:00Z",
      favorites: [{ user_id: 1 }],
    },
    {
      id: 4,
      title: "响应式设计技巧",
      author: "admin",
      excerpt: "如何创建适应各种设备的响应式网页设计...",
      created_at: "2023-06-18T16:45:00Z",
      favorites: [{ user_id: 2 }, { user_id: 3 }],
    },
  ];

  favoritePosts.value = allPosts.filter((post) =>
    post.favorites.some((fav) => fav.user_id === props.userId)
  );
};

// 查看文章
const viewPost = (postId) => {
  // 这里应该导航到文章详情页
  console.log("查看文章:", postId);
};

// 取消收藏
const toggleFavorite = (postId) => {
  if (confirm("确定要取消收藏吗？")) {
    favoritePosts.value = favoritePosts.value.filter(
      (post) => post.id !== postId
    );
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
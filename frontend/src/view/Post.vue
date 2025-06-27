<template>
  <div>
    <button class="btn btn-secondary mb-3" @click="$emit('back-to-home')">
      返回首页
    </button>

    <div class="card">
      <div class="card-header">
        <h2 class="card-title">我的文章</h2>
        <button class="btn btn-primary" @click="createPost">新建文章</button>
      </div>

      <div class="post-list">
        <div class="card post-item" v-for="post in userPosts" :key="post.id">
          <div class="post-content">
            <h3 class="post-title">{{ post.title }}</h3>
            <div class="post-meta">
              <span>发布于: {{ formatDate(post.created_at) }}</span>
              <span>状态: {{ post.is_published ? "已发布" : "草稿" }}</span>
            </div>
          </div>
          <div class="post-actions">
            <button class="btn btn-primary" @click="viewPost(post.id)">
              查看
            </button>
            <button class="btn btn-secondary" @click="editPost(post.id)">
              编辑
            </button>
            <button class="btn btn-danger" @click="deletePost(post.id)">
              删除
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
  <Edit
    v-if="showEditor"
    :mode="editorMode"
    :postData="currentPost"
    @save="handleSave"
    @cancel="showEditor = false"
  />
</template>

<script setup>
import { ref, onMounted } from "vue";
import Edit from "./Edit.vue";
import { postAPI, userAPI, getToken } from '../api/index.js';

const props = defineProps({
  userId: {
    type: Number,
    required: false,
    default: null,
  },
});

const emit = defineEmits(["back-to-home", "view-post"]);

// 模拟用户文章数据
const userPosts = ref([]);

// 获取用户文章
const fetchUserPosts = async () => {
  try {
    // 检查用户认证状态
    if (!getToken()) {
      console.warn('用户未登录，无法获取文章列表');
      userPosts.value = [];
      return;
    }
    
    // 获取当前用户信息
    const userResponse = await userAPI.getMe();
    const currentUser = userResponse.user;
    
    if (!currentUser) {
      console.warn('无法获取当前用户信息');
      userPosts.value = [];
      return;
    }
    
    // 获取已发布的文章
    const response = await postAPI.list({ 
      page: 1, 
      limit: 100, // 获取更多文章
      published_only: true // 只显示已发布的文章
    });
    
    // 过滤出当前用户的文章
    userPosts.value = (response.posts || []).filter(post => post.author_id === currentUser.id);
  } catch (error) {
    console.error('获取文章列表失败:', error);
    alert('获取文章列表失败: ' + error.message);
    userPosts.value = [];
  }
};

const showEditor = ref(false);
const editorMode = ref("create");
const currentPost = ref(null);

// 查看文章
const viewPost = (postId) => {
  // 通知父组件切换到详情页
  emit('view-post', postId);
};

// 编辑文章
const editPost = async (postId) => {
  try {
    const response = await postAPI.get(postId);
    currentPost.value = response;
    editorMode.value = "edit";
    showEditor.value = true;
  } catch (error) {
    console.error('获取文章详情失败:', error);
    alert('获取文章详情失败: ' + error.message);
  }
};

// 删除文章
const deletePost = async (postId) => {
  if (confirm("确定要删除这篇文章吗？")) {
    try {
      await postAPI.delete(postId);
      alert('文章删除成功');
      await fetchUserPosts(); // 重新获取文章列表
    } catch (error) {
      console.error('删除文章失败:', error);
      alert('删除文章失败: ' + error.message);
    }
  }
};

// 创建新文章
const createPost = () => {
  editorMode.value = "create";
  currentPost.value = null;
  showEditor.value = true;
};

// 格式化日期
const formatDate = (dateString) => {
  const options = { year: "numeric", month: "long", day: "numeric" };
  return new Date(dateString).toLocaleDateString("zh-CN", options);
};

const handleSave = async (postData) => {
  if (editorMode.value === "create") {
    // 添加新文章到列表顶部
    userPosts.value.unshift(postData);
  } else {
    // 更新现有文章
    const index = userPosts.value.findIndex((p) => p.id === postData.id);
    if (index !== -1) {
      userPosts.value[index] = postData;
    }
  }
  showEditor.value = false;
  // 重新获取文章列表以确保数据同步
  await fetchUserPosts();
};

onMounted(() => {
  fetchUserPosts();
});
</script>

<style scoped>
.post-list {
  display: grid;
  gap: 15px;
  margin-top: 20px;
}

.post-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px;
  transition: all 0.3s;
}

.post-item:hover {
  transform: translateY(-3px);
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
}

.post-content {
  flex: 1;
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

.post-actions {
  display: flex;
  gap: 10px;
  margin-left: 20px;
}

.mb-3 {
  margin-bottom: 15px;
}
</style>
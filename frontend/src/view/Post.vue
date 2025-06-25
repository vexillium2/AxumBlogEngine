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
              <span>状态: {{ post.published ? "已发布" : "草稿" }}</span>
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

const props = defineProps({
  userId: {
    type: Number,
    required: true,
  },
});

const emit = defineEmits(["back-to-home"]);

// 模拟用户文章数据
const userPosts = ref([]);

// 获取用户文章
const fetchUserPosts = () => {
  // 这里应该是API调用，我们暂时使用测试数据
  const allPosts = [
    {
      id: 1,
      title: "Vue.js入门指南",
      author_id: 1,
      created_at: "2023-05-15T09:30:00Z",
      updated_at: "2023-05-15T09:30:00Z",
      published: true,
    },
    {
      id: 2,
      title: "现代前端开发工具链",
      author_id: 2,
      created_at: "2023-06-02T14:15:00Z",
      updated_at: "2023-06-05T10:20:00Z",
      published: true,
    },
    {
      id: 3,
      title: "RESTful API设计原则",
      author_id: 3,
      created_at: "2023-06-10T11:00:00Z",
      updated_at: "2023-06-10T11:00:00Z",
      published: false,
    },
  ];

  userPosts.value = allPosts.filter((post) => post.author_id === props.userId);
};

const showEditor = ref(false);
const editorMode = ref("create");
const currentPost = ref(null);

// 查看文章
const viewPost = (postId) => {
  // 这里应该导航到文章详情页
  console.log("查看文章:", postId);
};

// 编辑文章
const editPost = (postId) => {
  const post = userPosts.value.find((p) => p.id === postId);
  if (post) {
    editorMode.value = "edit";
    currentPost.value = post;
    showEditor.value = true;
  }
};

// 删除文章
const deletePost = (postId) => {
  if (confirm("确定要删除这篇文章吗？")) {
    userPosts.value = userPosts.value.filter((post) => post.id !== postId);
  }
};

// 新建文章
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

const handleSave = (postData) => {
  if (editorMode.value === "create") {
    // 添加新文章
    userPosts.value.unshift(postData);
  } else {
    // 更新现有文章
    const index = userPosts.value.findIndex((p) => p.id === postData.id);
    if (index !== -1) {
      userPosts.value[index] = postData;
    }
  }
  showEditor.value = false;
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
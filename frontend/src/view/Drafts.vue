<template>
  <div>
    <button class="btn btn-secondary mb-3" @click="$emit('back-to-home')">
      返回首页
    </button>

    <div class="card">
      <div class="card-header">
        <h2 class="card-title">我的草稿</h2>
        <button class="btn btn-primary" @click="createDraft">新建草稿</button>
      </div>

      <div class="draft-list" v-if="!isLoading">
        <div v-if="draftPosts.length === 0" class="empty-state">
          <div class="empty-icon">
            <i class="fas fa-file-alt"></i>
          </div>
          <h3>暂无草稿</h3>
          <p>您还没有保存任何草稿，开始创建您的第一篇草稿吧！</p>
          <button class="btn btn-primary" @click="createDraft">
            <i class="fas fa-plus"></i> 创建草稿
          </button>
        </div>

        <div class="card draft-item" v-for="draft in draftPosts" :key="draft.id" v-else>
          <div class="draft-content">
            <h3 class="draft-title">{{ draft.title || '无标题草稿' }}</h3>
            <div class="draft-meta">
              <span><i class="fas fa-calendar"></i> 创建于: {{ formatDate(draft.created_at) }}</span>
              <span><i class="fas fa-edit"></i> 更新于: {{ formatDate(draft.updated_at) }}</span>
              <span class="draft-status"><i class="fas fa-file-alt"></i> 草稿</span>
            </div>
            <div class="draft-preview" v-if="draft.content_markdown">
              <p>{{ getPreviewText(draft.content_markdown) }}</p>
            </div>
            <div class="draft-category" v-if="draft.category">
              <span class="category-tag">
                <i class="fas fa-tag"></i> {{ draft.category }}
              </span>
            </div>
          </div>
          <div class="draft-actions">
            <button class="btn btn-primary" @click="editDraft(draft.id)">
              <i class="fas fa-edit"></i> 继续编辑
            </button>
            <button class="btn btn-success" @click="publishDraft(draft.id)">
              <i class="fas fa-paper-plane"></i> 发布
            </button>
            <button class="btn btn-secondary" @click="viewDraft(draft.id)">
              <i class="fas fa-eye"></i> 预览
            </button>
            <button class="btn btn-danger" @click="deleteDraft(draft.id)">
              <i class="fas fa-trash"></i> 删除
            </button>
          </div>
        </div>
      </div>

      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner"></div>
        <p>正在加载草稿...</p>
      </div>
    </div>
  </div>
  
  <Edit
    v-if="showEditor"
    :mode="editorMode"
    :postData="currentDraft"
    @save="handleSave"
    @cancel="showEditor = false"
    @back-to-drafts="showEditor = false"
  />
</template>

<script setup>
import { ref, onMounted } from "vue";
import Edit from "./Edit.vue";
import { postAPI, userAPI, getToken } from '../api/index.js';

const props = defineProps({
  userId: {
    type: Number,
    default: null,
  },
});

const emit = defineEmits(["back-to-home", "view-post"]);

// 草稿数据
const draftPosts = ref([]);
const isLoading = ref(true);

// 编辑器状态
const showEditor = ref(false);
const editorMode = ref("create");
const currentDraft = ref(null);

// 获取用户草稿
const fetchUserDrafts = async () => {
  isLoading.value = true;
  try {
    const token = getToken();
    if (!token) {
      console.warn('用户未登录，无法获取草稿列表');
      draftPosts.value = [];
      return;
    }

    // 获取当前用户信息
    const currentUser = await userAPI.getMe();
    if (!currentUser) {
      console.warn('无法获取用户信息');
      draftPosts.value = [];
      return;
    }

    // 获取当前用户的所有文章（包括草稿）
    const response = await postAPI.list({
      page: 1,
      limit: 100, // 获取更多文章
      published_only: false, // 包括草稿
      author_id: currentUser.id // 只获取当前用户的文章
    });

    // 过滤出草稿（未发布的文章）
    draftPosts.value = (response.posts || []).filter(post => 
      !post.is_published
    );
    
    console.log('获取到的草稿数量:', draftPosts.value.length);
    console.log('草稿列表:', draftPosts.value);
  } catch (error) {
    console.error('获取草稿列表失败:', error);
    alert('获取草稿列表失败: ' + error.message);
    draftPosts.value = [];
  } finally {
    isLoading.value = false;
  }
};

// 格式化日期
const formatDate = (dateString) => {
  const date = new Date(dateString);
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  });
};

// 获取预览文本
const getPreviewText = (content) => {
  if (!content) return '';
  // 移除 Markdown 标记并截取前100个字符
  const plainText = content.replace(/[#*`_\[\]()]/g, '').replace(/\n/g, ' ');
  return plainText.length > 100 ? plainText.substring(0, 100) + '...' : plainText;
};

// 创建新草稿
const createDraft = () => {
  editorMode.value = "create";
  currentDraft.value = null;
  showEditor.value = true;
};

// 编辑草稿
const editDraft = async (draftId) => {
  try {
    const response = await postAPI.get(draftId);
    currentDraft.value = response;
    editorMode.value = "edit";
    showEditor.value = true;
  } catch (error) {
    console.error('获取草稿详情失败:', error);
    alert('获取草稿详情失败: ' + error.message);
  }
};

// 发布草稿
const publishDraft = async (draftId) => {
  if (confirm("确定要发布这篇草稿吗？")) {
    try {
      const draft = draftPosts.value.find(d => d.id === draftId);
      if (!draft) {
        alert('草稿不存在');
        return;
      }

      await postAPI.update(draftId, {
        title: draft.title,
        content_markdown: draft.content_markdown,
        category: draft.category,
        is_published: true,
        cover_url: draft.cover_url
      });
      
      alert('草稿发布成功！');
      await fetchUserDrafts(); // 重新获取草稿列表
    } catch (error) {
      console.error('发布草稿失败:', error);
      alert('发布草稿失败: ' + error.message);
    }
  }
};

// 预览草稿
const viewDraft = (draftId) => {
  emit('view-post', draftId);
};

// 删除草稿
const deleteDraft = async (draftId) => {
  if (confirm("确定要删除这篇草稿吗？此操作不可恢复。")) {
    try {
      await postAPI.delete(draftId);
      alert('草稿删除成功');
      await fetchUserDrafts(); // 重新获取草稿列表
    } catch (error) {
      console.error('删除草稿失败:', error);
      alert('删除草稿失败: ' + error.message);
    }
  }
};

// 处理保存
const handleSave = async (postData) => {
  showEditor.value = false;
  // 重新获取草稿列表以确保数据同步
  await fetchUserDrafts();
};

// 组件挂载时获取草稿
onMounted(() => {
  fetchUserDrafts();
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
  --light: #ecf0f1;
  --border: #ddd;
}

.card {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  margin-bottom: 20px;
  overflow: hidden;
}

.card-header {
  background: linear-gradient(135deg, var(--primary), var(--secondary));
  color: white;
  padding: 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.card-title {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
}

.mb-3 {
  margin-bottom: 1rem;
}

/* 草稿列表 */
.draft-list {
  padding: 20px;
}

.draft-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 20px;
  margin-bottom: 15px;
  border: 1px solid var(--border);
  transition: all 0.3s ease;
}

.draft-item:hover {
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  transform: translateY(-2px);
}

.draft-content {
  flex: 1;
  margin-right: 20px;
}

.draft-title {
  color: var(--dark);
  margin-bottom: 10px;
  font-size: 1.2rem;
  font-weight: 600;
}

.draft-meta {
  display: flex;
  gap: 15px;
  margin-bottom: 10px;
  font-size: 0.9rem;
  color: #666;
}

.draft-meta span {
  display: flex;
  align-items: center;
  gap: 5px;
}

.draft-status {
  color: var(--warning);
  font-weight: 500;
}

.draft-preview {
  margin: 10px 0;
  color: #666;
  font-size: 0.95rem;
  line-height: 1.5;
}

.draft-category {
  margin-top: 10px;
}

.category-tag {
  background-color: #f0f0f0;
  color: #555;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.85rem;
  display: inline-flex;
  align-items: center;
  gap: 4px;
}

.draft-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  align-items: flex-start;
}

/* 空状态 */
.empty-state {
  text-align: center;
  padding: 60px 20px;
  color: #666;
}

.empty-icon {
  font-size: 4rem;
  color: #ddd;
  margin-bottom: 20px;
}

.empty-state h3 {
  margin-bottom: 10px;
  color: var(--dark);
}

.empty-state p {
  margin-bottom: 20px;
  font-size: 1rem;
}

/* 加载状态 */
.loading-state {
  text-align: center;
  padding: 60px 20px;
  color: #666;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f3f3f3;
  border-top: 4px solid var(--primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 20px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* 按钮样式 */
.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s;
  display: inline-flex;
  align-items: center;
  gap: 6px;
  text-decoration: none;
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

.btn:hover {
  opacity: 0.9;
  transform: translateY(-2px);
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .draft-item {
    flex-direction: column;
    align-items: stretch;
  }
  
  .draft-content {
    margin-right: 0;
    margin-bottom: 15px;
  }
  
  .draft-actions {
    justify-content: flex-start;
  }
  
  .draft-meta {
    flex-direction: column;
    gap: 8px;
  }
}
</style>
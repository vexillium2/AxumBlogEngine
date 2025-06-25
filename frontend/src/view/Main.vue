<template>
  <nav class="navbar">
    <div class="container navbar-container">
      <a href="#" class="logo" @click.prevent="currentView = 'home'"
        >轻量博客</a
      >

      <div class="nav-links">
        <a href="#" class="nav-link" @click.prevent="currentView = 'home'"
          >首页</a
        >
        <a
          href="#"
          class="nav-link"
          @click.prevent="currentView = 'posts'"
          v-if="isAuthenticated"
          >我的文章</a
        >
        <a
          href="#"
          class="nav-link"
          @click.prevent="currentView = 'drafts'"
          v-if="isAuthenticated"
          >草稿箱</a
        >
        <a
          href="#"
          class="nav-link"
          @click.prevent="currentView = 'favorites'"
          v-if="isAuthenticated"
          >收藏</a
        >
        <a
          href="#"
          class="nav-link"
          @click.prevent="currentView = 'create-post'"
          v-if="isAuthenticated"
          >写文章</a
        >
      </div>

      <div class="auth-buttons" v-if="!isAuthenticated">
        <button class="btn btn-outline" @click="isLogin = true">登录</button>
        <button class="btn btn-primary" @click="isRegister = true">注册</button>
      </div>

      <div class="auth-buttons" v-else>
        <span class="user-welcome">欢迎, {{ user.username }}</span>
        <button class="btn btn-danger" @click="logout">登出</button>
      </div>
    </div>
  </nav>
  <!-- 新增的搜索框部分 -->
  <div class="search-container">
    <div class="container">
      <div class="search-box">
        <input type="text" placeholder="搜索文章..." class="search-input" />
        <button class="search-button">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <circle cx="11" cy="11" r="8"></circle>
            <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
          </svg>
        </button>
      </div>
    </div>
  </div>
  <Home v-if="currentView === 'home'" @view-post="handleViewPost" />
  <Detail
    v-if="currentView === 'detail'"
    :postId="currentPostId"
    @back-to-home="currentView = 'home'"
  />
  <Post
    v-if="currentView === 'posts'"
    :userId="user?.id"
    @back-to-home="currentView = 'home'"
    @view-post="handleViewPost"
  />
  <Drafts
    v-if="currentView === 'drafts'"
    :userId="user?.id"
    @back-to-home="currentView = 'home'"
    @view-post="handleViewPost"
  />
  <Favorites
    v-if="currentView === 'favorites'"
    :userId="user?.id"
    @back-to-home="currentView = 'home'"
    @view-post="handleViewPost"
  />
  <Edit
    v-if="currentView === 'create-post'"
    mode="create"
    @save="handlePostSave"
    @cancel="currentView = 'home'"
  />

  <div>
    <el-dialog
      v-model="isLogin"
      class="card"
      style="max-width: 500px; margin: 200px auto"
      @closed="handleDialogClosed"
    >
      <Login
        @switch-to-register="switchToRegister"
        @login-success="handleLoginSuccess"
      />
    </el-dialog>
  </div>

  <div>
    <el-dialog
      v-model="isRegister"
      class="card"
      style="max-width: 500px; margin: 100px auto"
      @closed="handleDialogClosed"
    >
      <Register @switch-to-login="switchToLogin" />
    </el-dialog>
  </div>
</template>

<script setup>
import Home from "./Home.vue";
import Login from "./login.vue";
import Register from "./register.vue";
import Detail from "./Detail.vue";
import Post from "./Post.vue";
import Drafts from "./Drafts.vue";
import Favorites from "./favorites.vue";
import Edit from "./Edit.vue";
import { ref, computed, watch, onMounted } from "vue";
import { userAPI, getToken, clearToken } from "../api/index.js";

const user = ref(null); // 存储登录用户信息
const isAuthenticated = ref(false);
const currentView = ref("home");
const currentPostId = ref(null); // 存储当前查看的文章ID
const isRegister = ref(false);
const isLogin = ref(false);

// 检查用户认证状态
const checkAuthStatus = async () => {
  const token = getToken();
  if (token) {
    try {
      const userData = await userAPI.getMe();
      user.value = userData;
      isAuthenticated.value = true;
    } catch (error) {
      console.error('获取用户信息失败:', error);
      clearToken();
      isAuthenticated.value = false;
      user.value = null;
    }
  }
};

// 组件挂载时检查认证状态
onMounted(() => {
  checkAuthStatus();
});

// 切换到注册弹窗
const switchToRegister = () => {
  isLogin.value = false;
  isRegister.value = true;
};

// 切换到登录弹窗
const switchToLogin = () => {
  isRegister.value = false;
  isLogin.value = true;
};

const handleDialogClosed = () => {
  isLogin.value = false;
  isRegister.value = false;
  currentView.value = "home";
};

const handleViewPost = (postId) => {
  currentPostId.value = postId;
  currentView.value = "detail";
};

// 处理登录成功
const handleLoginSuccess = (userData) => {
  isAuthenticated.value = true;
  user.value = userData;
  isLogin.value = false; // 关闭登录弹窗
};

// 登出功能
const logout = () => {
  clearToken();
  isAuthenticated.value = false;
  user.value = null;
  currentView.value = 'home';
};

// 处理文章保存
const handlePostSave = (articleData) => {
  console.log('文章保存成功:', articleData);
  // 保存成功后跳转到我的文章页面
  currentView.value = 'posts';
};
</script>

<style lang="less">
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
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
}

.card {
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
  padding: 20px;
  margin-bottom: 20px;
}

/* 导航栏样式 */
.navbar {
  background: var(--card-bg);
  color: var(--text-primary);
  padding: 16px 0;
  box-shadow: 0 2px 4px var(--shadow-light);
  position: sticky;
  top: 0;
  z-index: 1000;
  border-bottom: 1px solid var(--border-color);
}

.navbar-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.logo {
  font-size: 1.5rem;
  font-weight: 700;
  text-decoration: none;
  color: var(--primary);
  transition: color 0.2s ease;
}

.logo:hover {
  color: var(--primary-dark);
}

.nav-links {
  display: flex;
  gap: 24px;
}

.nav-link {
  color: var(--text-primary);
  text-decoration: none;
  font-weight: 500;
  font-size: 0.875rem;
  padding: 8px 12px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.nav-link:hover {
  color: var(--primary);
  background: rgba(255, 87, 34, 0.08);
}

.auth-buttons {
  display: flex;
  align-items: center;
  gap: 12px;
}

.user-welcome {
  color: var(--text-secondary);
  font-size: 0.875rem;
  font-weight: 500;
  white-space: nowrap;
  padding: 6px 12px;
  background: #f5f5f5;
  border-radius: 4px;
  border: 1px solid var(--border-color);
}

/* 按钮样式 */
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

.btn-success {
  background: var(--success);
  color: white;
  border-color: var(--success);
}

.btn-danger {
  background: var(--danger);
  color: white;
  border-color: var(--danger);
}

.btn-outline {
  background: transparent;
  border: 1px solid var(--primary);
  color: var(--primary);
}

.btn-outline:hover {
  background: var(--primary);
  color: white;
}

.btn:hover {
  box-shadow: 0 2px 4px var(--shadow-medium);
}
/* 搜索框样式 */
.search-container {
  background: var(--card-bg);
  padding: 16px 0;
  box-shadow: 0 1px 3px var(--shadow-light);
  margin-bottom: 16px;
  border-bottom: 1px solid var(--border-color);
}

.search-box {
  position: relative;
  max-width: 600px;
  margin: 0 auto;
  display: flex;
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid var(--border-color);
  box-shadow: 0 2px 4px var(--shadow-light);
}

.search-input {
  flex: 1;
  padding: 12px 16px;
  border: none;
  background: var(--card-bg);
  font-size: 0.875rem;
  outline: none;
  transition: all 0.2s ease;
}

.search-input:focus {
  box-shadow: inset 0 0 0 2px var(--primary);
}

.search-button {
  background: var(--primary);
  color: white;
  border: none;
  padding: 0 16px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.search-button:hover {
  background: var(--primary-dark);
}

.search-button svg {
  width: 16px;
  height: 16px;
}
</style>

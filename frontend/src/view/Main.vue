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
          @click.prevent="currentView = 'favorites'"
          v-if="isAuthenticated"
          >收藏</a
        >
      </div>

      <div class="auth-buttons" v-if="!isAuthenticated">
        <button class="btn btn-outline" @click="isLogin = true">登录</button>
        <button class="btn btn-primary" @click="isRegister = true">注册</button>
      </div>

      <div class="auth-buttons" v-else>
        <span class="nav-link">欢迎, {{ user.username }}</span>
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
  />
  <Favorites
    v-if="currentView === 'favorites'"
    :userId="user?.id"
    @back-to-home="currentView = 'home'"
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
import Favorites from "./favorites.vue";
import { ref, computed, watch } from "vue";

const user = ref(null); // 存储登录用户信息
const isAuthenticated = ref(false);
const currentView = ref("home");
const currentPostId = ref(null); // 存储当前查看的文章ID
const isRegister = ref(false);
const isLogin = ref(false);

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
  isAuthenticated.value = false;
  user.value = null;
};
</script>

<style lang="less">
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

.card {
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
  padding: 20px;
  margin-bottom: 20px;
}

/* 导航栏样式 */
.navbar {
  background: linear-gradient(to right, var(--primary), var(--secondary));
  color: white;
  padding: 15px 0;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  position: sticky;
  top: 0;
  z-index: 1000;
}

.navbar-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.logo {
  font-size: 1.8rem;
  font-weight: 700;
  text-decoration: none;
  color: white;
}

.nav-links {
  display: flex;
  gap: 20px;
}

.nav-link {
  color: white;
  text-decoration: none;
  font-weight: 500;
  transition: opacity 0.3s;
}

.nav-link:hover {
  opacity: 0.8;
}

.auth-buttons {
  display: flex;
  gap: 15px;
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
/* 新增搜索框样式 */
.search-container {
  background-color: white;
  padding: 20px 0;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.05);
  margin-bottom: 20px;
}

.search-box {
  position: relative;
  max-width: 600px;
  margin: 0 auto;
  display: flex;
  border-radius: 30px;
  overflow: hidden;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.search-input {
  flex: 1;
  padding: 12px 20px;
  border: none;
  background-color: #f5f7fa;
  font-size: 1rem;
  outline: none;
  transition: all 0.3s;
}

.search-input:focus {
  background-color: white;
  box-shadow: inset 0 0 0 2px var(--primary);
}

.search-button {
  background: linear-gradient(to right, var(--primary), var(--secondary));
  color: white;
  border: none;
  padding: 0 20px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s;
}

.search-button:hover {
  opacity: 0.9;
  transform: scale(1.05);
}

.search-button svg {
  transition: transform 0.3s;
}

.search-button:hover svg {
  transform: scale(1.1);
}
</style>

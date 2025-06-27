<template>
  <div>
    <h2 class="card-title text-center">用户登录</h2>

    <form @submit.prevent="handleLogin">
      <div class="form-group">
        <label class="form-label">用户名</label>
        <input
          type="text"
          class="form-control"
          v-model="loginForm.username"
          required
        />
      </div>

      <div class="form-group">
        <label class="form-label">密码</label>
        <input
          type="password"
          class="form-control"
          v-model="loginForm.password"
          required
        />
      </div>

      <button
        type="submit"
        class="btn btn-primary btn-block"
        @click="handleLogin"
      >
        登录
      </button>
    </form>

    <p class="text-center mt-3">
      还没有账号?
      <a href="#" @click="emitSwitchToRegister">立即注册</a>
    </p>
  </div>
</template>

<script setup>
import { reactive, ref } from "vue";
import { defineProps, defineEmits } from "vue";

// 表单数据
const loginForm = reactive({
  username: "",
  password: "",
});

const emits = defineEmits(["switch-to-register"]);

const emitSwitchToRegister = () => {
  emits("switch-to-register");
};

// 用户账号数据
const testUsers = [
  {
    id: 1,
    username: "admin",
    email: "admin@example.com",
    password: "admin123", // 实际应用中密码应该加密存储
  },
  {
    id: 2,
    username: "tester",
    email: "tester@example.com",
    password: "test123",
  },
  {
    id: 3,
    username: "writer",
    email: "writer@example.com",
    password: "write123",
  },
];

const handleLogin = () => {
  // 查找匹配的用户
  const user = testUsers.find(
    (u) =>
      u.username === loginForm.username && u.password === loginForm.password
  );

  if (user) {
    // 登录成功，触发事件
    emits("login-success", user);
  } else {
    alert("用户名或密码错误");
  }
};
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

textarea.form-control {
  min-height: 120px;
  resize: vertical;
}
.text-center {
  text-align: center;
}

.mt-3 {
  margin-top: 15px;
}
</style>
<template>
  <div class="admin-container">
    <div class="admin-header">
      <h2>
        <i class="fas fa-users-cog" style="margin: 0 auto"></i> 用户管理系统
      </h2>
      <p>管理平台用户账户和权限</p>
    </div>
    <!-- 顶部操作栏 -->
    <div class="admin-header">
      <button class="btn-add" @click="showCreateDialog = true">
        <i class="el-icon-plus" /> 添加用户
      </button>
      <div class="search-box">
        <input
          v-model="searchQuery"
          placeholder="搜索用户..."
          class="search-input"
        />
        <button class="search-btn" @click="searchUsers">
          <i class="el-icon-search" />
          搜索
        </button>
      </div>
    </div>

    <!-- 用户表格 -->
    <table class="user-table">
      <thead>
        <tr>
          <th>ID</th>
          <th>用户名</th>
          <th>邮箱</th>
          <th>角色</th>
          <th>注册时间</th>
          <th>操作</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(user, index) in filteredUsers" :key="index">
          <td class="text-center">{{ user.id }}</td>
          <td class="text-center">
            <div
              class="avatar"
              :style="{ background: getAvatarColor(user.username) }"
            >
              {{ user.username.charAt(0).toUpperCase() }}
            </div>
            {{ user.username }}
          </td>
          <td class="text-center">{{ user.email }}</td>
          <td class="text-center">
            <el-tag
              :type="user.role === 'admin' ? 'primary' : 'info'"
              effect="light"
            >
              {{ user.role === "admin" ? "管理员" : "普通用户" }}
            </el-tag>
          </td>
          <td class="text-center">{{ formatDate(user.created_at) }}</td>
          <td class="text-center operation-col">
            <el-button
              icon="el-icon-edit"
              type="primary"
              size="mini"
              @click="openEditDialog(user)"
              >编辑</el-button
            >
            <el-button
              icon="el-icon-delete"
              type="danger"
              size="mini"
              @click="confirmDelete(user.id)"
              >删除</el-button
            >
          </td>
        </tr>
      </tbody>
    </table>

    <!-- 创建用户弹窗 -->
    <el-dialog title="添加新用户" v-model="showCreateDialog" width="500px">
      <el-form :model="newUser" label-width="80px">
        <el-form-item label="用户名">
          <el-input v-model="newUser.username" />
        </el-form-item>
        <el-form-item label="邮箱">
          <el-input v-model="newUser.email" type="email" />
        </el-form-item>
        <el-form-item label="密码">
          <el-input v-model="newUser.password" type="password" show-password />
        </el-form-item>
        <el-form-item label="角色">
          <el-radio-group v-model="newUser.role">
            <el-radio label="admin">管理员</el-radio>
            <el-radio label="user">普通用户</el-radio>
          </el-radio-group>
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showCreateDialog = false">取消</el-button>
          <el-button type="primary" @click="createUser">确认</el-button>
        </span>
      </template>
    </el-dialog>

    <!-- 编辑用户弹窗 -->
    <el-dialog title="编辑用户" v-model="showEditDialog" width="500px">
      <el-form :model="editUser" label-width="80px">
        <el-form-item label="用户名">
          <el-input v-model="editUser.username" />
        </el-form-item>
        <el-form-item label="邮箱">
          <el-input v-model="editUser.email" type="email" />
        </el-form-item>
        <el-form-item label="角色">
          <el-radio-group v-model="editUser.role">
            <el-radio label="admin">管理员</el-radio>
            <el-radio label="user">普通用户</el-radio>
          </el-radio-group>
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showEditDialog = false">取消</el-button>
          <el-button type="primary" @click="updateUser">确认</el-button>
        </span>
      </template>
    </el-dialog>

    <!-- 删除确认弹窗 -->
    <el-dialog title="删除确认" v-model="showDeleteDialog" width="300px">
      <div>确定删除该用户吗？</div>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showDeleteDialog = false">取消</el-button>
          <el-button type="danger" @click="deleteUser">确认删除</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed } from "vue";
import {
  ElMessage,
  ElDialog,
  ElButton,
  ElInput,
  ElForm,
  ElFormItem,
  ElTag,
  ElRadio,
  ElRadioGroup,
} from "element-plus";

// 模拟数据
const users = ref([
  {
    id: 1,
    username: "admin",
    email: "admin@example.com",
    role: "admin",
    created_at: "2023-01-15T10:30:00Z",
  },
  {
    id: 2,
    username: "user1",
    email: "user1@example.com",
    role: "user",
    created_at: "2023-02-20T14:15:00Z",
  },
  {
    id: 3,
    username: "user2",
    email: "user2@example.com",
    role: "user",
    created_at: "2023-03-10T09:45:00Z",
  },
  {
    id: 4,
    username: "editor",
    email: "editor@example.com",
    role: "user",
    created_at: "2023-04-06T16:20:00Z",
  },
]);

// 状态管理
const showCreateDialog = ref(false);
const showEditDialog = ref(false);
const showDeleteDialog = ref(false);
const searchQuery = ref("");
const deletingUserId = ref(null);

// 表单数据
const newUser = ref({ username: "", email: "", password: "", role: "user" });
const editUser = ref({ id: null, username: "", email: "", role: "" });

// 搜索过滤
const filteredUsers = computed(() => {
  return users.value.filter(
    (user) =>
      user.username.includes(searchQuery.value) ||
      user.email.includes(searchQuery.value)
  );
});

// 工具函数
const formatDate = (dateStr) => {
  return new Date(dateStr).toLocaleDateString("zh-CN", {
    year: "numeric",
    month: "long",
    day: "numeric",
  });
};

const getAvatarColor = (username) => {
  const colors = ["#f56c6c", "#409eff", "#67c23a", "#e6a23c", "#909399"];
  const hash = username
    .split("")
    .reduce((acc, char) => acc + char.charCodeAt(0), 0);
  return colors[hash % colors.length];
};

// 交互逻辑
const createUser = () => {
  users.value.push({
    ...newUser.value,
    id: users.value.length + 1,
    created_at: new Date().toISOString(),
  });
  showCreateDialog.value = false;
  ElMessage.success("用户创建成功！");
  newUser.value = { username: "", email: "", password: "", role: "user" };
};

const openEditDialog = (user) => {
  editUser.value = { ...user };
  showEditDialog.value = true;
};

const updateUser = () => {
  const index = users.value.findIndex((u) => u.id === editUser.value.id);
  users.value[index] = editUser.value;
  showEditDialog.value = false;
  ElMessage.success("用户信息更新成功！");
};

const confirmDelete = (userId) => {
  deletingUserId.value = userId;
  showDeleteDialog.value = true;
};

const deleteUser = () => {
  users.value = users.value.filter((u) => u.id !== deletingUserId.value);
  showDeleteDialog.value = false;
  ElMessage.success("用户删除成功！");
};

const searchUsers = () => {
  // 可扩展接口请求，这里仅演示过滤
  ElMessage.info("搜索功能已触发");
};
</script>

<style scoped>
/* 全局容器 */
.admin-container {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
  background: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
}

/* 顶部操作栏 */
.admin-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.btn-add {
  padding: 8px 16px;
  background: #409eff;
  color: #fff;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
}

.btn-add:hover {
  background: #66b1ff;
}

/* 搜索框 */
.search-box {
  display: flex;
  align-items: center;
}

.search-input {
  padding: 6px 12px;
  border: 1px solid #dcdcdc;
  border-radius: 4px 0 0 4px;
  outline: none;
}

.search-btn {
  padding: 6px 12px;
  background: #f0f0f0;
  border: 1px solid #dcdcdc;
  border-radius: 0 4px 4px 0;
  cursor: pointer;
}

.search-btn:hover {
  background: #e6e6e6;
}

/* 用户表格 */
.user-table {
  width: 100%;
  border-collapse: collapse;
  text-align: center;
}

.user-table th,
.user-table td {
  padding: 12px 8px;
  border: 1px solid #ebeef5;
}

.user-table th {
  background: #f8f9fa;
  font-weight: 500;
}

/* 头像样式 */
.avatar {
  display: inline-block;
  width: 32px;
  height: 32px;
  line-height: 32px;
  border-radius: 50%;
  color: #fff;
  text-align: center;
  margin-right: 8px;
}

/* 操作列 */
.operation-col {
  display: flex;
  justify-content: center;
  gap: 8px;
}

/* 弹窗样式增强 */
.el-dialog__header {
  border-bottom: 1px solid #ebeef5;
}

.el-dialog__footer {
  border-top: 1px solid #ebeef5;
}

.el-tag {
  margin: 0 4px;
}

/* 响应式 */
@media (max-width: 768px) {
  .admin-header {
    flex-direction: column;
    align-items: stretch;
    gap: 10px;
  }

  .user-table {
    font-size: 14px;
  }
}
</style>
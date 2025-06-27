// API 配置文件
import { config, logger } from '../config/env.js';

// 使用环境配置中的 API 基础 URL
const API_BASE_URL = config.API_BASE_URL;

// 获取存储的 JWT token
function getToken() {
  return localStorage.getItem('jwt_token');
}

// 设置 JWT token
function setToken(token) {
  localStorage.setItem('jwt_token', token);
}

// 清除 JWT token
function clearToken() {
  localStorage.removeItem('jwt_token');
}

// 基础请求函数
async function request(url, options = {}) {
  const token = getToken();
  const fullUrl = `${API_BASE_URL}${url}`;
  
  const requestConfig = {
    headers: {
      'Content-Type': 'application/json',
      ...(token && { 'Authorization': `Bearer ${token}` }),
      ...options.headers,
    },
    ...options,
  };

  if (requestConfig.body && typeof requestConfig.body === 'object') {
    requestConfig.body = JSON.stringify(requestConfig.body);
  }

  // 记录请求日志
  logger.debug('API Request:', {
    method: requestConfig.method || 'GET',
    url: fullUrl,
    headers: requestConfig.headers,
    body: requestConfig.body,
  });

  try {
    const response = await fetch(fullUrl, requestConfig);
    
    // 记录响应日志
    logger.debug('API Response:', {
      status: response.status,
      statusText: response.statusText,
      url: fullUrl,
    });
    
    if (!response.ok) {
      const errorData = await response.json().catch(() => ({ 
        message: 'Network error',
        status: response.status 
      }));
      
      logger.error('API Error:', {
        status: response.status,
        url: fullUrl,
        error: errorData,
      });
      
      throw new Error(errorData.message || `HTTP error! status: ${response.status}`);
    }
    
    const data = await response.json();
    logger.debug('API Success:', { url: fullUrl, data });
    
    return data;
  } catch (error) {
    logger.error('API request failed:', {
      url: fullUrl,
      error: error.message,
      stack: error.stack,
    });
    throw error;
  }
}

// 用户相关 API
export const userAPI = {
  // 用户注册
  register: (userData) => request('/user/register', {
    method: 'POST',
    body: userData,
  }),
  
  // 用户登录
  login: async (loginData) => {
    const response = await request('/user/login', {
      method: 'POST',
      body: { username_or_email: loginData.username_or_email || loginData.username, password: loginData.password },
    });
    if (response.token) {
      setToken(response.token);
    }
    return response;
  },
  
  // 用户登出
  logout: async () => {
    try {
      await request('/user/logout', { method: 'POST' });
    } finally {
      clearToken();
    }
  },
  
  // 获取当前用户信息
  getMe: () => request('/user/me'),
  
  // 更新个人信息
  updateMe: (userData) => request('/user/me', {
    method: 'PUT',
    body: userData,
  }),
};

// 文章相关 API
export const postAPI = {
  // 创建文章
  create: (postData) => {
    // 转换字段名以匹配后端API
    const apiData = {
      title: postData.title,
      content_markdown: postData.content || postData.content_markdown,
      category: postData.category,
      is_published: postData.is_published,
      cover_url: postData.cover_url
    };
    return request('/post', {
      method: 'POST',
      body: apiData,
    });
  },
  
  // 获取文章列表
  async list(options = {}) {
    const { page = 1, limit = 10, category = null, query = null, published_only = true } = options;
    const params = new URLSearchParams();
    params.append('page', page.toString());
    params.append('limit', limit.toString());
    if (category) params.append('category', category);
    if (query) params.append('query', query);
    if (published_only !== undefined) params.append('published_only', published_only.toString());
    
    return request(`/post?${params.toString()}`, {
      method: 'GET',
    });
  },
  
  // 获取文章详情
  get: (id) => request(`/post/${id}`),
  
  // 更新文章
  update: (id, postData) => {
    // 转换字段名以匹配后端API
    const apiData = {
      title: postData.title,
      content_markdown: postData.content || postData.content_markdown,
      category: postData.category,
      is_published: postData.is_published,
      cover_url: postData.cover_url
    };
    return request(`/post/${id}`, {
      method: 'PUT',
      body: apiData,
    });
  },
  
  // 删除文章
  delete: (id) => request(`/post/${id}`, {
    method: 'DELETE',
  }),
  
  // 搜索文章
  search: (query) => {
    const queryString = new URLSearchParams({ q: query }).toString();
    return request(`/post/search?${queryString}`);
  },
};

// 评论相关 API
export const commentAPI = {
  // 创建评论
  create: (commentData) => request('/comment', {
    method: 'POST',
    body: commentData,
  }),
  
  // 获取文章评论
  getByPost: (postId) => request(`/post/${postId}/comments`),
  
  // 获取评论详情
  get: (id) => request(`/comment/${id}`),
  
  // 更新评论
  update: (id, commentData) => request(`/comment/${id}`, {
    method: 'PUT',
    body: commentData,
  }),
  
  // 删除评论
  delete: (id) => request(`/comment/${id}`, {
    method: 'DELETE',
  }),
};

// 收藏相关 API
export const favoriteAPI = {
  // 切换收藏状态（添加/取消收藏）
  toggle: (postId) => request('/post_fav', {
    method: 'POST',
    body: { post_id: postId },
  }),
  
  // 获取用户收藏列表
  list: () => request('/post_fav/my/list'),
};

// 导出工具函数
export { getToken, setToken, clearToken };

// 默认导出
export default {
  userAPI,
  postAPI,
  commentAPI,
  favoriteAPI,
  getToken,
  setToken,
  clearToken,
};
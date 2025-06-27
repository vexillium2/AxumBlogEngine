// 环境配置文件
// 用于管理不同环境下的配置

// 获取当前环境
const getEnv = () => {
  // Vite 环境变量
  return import.meta.env.MODE || 'development';
};

// 环境配置
const envConfig = {
  development: {
    // 开发环境配置
    API_BASE_URL: '/api', // 使用 Vite 代理
    FRONTEND_URL: 'http://localhost:5173',
    BACKEND_URL: 'http://localhost:3000',
    DEBUG: true,
    LOG_LEVEL: 'debug',
  },
  production: {
    // 生产环境配置
    API_BASE_URL: '/api', // 生产环境中应该配置为实际的 API 地址
    FRONTEND_URL: window.location.origin,
    BACKEND_URL: window.location.origin,
    DEBUG: false,
    LOG_LEVEL: 'error',
  },
  test: {
    // 测试环境配置
    API_BASE_URL: 'http://localhost:3000/api',
    FRONTEND_URL: 'http://localhost:5173',
    BACKEND_URL: 'http://localhost:3000',
    DEBUG: true,
    LOG_LEVEL: 'info',
  },
};

// 获取当前环境配置
const getCurrentConfig = () => {
  const env = getEnv();
  return envConfig[env] || envConfig.development;
};

// 导出配置
export const config = getCurrentConfig();

// 导出环境检查函数
export const isDevelopment = () => getEnv() === 'development';
export const isProduction = () => getEnv() === 'production';
export const isTest = () => getEnv() === 'test';

// 日志工具
export const logger = {
  debug: (...args) => {
    if (config.DEBUG && config.LOG_LEVEL === 'debug') {
      console.log('[DEBUG]', ...args);
    }
  },
  info: (...args) => {
    if (config.DEBUG && ['debug', 'info'].includes(config.LOG_LEVEL)) {
      console.info('[INFO]', ...args);
    }
  },
  warn: (...args) => {
    if (config.DEBUG && ['debug', 'info', 'warn'].includes(config.LOG_LEVEL)) {
      console.warn('[WARN]', ...args);
    }
  },
  error: (...args) => {
    console.error('[ERROR]', ...args);
  },
};

// 默认导出
export default {
  config,
  isDevelopment,
  isProduction,
  isTest,
  logger,
};
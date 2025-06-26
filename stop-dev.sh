#!/bin/bash

# 停止开发环境脚本

echo "🛑 正在停止 Axum Blog Engine 开发环境..."

# 从 PID 文件读取进程 ID
if [ -f "logs/backend.pid" ]; then
    BACKEND_PID=$(cat logs/backend.pid)
    if ps -p $BACKEND_PID > /dev/null 2>&1; then
        echo "🔧 停止后端服务 (PID: $BACKEND_PID)..."
        kill $BACKEND_PID
        echo "✅ 后端服务已停止"
    else
        echo "ℹ️  后端服务未运行"
    fi
    rm -f logs/backend.pid
else
    echo "ℹ️  未找到后端 PID 文件，尝试通过端口停止..."
    # 尝试通过端口号杀死进程
    BACKEND_PID=$(lsof -ti:3000)
    if [ ! -z "$BACKEND_PID" ]; then
        echo "🔧 停止占用端口 3000 的进程 (PID: $BACKEND_PID)..."
        kill $BACKEND_PID
        echo "✅ 后端服务已停止"
    fi
fi

if [ -f "logs/frontend.pid" ]; then
    FRONTEND_PID=$(cat logs/frontend.pid)
    if ps -p $FRONTEND_PID > /dev/null 2>&1; then
        echo "🎨 停止前端服务 (PID: $FRONTEND_PID)..."
        kill $FRONTEND_PID
        echo "✅ 前端服务已停止"
    else
        echo "ℹ️  前端服务未运行"
    fi
    rm -f logs/frontend.pid
else
    echo "ℹ️  未找到前端 PID 文件，尝试通过端口停止..."
    # 尝试通过端口号杀死进程
    FRONTEND_PID=$(lsof -ti:5173)
    if [ ! -z "$FRONTEND_PID" ]; then
        echo "🎨 停止占用端口 5173 的进程 (PID: $FRONTEND_PID)..."
        kill $FRONTEND_PID
        echo "✅ 前端服务已停止"
    fi
fi

# 清理可能残留的进程
echo "🧹 清理残留进程..."
pkill -f "cargo run --bin server" 2>/dev/null || true
pkill -f "npm run dev" 2>/dev/null || true
pkill -f "vite" 2>/dev/null || true

echo "✅ 开发环境已完全停止"
echo "📋 如需重新启动，请运行: ./start-dev.sh"
#!/bin/bash

# 开发环境启动脚本
# 同时启动前端和后端服务

echo "🚀 启动 Axum Blog Engine 开发环境..."

# 检查是否安装了必要的依赖
echo "📦 检查依赖..."

# 检查 Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ 未找到 Rust/Cargo，请先安装 Rust"
    echo "   访问: https://rustup.rs/"
    exit 1
fi

# 检查 Node.js
if ! command -v node &> /dev/null; then
    echo "❌ 未找到 Node.js，请先安装 Node.js"
    echo "   访问: https://nodejs.org/"
    exit 1
fi

# 检查 npm
if ! command -v npm &> /dev/null; then
    echo "❌ 未找到 npm，请先安装 npm"
    exit 1
fi

echo "✅ 依赖检查完成"

# 创建日志目录
mkdir -p logs

# 启动后端服务
echo "🔧 启动后端服务 (端口 3000)..."
cd backend

# 检查数据库文件是否存在
if [ ! -f "blogdb.db" ]; then
    echo "📊 初始化数据库..."
    if command -v sqlite3 &> /dev/null; then
        sqlite3 blogdb.db ".read init.sql"
        echo "✅ 数据库初始化完成"
    else
        echo "⚠️  未找到 sqlite3，请手动创建数据库:"
        echo "   sqlite3 blogdb.db '.read init.sql'"
    fi
fi

# 检查 .env 文件
if [ ! -f ".env" ]; then
    echo "📝 创建 .env 文件..."
    cat > .env << EOF
DATABASE_URL=sqlite:blogdb.db
JWT_SECRET=your-super-secret-jwt-key-here-$(date +%s)
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
BCRYPT_COST=12
EOF
    echo "✅ .env 文件创建完成"
fi

# 构建并启动后端
echo "🔨 构建后端..."
cargo build
if [ $? -ne 0 ]; then
    echo "❌ 后端构建失败"
    exit 1
fi

echo "🚀 启动后端服务..."
cargo run --bin server > ../logs/backend.log 2>&1 &
BACKEND_PID=$!
echo "✅ 后端服务已启动 (PID: $BACKEND_PID)"

# 等待后端启动
echo "⏳ 等待后端服务启动..."
sleep 3

# 检查后端是否启动成功
if ! curl -s http://localhost:3000/api/post/list > /dev/null 2>&1; then
    echo "⚠️  后端服务可能未完全启动，继续启动前端..."
else
    echo "✅ 后端服务启动成功"
fi

# 启动前端服务
echo "🎨 启动前端服务 (端口 5173)..."
cd ../frontend

# 安装前端依赖
if [ ! -d "node_modules" ]; then
    echo "📦 安装前端依赖..."
    npm install
    if [ $? -ne 0 ]; then
        echo "❌ 前端依赖安装失败"
        kill $BACKEND_PID
        exit 1
    fi
    echo "✅ 前端依赖安装完成"
fi

echo "🚀 启动前端开发服务器..."
npm run dev > ../logs/frontend.log 2>&1 &
FRONTEND_PID=$!
echo "✅ 前端服务已启动 (PID: $FRONTEND_PID)"

# 保存 PID 到文件
echo $BACKEND_PID > ../logs/backend.pid
echo $FRONTEND_PID > ../logs/frontend.pid

echo ""
echo "🎉 开发环境启动完成！"
echo ""
echo "📍 服务地址:"
echo "   前端: http://localhost:5173"
echo "   后端: http://localhost:3000"
echo "   API文档: http://localhost:3000/api"
echo ""
echo "📋 管理命令:"
echo "   查看日志: tail -f logs/backend.log 或 tail -f logs/frontend.log"
echo "   停止服务: ./stop-dev.sh"
echo ""
echo "⏳ 等待服务完全启动..."
sleep 5

# 尝试打开浏览器
if command -v open &> /dev/null; then
    echo "🌐 正在打开浏览器..."
    open http://localhost:5173
elif command -v xdg-open &> /dev/null; then
    echo "🌐 正在打开浏览器..."
    xdg-open http://localhost:5173
else
    echo "🌐 请手动打开浏览器访问: http://localhost:5173"
fi

echo "✨ 开发环境已就绪！按 Ctrl+C 停止服务"

# 等待用户中断
trap 'echo "\n🛑 正在停止服务..."; kill $BACKEND_PID $FRONTEND_PID 2>/dev/null; echo "✅ 服务已停止"; exit 0' INT

# 保持脚本运行
while true; do
    sleep 1
done
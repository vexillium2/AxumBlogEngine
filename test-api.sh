#!/bin/bash

# API 测试脚本
# 用于测试前后端连接是否正常

API_BASE="http://localhost:3000/api"
FRONTEND_URL="http://localhost:5173"

echo "🧪 测试 Axum Blog Engine API 连接..."
echo ""

# 检查后端服务是否运行
echo "📡 检查后端服务 (端口 3000)..."
if curl -s "$API_BASE/post/list" > /dev/null 2>&1; then
    echo "✅ 后端服务运行正常"
else
    echo "❌ 后端服务未运行或无法访问"
    echo "   请确保后端服务已启动: cargo run --bin server"
    exit 1
fi

# 检查前端服务是否运行
echo "🎨 检查前端服务 (端口 5173)..."
if curl -s "$FRONTEND_URL" > /dev/null 2>&1; then
    echo "✅ 前端服务运行正常"
else
    echo "❌ 前端服务未运行或无法访问"
    echo "   请确保前端服务已启动: npm run dev"
    exit 1
fi

echo ""
echo "🔍 测试 API 接口..."
echo ""

# 测试获取文章列表
echo "📝 测试获取文章列表..."
RESPONSE=$(curl -s -w "\n%{http_code}" "$API_BASE/post?page=1&limit=10")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | head -n -1)

if [ "$HTTP_CODE" = "200" ]; then
    echo "✅ 文章列表 API 正常 (HTTP $HTTP_CODE)"
    echo "   响应: $(echo "$BODY" | head -c 100)..."
else
    echo "❌ 文章列表 API 异常 (HTTP $HTTP_CODE)"
    echo "   响应: $BODY"
fi

echo ""

# 测试用户注册 (使用测试数据)
echo "👤 测试用户注册..."
TEST_USER='{"username":"testuser_'$(date +%s)'","email":"test'$(date +%s)'@example.com","password":"test123456"}'
RESPONSE=$(curl -s -w "\n%{http_code}" -X POST \
    -H "Content-Type: application/json" \
    -d "$TEST_USER" \
    "$API_BASE/user/register")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | head -n -1)

if [ "$HTTP_CODE" = "200" ] || [ "$HTTP_CODE" = "201" ]; then
    echo "✅ 用户注册 API 正常 (HTTP $HTTP_CODE)"
    echo "   响应: $(echo "$BODY" | head -c 100)..."
else
    echo "⚠️  用户注册 API 响应异常 (HTTP $HTTP_CODE)"
    echo "   响应: $BODY"
    echo "   这可能是正常的，如果用户已存在或其他业务逻辑限制"
fi

echo ""

# 测试用户登录 (使用默认管理员账户)
echo "🔐 测试用户登录..."
LOGIN_DATA='{"username_or_email":"admin","password":"admin123"}'
RESPONSE=$(curl -s -w "\n%{http_code}" -X POST \
    -H "Content-Type: application/json" \
    -d "$LOGIN_DATA" \
    "$API_BASE/user/login")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | head -n -1)

if [ "$HTTP_CODE" = "200" ]; then
    echo "✅ 用户登录 API 正常 (HTTP $HTTP_CODE)"
    echo "   响应: $(echo "$BODY" | head -c 100)..."
    
    # 尝试提取 JWT token
    TOKEN=$(echo "$BODY" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)
    if [ ! -z "$TOKEN" ]; then
        echo "🎫 JWT Token 获取成功: ${TOKEN:0:20}..."
        
        # 测试需要认证的 API
        echo "🔒 测试需要认证的 API..."
        AUTH_RESPONSE=$(curl -s -w "\n%{http_code}" \
            -H "Authorization: Bearer $TOKEN" \
            "$API_BASE/user/get/me")
        AUTH_HTTP_CODE=$(echo "$AUTH_RESPONSE" | tail -n1)
        AUTH_BODY=$(echo "$AUTH_RESPONSE" | head -n -1)
        
        if [ "$AUTH_HTTP_CODE" = "200" ]; then
            echo "✅ 认证 API 正常 (HTTP $AUTH_HTTP_CODE)"
            echo "   用户信息: $(echo "$AUTH_BODY" | head -c 100)..."
        else
            echo "❌ 认证 API 异常 (HTTP $AUTH_HTTP_CODE)"
            echo "   响应: $AUTH_BODY"
        fi
    fi
else
    echo "❌ 用户登录 API 异常 (HTTP $HTTP_CODE)"
    echo "   响应: $BODY"
    echo "   请检查默认管理员账户是否已创建"
fi

echo ""
echo "📊 测试总结:"
echo "   后端服务: ✅ 运行中"
echo "   前端服务: ✅ 运行中"
echo "   API 连接: 请查看上述测试结果"
echo ""
echo "🌐 访问地址:"
echo "   前端应用: $FRONTEND_URL"
echo "   后端 API: $API_BASE"
echo ""
echo "💡 提示:"
echo "   - 如果 API 测试失败，请检查数据库是否已初始化"
echo "   - 如果认证失败，请确保默认管理员账户已创建"
echo "   - 查看详细日志: tail -f logs/backend.log"
echo ""
echo "✨ 测试完成！"
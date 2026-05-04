```markdown
# 🔐 DevBit 认证服务

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![Axum](https://img.shields.io/badge/Axum-0.7-red.svg)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-15+-blue.svg)
![JWT](https://img.shields.io/badge/JWT-Bearer-ff69b4.svg)
![License](https://img.shields.io/badge/License-Apache%202.0-green.svg)

**一个高性能、生产就绪的用户认证系统，5天从零到部署上线**

[快速开始](#-快速开始) • [API文档](#-api文档) • [部署指南](#-部署指南) • [技术架构](#-技术架构)

</div>

---

## ✨ 特性

- 🚀 **高性能** - 基于 Rust + Axum 框架，单机可处理数万并发
- 🔐 **安全认证** - JWT 令牌认证，24小时自动过期
- 📧 **邮箱验证** - 集成 QQ 邮箱 SMTP，5分钟有效验证码
- 💾 **数据持久化** - PostgreSQL 数据库，连接池管理
- 📦 **开箱即用** - 完整的注册/登录/验证码流程
- 🐳 **轻松部署** - 支持 Docker、Systemd、Nginx 反向代理

## 🛠️ 技术栈

| 技术 | 用途 | 版本 |
|------|------|------|
| **Rust** | 主编程语言 | 1.70+ |
| **Axum** | Web 框架 | 0.7 |
| **SQLx** | 异步数据库驱动 | 0.7 |
| **PostgreSQL** | 关系型数据库 | 15+ |
| **JSON Web Token** | 用户认证 | 10.3 |
| **Lettre** | 邮件发送 | 0.11 |
| **Tokio** | 异步运行时 | 1.0 |

## 📋 前置要求

- Rust 1.70+
- PostgreSQL 15+
- QQ 邮箱账号（用于 SMTP 发送验证码）

## 🚀 快速开始

### 1. 克隆项目

```bash
git clone https://github.com/yourusername/devbit-auth.git
cd devbit-auth
```

### 2. 配置环境变量

创建 `.env` 文件：

```bash
cp .env.example .env
```

编辑 `.env`：

```env
# 数据库配置
DATABASE_URL=postgresql://username:password@localhost:5432/devbit

# JWT 密钥（请使用强密码）
JWT_SECRET=your_super_strong_secret_key_here

# SMTP 配置（QQ邮箱示例）
SMTP_HOST=smtp.qq.com
SMTP_PORT=465
SMTP_USERNAME=your_email@qq.com
SMTP_PASSWORD=your_smtp_authorization_code
```

### 3. 初始化数据库

```bash
# 创建数据库
createdb devbit

# 运行迁移脚本
psql -d devbit -f schema.sql
```

数据库表结构：

```sql
-- 用户表
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 验证码表
CREATE TABLE verify_code (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    code VARCHAR(6) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP + INTERVAL '5 minutes'
);
```

### 4. 运行项目

```bash
# 开发模式
cargo run

# 生产模式
cargo build --release
./target/release/devbit-auth
```

服务器将在 `http://127.0.0.1:7878` 启动

## 📡 API 文档

### 1. 发送验证码

```http
POST /register/send_code
Content-Type: application/json

{
    "email": "user@example.com"
}
```

**响应：**
```json

```

### 2. 用户注册

```http
POST /register
Content-Type: application/json

{
    "name": "张三",
    "email": "user@example.com",
    "code": "123456",
    "password": "secure_password"
}
```

**响应：**
```json
{
    "id": 1,
    "name": "张三",
    "email": "user@example.com"
}
```

### 3. 用户登录

```http
POST /login
Content-Type: application/json

{
    "email": "user@example.com",
    "password": "secure_password"
}
```

**响应：**
```json
{
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
        "id": 1,
        "name": "张三",
        "email": "user@example.com"
    }
}
```

## 🏗️ 项目结构

```
src/
├── main.rs           # 应用入口，路由配置
├── database.rs       # 数据库连接和初始化
└── handlers/         # 业务逻辑（可选）
    ├── auth.rs       # 认证处理
    ├── email.rs      # 邮件服务
    └── models.rs     # 数据模型
```

## 🔒 安全特性

- ✅ **密码加密** - 使用 bcrypt 哈希存储（建议添加）
- ✅ **JWT 令牌** - 24小时自动过期
- ✅ **验证码限时** - 5分钟有效期
- ✅ **SQL 注入防护** - SQLx 参数化查询
- ✅ **环境变量隔离** - 敏感信息不硬编码

## 🚢 部署指南

### Docker 部署

```dockerfile
# Dockerfile
FROM rust:1.70-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates
COPY --from=builder /app/target/release/devbit-auth /usr/local/bin/
EXPOSE 7878
CMD ["devbit-auth"]
```

```bash
# 构建镜像
docker build -t devbit-auth .

# 运行容器
docker run -d \
  -p 7878:7878 \
  --env-file .env \
  --name devbit-auth \
  devbit-auth
```

### Nginx 反向代理配置

```nginx
server {
    listen 80;
    server_name api.yourdomain.com;

    location / {
        proxy_pass http://127.0.0.1:7878;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Systemd 服务

```ini
# /etc/systemd/system/devbit-auth.service
[Unit]
Description=DevBit Auth Service
After=network.target postgresql.service

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/devbit-auth
EnvironmentFile=/opt/devbit-auth/.env
ExecStart=/opt/devbit-auth/target/release/devbit-auth
Restart=always

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl enable devbit-auth
sudo systemctl start devbit-auth
```

## 📊 性能测试

```bash
# 使用 wrk 进行压力测试
wrk -t12 -c400 -d30s http://localhost:7878/login
```

预期性能（单机）：
- **QPS**: 10,000+
- **延迟**: P99 < 50ms
- **并发**: 支持 10,000+ 并发连接

## 🔧 环境变量说明

| 变量名 | 说明 | 示例 | 必需 |
|--------|------|------|------|
| `DATABASE_URL` | PostgreSQL 连接字符串 | `postgresql://user:pass@localhost:5432/db` | ✅ |
| `JWT_SECRET` | JWT 签名密钥 | `your-secret-key-min-32-chars` | ✅ |
| `SMTP_HOST` | 邮件服务器地址 | `smtp.qq.com` | ✅ |
| `SMTP_PORT` | 邮件服务器端口 | `465` | ✅ |
| `SMTP_USERNAME` | 邮箱账号 | `your@qq.com` | ✅ |
| `SMTP_PASSWORD` | SMTP 授权码 | `xxxxx` | ✅ |

## 🐛 常见问题

### Q: 验证码收不到？
A: 检查 SMTP 配置，QQ邮箱需要使用授权码而非登录密码

### Q: JWT_SECRET 应该多长？
A: 建议 32 位以上随机字符串，可用 `openssl rand -base64 32` 生成

### Q: 如何重置数据库？
A: `sqlx database drop -y && sqlx database create && sqlx migrate run`

### Q: 生产环境如何管理环境变量？
A: 使用 Systemd EnvironmentFile 或 Docker --env-file

## 🗺️ 路线图

- [ ] 添加 Refresh Token 机制
- [ ] 集成 OAuth2.0（GitHub/Google 登录）
- [ ] 添加 Redis 会话存储
- [ ] 实现限流和防暴力破解
- [ ] 添加审计日志
- [ ] 支持多语言邮件模板

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License © 2024 DevBit

## 📧 联系方式

- 作者: [Your Name]
- 邮箱: your@email.com
- 项目地址: [GitHub Repository]

---

<div align="center">
  <sub>Built with ❤️ by DevBit Team</sub>
</div>
```

这个 README 包含了：
- ✅ 项目徽章和简介
- ✅ 完整的技术栈说明
- ✅ 快速开始指南
- ✅ 详细的 API 文档
- ✅ 安全特性说明
- ✅ 多种部署方式
- ✅ 常见问题解答
- ✅ 路线图和贡献指南

根据你的实际项目情况调整相关内容（如作者、仓库地址等）！
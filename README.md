DevBit Forum API
一个基于 Rust 的高性能社区论坛后端服务，采用 Axum 框架构建，提供 RESTful API 接口。

项目简介
DevBit Forum API 是一个轻量级、高性能的论坛系统后端，支持用户认证、帖子管理、邮件验证码等核心功能。项目采用现代化的 Rust 技术栈，具有出色的并发处理能力和安全性保障。

技术栈
框架: Axum - 基于 Tokio 的高性能 Web 框架

数据库: PostgreSQL + SQLx - 类型安全的异步数据库操作

认证: JWT (JSON Web Token) - 基于 jsonwebtoken 实现

邮件服务: Lettre - 支持 SMTP 的邮件发送

序列化: Serde - 高性能的序列化/反序列化框架

时间处理: Chrono - 日期时间处理库

功能特性
✅ 用户注册与登录

✅ JWT 身份认证

✅ 邮箱验证码发送（SMTP）

✅ 论坛帖子 CRUD 操作

✅ 帖子分类与标签系统

✅ 浏览量统计

✅ Bootstrap 数据预加载

✅ 类型安全的数据库查询

快速开始
环境要求
Rust 1.70+

PostgreSQL 14+

SMTP 邮件服务（QQ邮箱或其他）

安装步骤
克隆项目

bash
git clone https://github.com/EpsilonHunter/devbit-forum-api.git
cd devbit-forum-api
配置环境变量

创建 .env 文件：

env
DATABASE_URL=postgres://username:password@localhost/devbit_forum
JWT_SECRET=your_jwt_secret_key_here
SMTP_USERNAME=your_email@qq.com
SMTP_PASSWORD=your_smtp_authorization_code
SMTP_SERVER=smtp.qq.com
初始化数据库

sql
-- 用户表
CREATE TABLE users (
id SERIAL PRIMARY KEY,
name VARCHAR(100) NOT NULL,
email VARCHAR(255) UNIQUE NOT NULL,
password VARCHAR(255) NOT NULL,
avatar VARCHAR(500) DEFAULT '',
is_admin BOOLEAN DEFAULT FALSE,
created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 验证码表
CREATE TABLE verify_code (
email VARCHAR(255) PRIMARY KEY,
code VARCHAR(6) NOT NULL,
created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 帖子表
CREATE TABLE posts (
id SERIAL PRIMARY KEY,
title VARCHAR(200) NOT NULL,
content TEXT NOT NULL,
author_id INTEGER REFERENCES users(id),
category TEXT NOT NULL DEFAULT 'general',
tags TEXT[] DEFAULT '{}',
created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
view_count INTEGER DEFAULT 0,
comment_count INTEGER DEFAULT 0,
like_count INTEGER DEFAULT 0,
is_pinned BOOLEAN DEFAULT FALSE,
is_locked BOOLEAN DEFAULT FALSE
);
编译运行

bash
cargo build --release
cargo run --release
服务默认运行在 http://127.0.0.1:7878

API 文档
认证相关
发送验证码
http
POST /register/send_code
Content-Type: application/json

{
"email": "user@example.com"
}
用户注册
http
POST /register
Content-Type: application/json

{
"name": "username",
"email": "user@example.com",
"code": "123456",
"password": "password123"
}
用户登录
http
POST /login
Content-Type: application/json

{
"email": "user@example.com",
"password": "password123"
}
响应：

json
{
"token": "eyJhbGciOiJIUzI1NiIs...",
"user": {
"id": 1,
"name": "username",
"email": "user@example.com"
}
}
论坛相关
Bootstrap 数据加载
http
GET /forum/bootstrap
获取帖子详情
http
GET /forum/posts/{id}
创建帖子
http
POST /forum/posts
Authorization: Bearer {token}
Content-Type: application/json

{
"title": "Post Title",
"content": "Post content...",
"category": "tech",
"tags": ["rust", "axum"]
}
帖子分类
支持的分类类型：

general - 综合讨论

tech - 技术交流

devbit - DevBit 专区

help - 求助问答

showcase - 作品展示

announcement - 公告通知

项目结构
text
src/
├── main.rs          # 主入口、路由配置、请求处理
└── database.rs      # 数据库连接池初始化
安全特性
JWT 令牌有效期 24 小时

密码验证码机制（5分钟有效期）

Bearer Token 认证

邮箱唯一性验证

输入参数类型安全校验

开发团队
后端开发: EpsilonHunter

前端开发: Clearders

许可证
本项目采用 Apache License 2.0 开源协议。

text
Copyright 2024 EpsilonHunter

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
贡献指南
欢迎提交 Issue 和 Pull Request！

Fork 本仓库

创建特性分支 (git checkout -b feature/AmazingFeature)

提交更改 (git commit -m 'Add some AmazingFeature')

推送到分支 (git push origin feature/AmazingFeature)

创建 Pull Request

联系方式
Issues: GitHub Issues

邮箱: 2043399410@qq.com

<div align="center"> Made with ❤️ by DevBit Team </div>
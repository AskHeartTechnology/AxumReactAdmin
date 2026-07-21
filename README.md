<h1 style="text-align: center;"> Axum React Admin (ARA) </h1>

<p style="text-align: center;">
  <img src="https://img.shields.io/badge/Rust-1.80+-orange.svg" alt="Rust Version">
  <img src="https://img.shields.io/badge/Axum-0.8.7-blue.svg" alt="Axum Version">
  <img src="https://img.shields.io/badge/React-19.2-blue.svg" alt="React Version">
  <img src="https://img.shields.io/badge/TypeScript-5.9-blue.svg" alt="TypeScript Version">
  <img src="https://img.shields.io/badge/License-MIT-green.svg" alt="License">
</p>

## 📖 项目简介

> ARA (Axum React Admin) 是一个基于 [Axum](https://github.com/tokio-rs/axum) 和 [React](https://github.com/facebook/react) 开发的现代化全栈、前后端分离的后台管理系统基础平台。
> 该项目采用主流技术栈，旨在提供高性能、安全、易扩展的企业级解决方案。

### ✨ 特性

- 🚀 **高性能**：后端使用 Rust + Axum 框架，提供极致性能和内存安全
- 🎨 **现代化 UI**：前端采用 Vite + React + Shadcn UI，提供优雅的用户体验
- 🔐 **安全可靠**：基于 JWT 的身份认证，RBAC 权限控制
- 📦 **开箱即用**：内置常用功能模块，快速搭建管理系统
- 🔧 **易于扩展**：清晰的代码结构，便于二次开发
- 📝 **API 文档**：集成 Swagger，自动生成 API 文档

## 🛠️ 技术栈

### 后端技术

- **框架**：[Axum](https://github.com/tokio-rs/axum) - 基于 Tokio 的高性能 Web 框架
- **运行时**：[Tokio](https://tokio.rs/) - 异步运行时
- **数据库**：[PostgreSQL](https://www.postgresql.org/) - 关系型数据库
- **ORM**：[SeaORM](https://www.sea-ql.org/SeaORM/) - 异步 ORM 框架
- **缓存**：[Redis](https://redis.io/) - 内存数据库，用于 JWT 和高频数据缓存
- **日志**：[Tracing](https://github.com/tokio-rs/tracing) - 结构化日志系统
- **API 文档**：[Utoipa](https://github.com/juhaku/utoipa) - API 文档生成

### 前端技术

- **框架**：[React](https://react.dev/) - UI 框架
- **语言**：[TypeScript](https://www.typescriptlang.org/) - 类型安全
- **UI 库**：[Shadcn UI](https://ui.shadcn.com/) - 基于 Radix + Tailwind 完全可定制的组件集
- **构建工具**：[Vite](https://vitejs.dev/) - 下一代前端构建工具
- **状态管理**：[Zustand](https://zustand-demo.pmnd.rs/) - 轻量级状态管理库
- **路由**：[React Router](https://reactrouter.com/) - 声明式路由管理
- **HTTP 客户端**：[Axios](https://axios-http.com/) - 基于 Promise 的 HTTP 客户端

## 📁 项目结构

```txt
AxumReactAdmin/               # 项目根
├── app/                      # 前端应用目录
│   ├── src/
│   │   ├── main.tsx          # 前端入口
│   │   ├── pages/            # 页面组件
│   │   ├── components/       # 通用组件
│   │   ├── router/           # 路由配置
│   │   ├── api/              # API 接口
│   │   ├── store/            # 状态管理
│   │   ├── utils/            # 工具函数
│   │   └── assets/           # 静态资源
│   ├── package.json          # 前端依赖配置
│   └── vite.config.ts        # Vite 配置
├── Cargo.toml
├── .env                      # 项目环境变量
├── config/                   # 项目配置文件
│   ├── default.toml
│   ├── development.toml
│   └── production.toml
├── migrations/               # 数据库迁移（sea-orm）
│   └── 20260101000000_init.sql
├── src/
│   ├── main.rs               # 项目入口
│   ├── lib.rs                # 组装 Router、启动服务
│   ├── config.rs             # 配置加载
│   ├── error.rs              # 统一错误类型 AppError
│   ├── state.rs              # AppState
│   ├── routes/               # 路由注册（只负责定义结构）
│   │   ├── mod.rs
│   │   └── users.rs
│   ├── handlers/             # HTTP 处理（解析请求、调 service、返回响应）
│   │   ├── mod.rs
│   │   └── users.rs
│   ├── services/             # 业务逻辑
│   │   ├── mod.rs
│   │   └── users.rs
│   ├── models/               # 数据库实体
│   │   ├── mod.rs
│   │   ├── users.rs
│   └── middleware/           # 中间件 鉴权、日志、CORS 等
│       ├── mod.rs
│       └── auth.rs
└── tests/                    # 测试用例
    └── api_test.rs
```

## 🚀 快速开始

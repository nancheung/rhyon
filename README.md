# Rhyon ✨
*一个使用 Rust 构建的快速、可扩展且优雅的博客平台*

Rhyon 是一个轻量、极致快速的博客系统，适用于希望掌控内容并拥有美好写作体验的个人或社区。它基于 Rust 构建，具备高安全性、高性能，并为未来扩展做好准备。

---

## 🔧 技术栈

### 后端（开源部分）
- 🦀 [**Rust**](https://www.rust-lang.org/) — 安全、现代、并发友好的系统级语言
- 🕸 [**Axum**](https://docs.rs/axum) — 基于 Tower 的高性能 Web 框架
- 🐚 [**SeaORM**](https://www.sea-ql.org/SeaORM/) — 强类型异步 ORM
- 🐘 [**PostgreSQL**](https://www.postgresql.org/) — 稳定可靠的开源数据库

### 前端（暂未开源）
- ⚛️ [**Next.js**](https://nextjs.org/)
- 🧩 [**Tailwind CSS**](https://tailwindcss.com/)
> _Rhyon 的前端目前为私有项目，未来计划开源或提供开放 API 接入方案。_

---

## ✨ 功能特性

- 📝 **支持 Markdown 的博客系统**，可创建自定义页面，支持丰富的内容排版
- 💬 **评论系统**，支持评论管理和审核
- 🧩 **可扩展 API**，便于自定义主题与插件开发
- ⚙️ **后台管理界面**，可管理文章、设置与站点外观
- 👥 **多用户支持**
- 🚀 基于 SeaORM 和现代 Rust 工具链开发 
- （以上都 ***即将推出！***）
---

## 🛠️ 快速开始（Docker）

### 环境要求

- Docker
- PostgreSQL 数据库

### 启动步骤

```bash
# 克隆项目
git clone https://github.com/nanchueng/rhyon.git
cd rhyon
# 构建镜像
docker build -t rhyon .

# 配置环境变量
# 修改 .env 中的数据库连接配置
# 初始化数据库

# 启动容器
docker run -d --name rhyon -p 8080:8080 rhyon
```
---
## 🌱 开发计划
- [x] Markdown 支持 
- [ ] 评论系统
- [ ] 管理后台（基础）
- [ ] 主题支持
- [ ] 用户注册与权限管理
- [ ] API 文档与第三方接入支持
---
## ❤️ 关于名字
> **Rhyon** — 来自“Rhythm（节奏）”与 “Beyond（远方）”，象征自由表达与思想的延伸。
--- 
## 📄 开源协议 
本项目遵循 **[AGPL-3.0 License](https://github.com/nancheung/rhyon/blob/main/LICENSE)** 开源协议。

由 [nancheung](https://github.com/nancheung) 使用 🦀 Rust 构建并维护。
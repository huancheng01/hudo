---
title: "PostgreSQL 安装配置 - Windows 一键安装 PostgreSQL - hudo"
description: "使用 hudo 一键安装 PostgreSQL 数据库，自动初始化数据目录、注册 Windows 服务并配置环境变量，开箱即用。"
head:
  - - meta
    - name: keywords
      content: "PostgreSQL 安装, Windows PostgreSQL, PG 数据库安装, PostgreSQL 配置, PostgreSQL 免安装, 数据库环境搭建, hudo"
---

# PostgreSQL

PostgreSQL 是功能最强大的开源关系型数据库，以其对 SQL 标准的完整支持、JSONB 文档存储和强大的扩展生态（如 PostGIS、pgvector）著称。它是需要复杂查询、数据完整性和高级数据类型的开发者的首选。

PostgreSQL 关系型数据库。

## 安装

```powershell
hudo install pgsql
```

安装到 `{install_root}\tools\pgsql\`，自动获取最新版本，自动注册为 Windows 服务（需要 UAC 提权）。

## 安装后

```powershell
psql --version

# 连接
psql -U postgres
```

## 服务管理

```powershell
# 启动
net start PostgreSQL

# 停止
net stop PostgreSQL
```

## 卸载

```powershell
hudo uninstall pgsql
```

## 注意

- 服务注册需要管理员权限，安装时会弹出 UAC 提示
- 服务名为 `PostgreSQL`

## hudo 安装优势

- **自动初始化数据目录**：`initdb` 自动执行，无需手动配置数据目录和编码
- **自动注册 Windows 服务**：安装后即可通过 `net start PostgreSQL` 管理，支持开机自启
- **免去官方安装器**：不需要下载 EDB 安装包、选择组件、配置端口等繁琐流程
- **环境变量自动配置**：安装后直接使用 `psql`、`pg_dump` 等命令行工具
- **干净卸载**：自动停止服务、注销服务、清理数据和环境变量

## 常见问题

### 安装后 `psql` 连接提示认证失败？

默认使用 `trust` 认证方式，用户名为 `postgres`。如果遇到认证问题，检查 `pg_hba.conf` 文件中的认证配置。

### PostgreSQL 和 [MySQL](/tools/mysql) 应该选哪个？

PostgreSQL 在 SQL 标准支持、JSONB 文档存储、全文搜索和扩展生态（pgvector 向量搜索、PostGIS 地理信息）方面更强大。MySQL 则上手更简单，Web 框架默认支持更广泛。

### 如何使用图形化管理工具？

推荐安装 [pgAdmin](https://www.pgadmin.org/) 或在 [VS Code](/tools/vscode) 中使用 Database Client 扩展连接管理。

## 相关阅读

- [Windows PostgreSQL 快速安装指南：从下载到第一条 SQL](/blog/windows-postgresql-install) — 详细安装教程与常见问题解答

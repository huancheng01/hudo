---
title: "Redis 安装配置 - Windows 一键安装 Redis 数据库"
description: "使用 hudo 一键安装 Windows 版 Redis 内存数据库，自动注册系统服务并配置环境变量，支持后台运行和开机自启。"
head:
  - - meta
    - name: keywords
      content: "Redis 安装, Windows Redis, Redis Windows 版, Redis 服务安装, 内存数据库, Redis 环境配置, hudo"
---

# Redis

Redis 是高性能的内存键值数据库，常用于缓存、会话管理、消息队列和实时排行榜等场景。由于 Redis 官方不提供 Windows 版本，hudo 使用社区维护的预编译包让 Windows 开发者也能便捷地使用 Redis。

Redis 内存数据库，使用 [redis-windows](https://github.com/redis-windows/redis-windows) 提供的 Windows 预编译包。

## 安装

```powershell
hudo install redis
```

安装到 `{install_root}\tools\redis\`，自动获取最新版本，自动注册为 Windows 服务（需要 UAC 提权）。

## 安装后

```powershell
redis-server --version

# 连接
redis-cli
```

## 服务管理

```powershell
# 启动
net start Redis

# 停止
net stop Redis
```

## 卸载

```powershell
hudo uninstall redis
```

## 注意

- 服务注册需要管理员权限，安装时会弹出 UAC 提示
- 服务名为 `Redis`
- 默认绑定 `127.0.0.1:6379`
- 数据目录在 `tools\redis\data\`

## hudo 安装优势

- **解决 Windows 兼容问题**：Redis 官方不提供 Windows 版本，hudo 自动下载社区维护的可靠预编译包
- **自动注册系统服务**：安装后 Redis 在后台运行，开机自启，无需每次手动启动
- **一条命令完成**：不需要自己去 GitHub 找 Windows 构建版、解压配置、注册服务
- **干净卸载**：自动停止服务、注销服务、清理数据目录和环境变量

## 常见问题

### 端口 6379 被占用怎么办？

检查是否已有其他 Redis 实例在运行：`netstat -ano | findstr 6379`。如果有，先停止旧实例再安装。

### Redis 数据持久化在哪里？

数据目录在 `{install_root}\tools\redis\data\`，Redis 默认启用 RDB 持久化，重启后数据不会丢失。

### 如何在项目中连接 Redis？

安装后 Redis 默认监听 `127.0.0.1:6379`，无密码。在项目中直接使用默认连接参数即可，如 Node.js 的 `ioredis` 或 Python 的 `redis-py`。

## 相关阅读

- [Windows 也能用 Redis：2026 最新安装方案 + 开机自启配置](/blog/windows-redis-install) — 详细安装教程与常见问题解答

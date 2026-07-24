---
title: "MySQL 安装配置 - Windows 一键安装 MySQL 数据库"
description: "使用 hudo 一键安装 MySQL Community Server，自动注册 Windows 服务、配置环境变量和 root 密码，免去繁琐的手动配置。"
head:
  - - meta
    - name: keywords
      content: "MySQL 安装, Windows MySQL, MySQL 免安装配置, MySQL Community Server, MySQL 环境变量, 数据库安装, hudo"
---

# MySQL

MySQL 是全球最流行的开源关系型数据库管理系统，广泛应用于 Web 后端、企业应用和数据存储场景。无论是学习 SQL 还是开发生产级应用，MySQL 都是 Windows 开发者必备的数据库工具。

MySQL Community Server，关系型数据库。

## 安装

```powershell
hudo install mysql
```

安装到 `{install_root}\tools\mysql\`，自动获取最新 LTS 版本（只跟随 LTS 周期，不安装三个月即停止支持的 innovation 版本），自动注册为 Windows 服务（需要 UAC 提权）。可通过[配置文件](/guide/config)的 `versions.mysql` 锁定版本。

## 安装后

```powershell
mysql --version

# 连接（初始无密码）
mysql -u root
```

## 服务管理

```powershell
# 启动
net start MySQL

# 停止
net stop MySQL
```

## 卸载

```powershell
hudo uninstall mysql
```

## 注意

- 服务注册需要管理员权限，安装时会弹出 UAC 提示
- 服务名为 `MySQL`

## hudo 安装优势

- **自动注册 Windows 服务**：安装后 MySQL 作为系统服务运行，支持开机自启，无需手动配置
- **免去安装向导**：不需要下载 MySQL Installer、选择组件、配置 root 密码等繁琐步骤
- **自动配置 PATH**：安装后直接在任意终端使用 `mysql` 命令，无需手动添加环境变量
- **干净卸载**：`hudo uninstall mysql` 自动停止服务、注销服务、清理环境变量和文件
- **版本自动获取**：默认安装最新稳定版，也可通过[配置文件](/guide/config)指定版本

## 常见问题

### 安装后无法连接 MySQL？

确认服务已启动：`net start MySQL`。如果服务未注册成功，可能是 UAC 提权被拒绝，重新运行 `hudo install mysql` 并在弹出提示时点击「是」。

### 如何修改 root 密码？

首次安装后 root 无密码，连接后执行：

```sql
ALTER USER 'root'@'localhost' IDENTIFIED BY '你的新密码';
```

### MySQL 和 [PostgreSQL](/tools/pgsql) 应该选哪个？

MySQL 上手更简单、生态更广（WordPress、Laravel 等默认支持），适合 Web 开发入门。PostgreSQL 功能更强大，适合需要复杂查询、JSONB 或地理空间数据的场景。

## 相关阅读

- [Windows MySQL 安装不求人：一键部署 + 服务注册 + 环境变量配置](/blog/windows-mysql-install) — 详细安装教程与常见问题解答

---
date: 2026-03-15
author: Zexa
title: Windows 也能用 Redis：2026 最新安装方案 + 开机自启配置
description: Redis 官方不支持 Windows，本文介绍 2026 年最新的 Windows Redis 安装方案，从社区构建版下载到服务注册、AOF 持久化配置，以及如何用 hudo 一条命令完成全部步骤。
keywords:
  - Windows
  - Redis
  - 安装
  - 教程
  - 服务
  - 开机自启
  - hudo
---

# Windows 也能用 Redis：2026 最新安装方案 + 开机自启配置

想在 Windows 上用 Redis？第一个坏消息：Redis 官方从未正式支持 Windows。搜出来的教程大多指向微软归档的 Redis 3.x 版本——那已经是 2016 年的产物，既不安全也没有现代特性。

好消息是，2026 年我们有更好的选择。

## 现在 Windows 用什么版本的 Redis

社区项目 [redis-windows](https://github.com/redis-windows/redis-windows) 持续跟进官方版本，提供基于 MSYS2 编译的 Windows 构建包，目前已支持 Redis 8.x。每次 Redis 官方发版后不久就能拿到对应的 Windows 版本，功能完整，生产可用。

不过，这些包托管在 GitHub Releases 上，国内下载速度经常不理想，动辄几十 KB/s，大文件下到一半断掉是常有的事。

## 传统手动安装步骤

如果你选择自己动手，流程大致如下：

1. 去 GitHub 下载 `Redis-x.x.x-Windows-x64-msys2-with-Service.zip`
2. 解压到目标目录，比如 `D:\redis`
3. 编辑 `redis.conf`，配置 `bind 127.0.0.1`、`port 6379`、`appendonly yes` 等参数
4. 用 `RedisService.exe install` 注册 Windows 服务（需要管理员权限）
5. 执行 `net start Redis` 启动服务
6. 将 Redis 目录手动添加到系统 PATH 环境变量

全程六步，还得处理 UAC 提权、路径配置和防火墙。对于只想快速用上 Redis 的开发者来说，这些步骤显得繁琐。

## 用 hudo 一条命令搞定

[hudo](https://hudo.zexa.cc) 是一个 Windows 开发环境一键引导工具，支持 20 多种常用开发工具的自动安装。Redis 当然也在其中：

```powershell
hudo install redis
```

这条命令背后自动完成了以下步骤：

- **下载解压**：从 GitHub 下载最新的 redis-windows 构建包，下载失败时自动回退国内镜像
- **生成 redis.conf**：自动写入配置文件，默认绑定 `127.0.0.1`、端口 `6379`、开启 AOF 持久化
- **注册 Windows 服务**：调用 `RedisService.exe install`，权限不足时自动弹出 UAC 提权窗口
- **启动服务**：自动执行 `net start Redis`，安装完直接可用
- **配置环境变量**：将 Redis 目录写入用户 PATH，新终端直接可用 `redis-cli` 命令

整个过程一两分钟，结束后终端会提示连接和停止命令。

## 验证安装

打开一个新终端，输入：

```bash
redis-cli ping
```

看到 `PONG` 就说明 Redis 已经在正常运行了。

## 数据持久化：自动开启 AOF

很多人在 Windows 上用 Redis 时忘了配置持久化，一重启数据全没了。hudo 生成的 `redis.conf` 默认开启了 AOF（Append Only File）持久化：

```
bind 127.0.0.1
port 6379
dir D:/hudo/tools/redis/data
appendonly yes
appendfilename "appendonly.aof"
```

数据文件存放在 `tools/redis/data/` 目录下。即使 Redis 服务重启，数据也不会丢失。如果需要调整配置，直接编辑 `redis.conf` 然后重启服务即可。

## 开机自启：注册为 Windows 服务

hudo 安装时已经把 Redis 注册为 Windows 服务。这意味着每次开机后 Redis 会自动启动，无需手动操作。你可以通过以下命令管理服务：

```powershell
net stop Redis    # 停止
net start Redis   # 启动
```

也可以在"服务"管理面板中查看 Redis 的运行状态。

## 不需要了？一条命令卸载

```powershell
hudo uninstall redis
```

hudo 会自动停止服务、移除服务注册、清理安装目录和环境变量。

## 总结

Redis 官方不支持 Windows 不代表 Windows 用不了 Redis。借助社区的 redis-windows 项目，我们已经能在 Windows 上运行最新版的 Redis 8.x。而 `hudo install redis` 把下载、配置、服务注册、持久化、环境变量这些步骤全部自动化了，一条命令即可获得一个开箱即用的 Redis 环境。

如果你还没有安装 hudo，一行 PowerShell 即可：

```powershell
irm hudo.zexa.cc/install.ps1 | iex
```


---

> 查看 [Redis 工具文档](/tools/redis) 了解完整安装参数与配置选项。

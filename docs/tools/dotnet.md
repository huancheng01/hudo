---
title: ".NET SDK 安装配置 - Windows 一键安装 .NET"
description: "使用 hudo 一键安装 .NET SDK（C#/F#），官方脚本用户级安装，自动配置 DOTNET_ROOT 与 PATH，默认最新 LTS。"
head:
  - - meta
    - name: keywords
      content: ".NET SDK 安装, dotnet Windows, C# 开发环境, .NET LTS, dotnet-install, hudo"
---

# .NET SDK

.NET 是微软的跨平台开发框架，C#/F# 的官方工具链。hudo 通过官方 `dotnet-install.ps1` 脚本做用户级安装，默认跟随最新活跃 LTS 通道（STS 通道 18 个月即停止支持，不作默认）。

## 安装

```powershell
hudo install dotnet
```

安装到 `{install_root}\lang\dotnet\`，免管理员权限，自动配置 `DOTNET_ROOT` 和 PATH。

## 安装后

```powershell
dotnet --version

# 新建控制台项目
dotnet new console -o hello && cd hello && dotnet run
```

## 卸载

```powershell
hudo uninstall dotnet
```

## 配置文件版本

```toml
[versions]
# 不填则自动获取最新活跃 LTS 的 SDK 版本
dotnet = "10.0.302"
```

## hudo 安装优势

- **免管理员**：官方安装器为机器级安装，hudo 走官方脚本的用户级模式
- **只跟 LTS**：默认不安装 18 个月即停止支持的 STS 版本
- **版本可锁定**：通过[配置文件](/guide/config)固定 SDK 版本，团队环境可复现

## 常见问题

### 和系统里已装的 .NET 冲突吗？

`DOTNET_ROOT` 指向 hudo 安装目录，PATH 中 hudo 目录的优先级取决于追加顺序。如果系统已有机器级 .NET（`C:\Program Files\dotnet`），hudo 检测到后不会重复安装，可选择由 hudo 接管。

### 需要装运行时吗？

SDK 已包含运行时，开发机装 SDK 即可；只跑别人程序的机器才需要单独的 Runtime。

## 相关阅读

- [.NET SDK 安装教程：环境变量、多版本共存与 global.json](/blog/windows-dotnet-install) — 多版本共存的完整讲解


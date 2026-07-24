---
title: "什么是 hudo - Windows 开发环境一键搭建工具"
description: "hudo（混沌）是一个 Windows 开发环境一键引导工具，自动下载安装 Git、Node.js、Rust、Go、JDK 等 26 款开发工具并配置环境变量，告别手动搭建。"
head:
  - - meta
    - name: keywords
      content: "Windows 开发环境工具, hudo, 开发环境搭建, 一键安装开发工具, Windows 环境配置, 开发环境自动化"
---

# 什么是 hudo？

**hudo（混沌）** 是一个 Windows 开发环境一键引导工具。

在新电脑上搭建开发环境是件繁琐的事：下载安装包、配置环境变量、一个个安装……hudo 把这些全部自动化，用一条命令搞定。

## 能做什么

- 交互式选择需要安装的工具，全程免管理员权限（数据库服务注册除外）
- 自动下载并安装到指定盘（不装 C 盘），写用户级环境变量
- `hudo upgrade` 一键升级已装工具，支持版本锁定复现环境
- 内置国内镜像回退与全局代理配置，大陆网络也能顺畅安装
- 导出/导入配置档案，换电脑一条命令还原整套环境
- 所有命令支持 `-y` 非交互模式，可脚本化、可无人值守

## 支持的工具（26 款）

| 分类 | 工具 |
|------|------|
| 版本控制 | Git、GitHub CLI |
| 运行时 | Node.js、fnm、Bun、Python（uv）、Miniconda、Go、Rust、.NET SDK |
| JVM | JDK、Maven、Gradle |
| 数据库 | MySQL、PostgreSQL、Redis |
| IDE | VS Code、PyCharm、IntelliJ IDEA |
| 系统工具 | MinGW（GCC）、7-Zip、PowerShell 7、PowerToys、Oh My Posh、Google Chrome、Claude Code |

## 下一步

- [安装 hudo](/guide/install)
- [快速上手](/guide/quickstart)

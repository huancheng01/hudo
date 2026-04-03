---
title: "什么是 hudo - Windows 开发环境一键搭建工具"
description: "hudo（混沌）是一个 Windows 开发环境一键引导工具，自动下载安装 Git、Node.js、Rust、Go、JDK 等 20+ 开发工具并配置环境变量，告别手动搭建。"
head:
  - - meta
    - name: keywords
      content: "Windows 开发环境工具, hudo, 开发环境搭建, 一键安装开发工具, Windows 环境配置, 开发环境自动化"
---

# 什么是 hudo？

**hudo（混沌）** 是一个 Windows 开发环境一键引导工具。

在新电脑上搭建开发环境是件繁琐的事：下载安装包、配置环境变量、一个个安装……hudo 把这些全部自动化，用一条命令搞定。

## 能做什么

- 交互式选择需要安装的工具
- 自动下载并安装到指定盘（不装 C 盘）
- 自动配置 PATH 等环境变量
- 导出/导入配置档案，换电脑一键还原

## 支持的工具

| 分类 | 工具 |
|------|------|
| 版本控制 | Git、GitHub CLI |
| 运行时 | Node.js、Bun、Python（uv）、Miniconda、Go、Rust |
| JVM | JDK、Maven、Gradle |
| 数据库 | MySQL、PostgreSQL |
| IDE | VS Code、PyCharm |
| 系统工具 | MinGW（GCC）、Google Chrome、Claude Code |

## 下一步

- [安装 hudo](/guide/install)
- [快速上手](/guide/quickstart)

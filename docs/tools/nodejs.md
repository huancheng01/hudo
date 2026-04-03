---
title: "Node.js Windows 安装与配置 - hudo"
description: "使用 hudo 在 Windows 上一键安装 Node.js，自动配置 PATH 和 npm 镜像，支持国内加速下载，无需手动设置环境变量。"
head:
  - - meta
    - name: keywords
      content: "Node.js 安装, Node.js Windows, npm 镜像, Node.js 环境变量, 一键安装 Node.js, hudo"
---

# Node.js

Node.js 是基于 Chrome V8 引擎的 JavaScript 运行时，广泛用于 Web 后端开发、前端构建工具链和命令行工具。hudo 直接安装官方预编译包，无需额外的版本管理器即可快速搭建开发环境。

## 安装

```powershell
hudo install nodejs
```

下载 Node.js 官方 zip 包并解压到 `{install_root}\lang\node\`，自动安装最新 LTS 版本。

## 安装后

安装完成后重新打开终端即可使用 `node`、`npm`、`npx` 命令，支持 CMD、PowerShell、Git Bash 等所有终端。

```powershell
node --version
npm --version
```

## 卸载

```powershell
hudo uninstall nodejs
```

## 配置

指定安装版本：

```toml
[versions]
nodejs = "24.14.1"
```

自定义下载镜像：

```toml
[mirrors]
nodejs = "https://npmmirror.com/mirrors/node/v24.14.1"
```

## hudo 安装优势

- **无需 nvm 等版本管理器**：直接安装官方 LTS 版本，避免 nvm-windows 的各种兼容性问题
- **自动配置所有环境变量**：`node`、`npm`、`npx` 命令安装后即可在所有终端中使用
- **支持[国内镜像加速](/guide/config)**：可配置 npmmirror 等镜像源，解决国内下载 Node.js 安装包慢的问题
- **与 [Bun](/tools/bun) 互补**：hudo 同时支持安装 Node.js 和 Bun，可根据项目需要灵活选择运行时
- **版本锁定**：可在[配置文件](/guide/config)中指定安装特定版本，团队统一开发环境

## 常见问题

**Q: 安装后 `node` 命令找不到怎么办？**

打开新的终端窗口即可，hudo 已自动配置环境变量，需要新终端加载。

**Q: npm 下载包很慢怎么办？**

可以配置 npm 使用国内镜像：`npm config set registry https://registry.npmmirror.com`。或者考虑使用 [Bun](/tools/bun) 作为替代包管理器，安装速度更快。

**Q: hudo 安装的 Node.js 和 nvm-windows 冲突吗？**

如果系统中已安装 nvm-windows，建议先卸载以避免 PATH 冲突。hudo 直接管理 Node.js 安装，不需要额外的版本管理器。

## 相关阅读

- [2026 年 Windows Node.js 环境搭建终极方案](/blog/windows-nodejs-install) — 详细安装教程与常见问题解答

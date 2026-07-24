---
title: "Bun Windows 安装与配置"
description: "使用 hudo 在 Windows 上一键安装 Bun 运行时，自动配置环境变量，高性能 JavaScript/TypeScript 运行时与包管理器。"
head:
  - - meta
    - name: keywords
      content: "Bun 安装, Bun Windows, Bun 运行时, JavaScript 运行时, 一键安装 Bun, hudo"
---

# Bun

Bun 是用 Zig 编写的高性能 JavaScript/TypeScript 运行时，集运行时、包管理器、打包器和测试运行器于一体。它兼容大部分 Node.js API，启动速度和包安装速度远超传统方案，适合追求极致开发效率的前端和全栈开发者。

## 安装

```powershell
hudo install bun
```

安装到 `{install_root}\tools\bun\`。

## 安装后

```powershell
bun --version
bun run index.ts
bun install
```

## 卸载

```powershell
hudo uninstall bun
```

## hudo 安装优势

- **一条命令完成安装**：无需通过 npm 或 PowerShell 脚本安装，`hudo install bun` 直接下载官方二进制文件
- **自动配置 PATH**：安装后 `bun` 命令在所有终端中立即可用
- **与 [Node.js](/tools/nodejs) 共存**：hudo 分别管理 Bun 和 Node.js 的安装目录，两者互不干扰，可根据项目需要切换使用
- **统一管理**：安装在 `{install_root}\tools\bun\`，`hudo uninstall bun` 干净卸载

## 常见问题

**Q: 安装后 `bun` 命令找不到怎么办？**

打开新的终端窗口即可，hudo 已自动配置环境变量，需要新终端加载。

**Q: Bun 能完全替代 Node.js 吗？**

Bun 兼容大部分 Node.js API，但某些依赖原生模块的包可能存在兼容性问题。建议同时安装 [Node.js](/tools/nodejs) 和 Bun，在新项目中优先尝试 Bun，遇到兼容性问题时回退到 Node.js。

**Q: Bun 和 npm/pnpm 有什么区别？**

Bun 不仅是包管理器，还是完整的 JavaScript/TypeScript 运行时，内置打包器和测试运行器。包安装速度通常比 npm 快 10 倍以上。

## 相关阅读

- [Bun：比 Node.js 快数倍的 JS 运行时，Windows 一分钟上手](/blog/windows-bun-install) — 详细安装教程与常见问题解答

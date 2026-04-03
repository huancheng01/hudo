---
title: "VS Code 安装配置 - Windows 一键安装 Visual Studio Code - hudo"
description: "使用 hudo 一键安装 VS Code 编辑器，自动配置 PATH 环境变量和右键菜单，支持命令行 code 命令直接打开项目。"
head:
  - - meta
    - name: keywords
      content: "VS Code 安装, Visual Studio Code, Windows VS Code, VS Code 配置, 代码编辑器, VS Code 下载, hudo"
---

# VS Code

Visual Studio Code 是微软推出的免费、开源代码编辑器，凭借丰富的扩展生态、内置终端和 Git 集成，已成为全球开发者使用最广泛的编辑器。hudo 安装的是免安装便携版，不污染系统注册表，干净可控。

Microsoft Visual Studio Code，轻量级代码编辑器。

## 安装

```powershell
hudo install vscode
```

安装到 `{install_root}\ide\vscode\`，免安装版（zip）。

安装时自动注册 Windows 右键菜单「通过 Code 打开」，支持：

- 右键文件 → 通过 Code 打开
- 右键文件夹 → 通过 Code 打开
- 右键文件夹空白处 → 通过 Code 打开

## 安装后

```powershell
code --version
code .
```

## 卸载

```powershell
hudo uninstall vscode
```

卸载时自动清理右键菜单注册表项。

## hudo 安装优势

- **免安装便携版**：使用 zip 解压方式，不写入系统注册表，不占用 `Program Files`
- **自动注册右键菜单**：安装后即可在文件和文件夹上右键「通过 Code 打开」，卸载时自动清理
- **PATH 自动配置**：终端直接使用 `code` 命令打开项目，无需手动设置
- **干净卸载**：连同右键菜单注册表项一并清除，不留残余

## 常见问题

### 便携版和官方安装版有什么区别？

功能完全相同。hudo 安装的便携版不写入 `Program Files`，不需要管理员权限，卸载更干净。扩展和设置存储在 VS Code 目录下的 `data` 文件夹中。

### 安装后右键菜单没有出现？

重新启动 Windows 资源管理器或注销重新登录即可。如果仍然没有，尝试 `hudo uninstall vscode` 后重新安装。

### 推荐搭配哪些开发工具？

配合 [Git](/tools/git) 使用内置版本控制功能，搭配 [Node.js](/tools/nodejs) 进行前端开发，使用 [Python (uv)](/tools/python) 或 [Miniconda](/tools/miniconda) 进行 Python 开发。

## 相关阅读

- [VS Code 便携版安装：免安装器 + 右键菜单 + 数据隔离，Windows 最干净的方案](/blog/windows-vscode-install) — 详细安装教程与常见问题解答

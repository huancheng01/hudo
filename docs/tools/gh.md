---
title: "GitHub CLI (gh) Windows 安装"
description: "使用 hudo 在 Windows 上一键安装 GitHub CLI，自动配置环境变量，快速管理 PR、Issue 和 Release，提升 GitHub 工作流效率。"
head:
  - - meta
    - name: keywords
      content: "GitHub CLI, gh 安装, gh Windows, GitHub 命令行工具, 一键安装, hudo"
---

# GitHub CLI

GitHub CLI（`gh`）是 GitHub 官方推出的命令行工具，可以直接在终端中管理 PR、Issue、Release 和仓库，无需频繁切换到浏览器。对于日常使用 GitHub 的开发者来说，它能大幅提升工作流效率。

## 安装

```powershell
hudo install gh
```

安装到 `{install_root}\tools\gh\`，自动获取最新版本。

## 安装后

首次使用需要登录：

```powershell
gh auth login
```

## 常用命令

```powershell
gh repo clone owner/repo
gh pr list
gh issue create
gh release create v1.0.0
```

## 注意

GitHub CLI 的登录状态不会导出到 profile 档案，换电脑后需重新运行 `gh auth login`。

## 卸载

```powershell
hudo uninstall gh
```

## hudo 安装优势

- **免去手动下载和解压**：`hudo install gh` 自动从 GitHub Releases 获取最新版本并安装
- **自动配置 PATH**：安装后 `gh` 命令在所有终端中立即可用，无需手动添加环境变量
- **与 [Git](/tools/git) 配合使用**：hudo 同时管理 Git 和 GitHub CLI，一套工具链搞定版本控制工作流
- **干净卸载**：所有文件集中在 `{install_root}\tools\gh\`，`hudo uninstall gh` 一键清除

## 常见问题

**Q: 安装后 `gh` 命令找不到怎么办？**

打开新的终端窗口即可，hudo 已自动配置环境变量，需要新终端加载。

**Q: 换电脑后 `gh` 提示未登录怎么办？**

这是正常的。出于安全考虑，GitHub CLI 的登录凭证不会导出到 [profile 档案](/guide/quickstart)中。在新设备上运行 `gh auth login` 重新授权即可。

**Q: gh 和 Git 需要分别安装吗？**

是的，`gh` 是 GitHub 的命令行管理工具，[Git](/tools/git) 是版本控制系统，两者功能不同。建议同时安装以获得完整的 GitHub 工作流体验。

## 相关阅读

- [告别网页操作：GitHub CLI 安装与十个最实用命令](/blog/windows-gh-install) — 详细安装教程与常见问题解答

---
title: 告别网页操作：GitHub CLI 安装与十个最实用命令
description: 详细介绍如何在 Windows 上安装 GitHub CLI（gh），对比传统手动下载配置方式和使用 hudo 一条命令自动安装的体验差异，并精选十个最实用的 gh 命令，涵盖 PR 创建、Issue 管理、CI 状态查看、仓库操作等日常高频场景。
keywords:
  - Windows GitHub CLI 安装
  - gh 命令行工具
  - GitHub CLI 教程
  - Git 命令行
  - hudo
  - GitHub PR 命令行
---

# 告别网页操作：GitHub CLI 安装与十个最实用命令

大多数开发者和 GitHub 打交道的方式是：打开浏览器，登录网页，点来点去。创建 PR 要开网页，看 Issue 要开网页，查 CI 跑没跑完还是要开网页。操作不算复杂，但频繁在编辑器和浏览器之间切换，节奏总会被打断。

GitHub CLI（命令行里叫 `gh`）就是用来解决这个问题的。它是 GitHub 官方出品的命令行工具，能让你在终端里直接完成 PR、Issue、仓库、CI 等几乎所有操作，不需要打开浏览器。

## gh 能做什么

一句话概括：**在命令行里完成你在 GitHub 网页上做的绝大多数事情。**

- 创建和审查 Pull Request
- 管理 Issue：创建、列出、关闭
- 克隆仓库，甚至直接创建新仓库
- 查看 CI/CD 运行状态和日志
- 管理 Release 和 Tag
- 查看通知、搜索代码

这些操作在终端里执行，比在网页上点击快得多。尤其是在你已经处于命令行环境中写代码的时候，顺手就能完成 GitHub 操作，工作流不会中断。

## 传统安装方式

手动安装 gh 需要这些步骤：

1. 打开 `github.com/cli/cli/releases`，找到最新版本
2. 下载 `gh_x.y.z_windows_amd64.zip`
3. 解压到一个你记得住的目录
4. 把该目录手动添加到系统 PATH 环境变量
5. 重新打开终端验证 `gh --version`

对于国内用户，第二步往往是最难受的——GitHub Releases 的下载速度时好时坏，有时候几 MB 的文件能卡好几分钟。配置 PATH 也容易出错，忘了加或者路径写错了，终端里就找不到 `gh` 命令。

## 用 hudo 一条命令搞定

[hudo](https://hudo.zexa.cc) 是一个 Windows 开发环境引导工具，可以把下载、解压、配置 PATH 这些重复劳动全部自动化。安装 gh 只需要：

```powershell
hudo install gh
```

执行过程：

1. 自动从 GitHub 下载最新版 gh（下载失败自动回退国内镜像）
2. 解压到 `{安装根目录}\tools\gh\`
3. 自动将路径写入用户 PATH 环境变量
4. 安装完成后引导你执行 `gh auth login` 完成 OAuth 登录

整个过程不需要手动配置任何东西。登录环节会自动打开浏览器，在 GitHub 页面上授权即可。

如果你还没有安装 hudo，先执行：

```powershell
irm hudo.zexa.cc/install.ps1 | iex
```

## 十个最实用的 gh 命令

安装并登录之后，下面这十个命令基本覆盖了日常开发中最常用的场景。

### 1. 克隆仓库

```bash
gh repo clone owner/repo
```

比 `git clone` 方便的地方在于，不需要拼完整的 URL，直接用 `用户名/仓库名` 就行。

### 2. 创建 Pull Request

```bash
gh pr create
```

在当前分支上直接创建 PR，交互式填写标题和描述。也可以加参数一步到位：

```bash
gh pr create --title "修复登录问题" --body "详细描述"
```

### 3. 查看 PR 列表

```bash
gh pr list
```

列出当前仓库所有打开的 PR，包括编号、标题和作者。

### 4. 查看 PR 详情

```bash
gh pr view 42
```

查看指定编号的 PR 详情，包括描述、审查状态、CI 结果。加 `--web` 会在浏览器中打开。

### 5. 创建 Issue

```bash
gh issue create
```

交互式创建 Issue，可以选标签和指派人。

### 6. 查看 Issue 列表

```bash
gh issue list
```

列出当前仓库的 Issue，支持 `--label` 等过滤条件。

### 7. 查看 CI 运行状态

```bash
gh run list
```

列出最近的 GitHub Actions 运行记录，能看到每次 push 或 PR 触发的 CI 是成功还是失败。

### 8. 查看 CI 运行详情

```bash
gh run view <run-id>
```

查看具体某次 CI 运行的详情。加 `--log` 可以直接在终端看完整日志，排查问题不用开网页。

### 9. 创建新仓库

```bash
gh repo create my-project --public
```

直接从命令行创建 GitHub 仓库，支持 `--public`、`--private`、`--clone` 等参数。

### 10. 查看登录状态

```bash
gh auth status
```

确认当前的 GitHub 登录状态和权限范围，排查认证问题时很有用。

## 小结

GitHub CLI 把 GitHub 操作从浏览器搬到了终端，对于日常在命令行里写代码的开发者来说，效率提升非常明显。在 Windows 上通过 hudo 安装只需要一条命令，省去了手动下载和配置环境变量的麻烦。如果你还没有用过 `gh`，值得试一试。

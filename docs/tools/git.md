---
title: "Git Windows 安装与配置 - hudo"
description: "使用 hudo 在 Windows 上一键安装 Git，自动配置 PATH 环境变量，支持国内镜像加速下载，免去手动配置烦恼。"
head:
  - - meta
    - name: keywords
      content: "Git 安装, Git Windows, Git 下载, Git 环境变量, 一键安装 Git, hudo"
---

# Git

Git 是全球最流行的分布式版本控制系统，几乎所有现代软件项目都依赖它进行代码管理。无论是个人项目还是团队协作，Git 都是 Windows 开发者的必备工具。

## 安装

```powershell
hudo install git
```

安装到 `{install_root}\tools\git\`，自动获取最新版本。

## 安装后

```powershell
git --version
git config --global user.name "Your Name"
git config --global user.email "you@example.com"
```

## 卸载

```powershell
hudo uninstall git
```

## 配置文件版本

```toml
[versions]
# 不填则自动获取最新版
git = "2.47.0"
```

## hudo 安装优势

- **一条命令完成安装**：无需手动下载安装包、点击安装向导，`hudo install git` 全自动完成
- **自动配置 PATH 环境变量**：安装后无需手动编辑系统环境变量，打开新终端即可使用 `git` 命令
- **自动获取最新版本**：通过 GitHub API 动态获取最新 Git for Windows 版本，也支持在[配置文件](/guide/config)中锁定指定版本
- **统一管理安装目录**：Git 安装在 `{install_root}\tools\git\`，不会污染系统目录，[卸载](/guide/quickstart#卸载工具)干净彻底
- **支持 [profile 档案导出](/guide/quickstart)**：团队成员可以共享相同的工具版本配置，保证开发环境一致性

## 常见问题

**Q: 安装后 `git` 命令找不到怎么办？**

打开新的终端窗口即可，hudo 已自动配置环境变量，需要新终端加载。如果仍然找不到，运行 `hudo list` 确认 Git 已安装成功。

**Q: 系统已经安装了 Git，hudo 会冲突吗？**

不会。hudo 会检测系统中已有的 Git 安装，如果检测到外部安装（如通过官方安装包安装），会提示你选择是否用 hudo 重新安装。两者不会冲突，但建议统一管理以避免版本混乱。

**Q: 如何升级 Git 版本？**

重新运行 `hudo install git` 即可，hudo 会自动下载最新版本覆盖安装。

## 相关阅读

- [Windows Git 安装与配置教程：一条命令搞定全部步骤](/blog/windows-git-install) — 详细安装教程与常见问题解答

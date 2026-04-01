---
title: Windows Git 安装与配置教程：一条命令搞定全部步骤
description: 详细介绍 Windows 系统下安装和配置 Git 的完整教程，包括传统手动安装方式的痛点分析，以及使用 hudo 工具一条命令自动完成 Git 下载、静默安装、PATH 环境变量配置、user.name/email 设置和 autocrlf 换行符配置的全流程方案。
keywords:
  - Windows Git 安装
  - Git 配置教程
  - Windows 开发环境
  - Git 环境变量
  - Git autocrlf
  - hudo
---

# Windows Git 安装与配置教程：一条命令搞定全部步骤

在 Windows 上安装 Git，听起来很简单——下载一个安装包，点几下"下一步"就行了。但真正操作过的人都知道，这件事远没有想象中顺畅。

## 传统安装方式有多麻烦？

先说下载。Git 官网的下载速度对国内用户非常不友好，几十 MB 的安装包有时要等好几分钟。好不容易下载完，打开安装程序后迎面而来的是一连串选项：

- 默认编辑器选 Vim 还是 Notepad++？
- PATH 环境变量是只在 Git Bash 用还是加到系统 PATH？
- HTTPS 传输后端选 OpenSSL 还是 Windows 原生？
- 换行符转换策略选哪个？
- 终端模拟器用 MinTTY 还是 Windows 默认终端？

这些选项对新手来说基本是天书。大多数人只能硬着头皮一路"Next"，装完之后打开 CMD 输入 `git`，发现提示"不是内部或外部命令"——因为 PATH 没配对。

即使安装成功了，还有后续配置：

```bash
# 手动设置用户信息
git config --global user.name "Your Name"
git config --global user.email "you@example.com"

# 解决 Windows 换行符问题
git config --global core.autocrlf true
```

忘了设置 `user.name` 和 `user.email`？第一次 commit 的时候 Git 会报错拒绝提交。忘了设置 `core.autocrlf`？团队协作时换行符混乱，diff 里满屏都是无意义的修改。

整套流程走下来，少说十几分钟，还容易踩坑。如果你在多台机器上重复这个过程，痛苦会成倍增加。

## 用 hudo 一条命令完成安装

[hudo](https://hudo.zexa.cc) 是一个 Windows 开发环境引导工具，可以用一条命令完成 Git 的下载、安装和配置。

首先安装 hudo（如果还没有的话）：

```powershell
irm hudo.zexa.cc/install.ps1 | iex
```

然后安装 Git：

```powershell
hudo install git
```

执行后，hudo 会自动完成以下操作：

1. **查询并下载最新版 Git for Windows**——从 GitHub Releases 获取最新版本，下载失败时自动回退到国内 npmmirror 镜像，不用担心网络问题
2. **静默安装**——无需点击任何选项，自动选择最佳配置并安装到统一目录
3. **配置 PATH 环境变量**——安装完成后自动将 Git 写入用户 PATH，新开终端即可使用
4. **交互式配置身份信息**——安装后提示你输入 `user.name` 和 `user.email`，如果之前配置过会自动读取作为默认值
5. **设置 `core.autocrlf=true`**——自动处理 Windows 换行符问题，避免团队协作中的换行符冲突

整个过程通常在一两分钟内完成。

## 验证安装结果

安装完成后，**打开一个新的终端窗口**（这一步很重要，新的 PATH 需要新终端才能生效），运行以下命令验证：

```bash
git --version
```

正常输出示例：

```
git version 2.47.1.windows.2
```

检查配置是否已生效：

```bash
git config --global --list
```

你应该能看到类似以下内容：

```
user.name=你的名字
user.email=you@example.com
core.autocrlf=true
```

如果以上输出都正常，说明 Git 已经安装并配置完毕，可以直接开始使用了。

## 指定版本安装

如果你的项目需要特定版本的 Git，可以在 hudo 配置文件 `%USERPROFILE%\.hudo\config.toml` 中指定：

```toml
[versions]
git = "2.47.0"
```

然后重新运行 `hudo install git`，hudo 会安装你指定的版本。

## 卸载同样简单

不需要了？一条命令卸载：

```powershell
hudo uninstall git
```

## 总结

在 Windows 上安装 Git 的核心问题在于：下载慢、选项多、配置散。这些琐碎的步骤不断消耗开发者的时间和耐心。`hudo install git` 把这些步骤压缩成一条命令，让你把精力留给真正重要的事——写代码。

如果你想了解 hudo 支持的完整工具列表和更多用法，可以访问 [hudo 文档站](https://hudo.zexa.cc)。

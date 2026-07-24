---
title: "Claude Code 安装配置 - Windows 一键安装 AI 编程助手"
description: "使用 hudo 一键安装 Anthropic Claude Code CLI，AI 驱动的命令行编程助手，自动配置 Node.js 依赖和环境变量。"
head:
  - - meta
    - name: keywords
      content: "Claude Code 安装, Claude Code CLI, AI 编程助手, Anthropic Claude, Windows Claude Code, AI 代码工具, hudo"
---

# Claude Code

Claude Code 是 Anthropic 推出的 AI 命令行编程助手，能够直接在终端中理解代码库、编写代码、执行命令和管理 Git 操作。它是目前最强大的 AI 编程工具之一，特别适合代码审查、重构和复杂任务自动化。

Anthropic Claude Code CLI，AI 驱动的命令行编程助手。

## 安装

```powershell
hudo install claude-code
```

从 Google Cloud Storage 下载官方二进制，安装到 `{root_dir}\tools\claude-code\claude.exe`，并进行 SHA256 完整性校验。GCS 不可达时（如网络受限）自动回退为 npm 安装，装出的 `claude.cmd` 同样会被 hudo 正确检测和管理。

如果系统中已通过 npm 安装了 Claude Code，hudo 会自动卸载旧版（`npm uninstall -g @anthropic-ai/claude-code`）后重新安装到 hudo 目录。SHA256 校验失败时会自动清除缓存并重试一次。

## 安装后

启动 claude，在对话中输入 `/login` 登录账号：

```powershell
claude
# 进入对话后输入 /login
```

或设置 API Key：

```powershell
$env:ANTHROPIC_API_KEY = "sk-ant-..."
```

## API 来源管理（hudo cc）

使用第三方 API 中转时，无需手动改配置文件，运行 `hudo cc`（或主菜单 `[K] Claude Code API 来源`）管理 Provider：

- **添加**：录入名称、Base URL（校验 http/https 前缀）、API Key（输入不回显），可选配置自定义模型；添加后可选择立即切换
- **查看 / 编辑**：选中 Provider 进入详情页（API Key 脱敏显示），支持逐字段编辑，回车保留原值
- **切换**：写入 `~/.claude/settings.json`，重启终端或 Claude Code 后生效
- **删除**：删除当前激活的 Provider 时会提示是否同时恢复官方默认配置
- **恢复默认**：一键清除全部自定义 API 配置

::: warning 档案导出提醒
`hudo export` 导出的环境档案会包含 Provider 的 API Key 明文（导出前有提示），请勿分享或提交到仓库。
:::

## 使用

```powershell
# 在项目目录启动
cd my-project
claude
```

## 卸载

```powershell
hudo uninstall claude-code
```

## 配置文件版本

```toml
[versions]
# 不填则自动获取最新版
claude_code = "1.0.0"
```

## hudo 安装优势

- **官方二进制直装**：从 Google Cloud Storage 下载官方编译包，不依赖 npm 全局安装
- **SHA256 完整性校验**：自动验证下载文件的哈希值，确保安装包未被篡改
- **自动清理旧版**：如果系统中已通过 npm 安装了 Claude Code，hudo 会自动卸载旧版后重装
- **版本可锁定**：通过[配置文件](/guide/config)指定版本号，避免自动更新导致行为变化
- **无需手动管理 Node.js**：使用独立二进制，不需要先安装 [Node.js](/tools/nodejs) 环境

## 常见问题

### Claude Code 需要付费吗？

Claude Code 本身免费安装，但使用时需要 Anthropic 账号或 API Key。可以运行 `claude` 后在对话中输入 `/login` 登录账号，或设置 `ANTHROPIC_API_KEY` 环境变量，或用 `hudo cc` 配置第三方 API 来源。

### 安装失败提示 SHA256 校验不通过？

hudo 会自动清除缓存并重试一次。如果仍然失败，可能是网络问题导致下载不完整，检查网络连接后重试 `hudo install claude-code`。

### 如何更新 Claude Code 到最新版？

重新运行 `hudo install claude-code` 即可，hudo 会自动获取并安装最新版本。

## 相关阅读

- [Claude Code 完整安装教程：Windows 开发者从零开始的 AI 编程助手](/blog/windows-claude-code-install) — 详细安装教程与常见问题解答

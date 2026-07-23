---
title: "配置档案 - 开发环境迁移与同步"
description: "使用 hudo 配置档案功能在多台电脑间导出和导入开发环境配置，一键迁移工具列表和版本设置，快速还原开发环境。"
head:
  - - meta
    - name: keywords
      content: "开发环境迁移, hudo profile, 环境配置导出, 开发环境同步, 多设备开发环境, hudo 档案"
---

# 配置档案

配置档案功能让你在多台电脑间同步开发环境配置。

## 导出档案

```powershell
hudo export
```

会在当前目录生成 `hudo-profile.toml`，记录已安装工具及版本、镜像与版本锁定配置、工具级配置（如 Git 身份）、Claude Code API 来源。

## 导入档案

在新电脑上安装好 hudo 后：

```powershell
hudo import hudo-profile.toml

# 无人值守（脚本/自动化）
hudo import hudo-profile.toml -y
```

hudo 会展示配置变更与待安装工具清单，确认后批量安装并应用配置。

## 新机一条命令还原

不必分两步——安装 hudo 时直接带上档案地址，装完自动导入：

```powershell
$env:HUDO_PROFILE = "https://example.com/hudo-profile.toml"   # 也可以是本地路径
irm hudo.zexa.cc/install.ps1 | iex
```

## 档案文件格式

```toml
[tools]
git = "2.47.0"
nodejs = "22.0.0"
go = "1.23.0"
vscode = "1.95.0"

[settings.mirrors]
nodejs = "https://npmmirror.com/mirrors/node"

[settings.versions]
nodejs = "22.0.0"
```

## 注意事项

- GitHub CLI 的登录状态**不会**导出到档案（出于安全考虑），导入后会自动提示运行 `gh auth login` 完成认证
- 档案可能包含 Claude Code API 密钥明文（导出时会提示），不要放到公开可访问的位置
- 档案文件可以提交到团队私有仓库，统一团队开发环境

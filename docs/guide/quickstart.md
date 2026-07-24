---
title: "快速上手 - 安装、升级与一键还原环境"
description: "hudo 快速上手指南：学习如何通过交互式菜单安装开发工具、管理已安装工具、卸载工具，五分钟搭建完整 Windows 开发环境。"
head:
  - - meta
    - name: keywords
      content: "hudo 使用教程, hudo 快速上手, hudo 命令, 开发工具安装教程, Windows 开发环境配置教程"
---

# 快速上手

## 第一次运行

安装 hudo 后，直接运行：

```powershell
hudo
```

首次运行会询问工具安装根目录（如 `D:\`），之后进入交互式安装菜单。

## 安装工具

```powershell
# 进入交互菜单，方向键选择，空格勾选，回车确认
hudo

# 直接安装指定工具
hudo install git
hudo install nodejs
hudo install vscode
```

## 查看已安装工具

```powershell
hudo list
```

## 升级工具

```powershell
# 检查全部 hudo 安装的工具并升级
hudo upgrade

# 升级单个工具 / 非交互升级
hudo upgrade nodejs
hudo upgrade -y
```

升级目标为配置锁定版本（如有）或 API 查询到的最新版；升级不重跑配置、不改环境变量。以下工具不参与自动升级：

- **mysql / pgsql / redis** — 数据目录在安装目录内，自动重装会丢数据，请备份后手动 uninstall + install
- **miniconda** — conda 环境在安装目录内，用 `conda update conda`
- **rust** — 用 `rustup update`
- **chrome** — 自带自动更新

## 更新 hudo 自身

```powershell
hudo update
```

## 卸载工具

```powershell
hudo uninstall git
```

## 配置档案

```powershell
# 导出当前环境档案（默认 hudo-profile.toml）
hudo export

# 在新电脑上还原
hudo import hudo-profile.toml
```

## 非交互模式（脚本/自动化）

所有命令支持 `-y/--yes`，跳过确认提示，适合脚本和无人值守场景：

```powershell
hudo install git -y          # 直接安装，不再询问
hudo import profile.toml -y  # 一条命令还原整套环境
hudo uninstall git -y        # 直接卸载
```

`-y` 只自动确认「推进类」提示（开始安装、确认卸载、确认导入等）；**可选分支保持安全默认**——例如检测到系统里已有外部安装的同名工具时，`-y` 不会自动接管（接管会卸载原安装，必须交互确认）。文本输入取默认值，无默认值时报错退出而不是猜测。首次运行的安装盘选择在 `-y` 下自动选首个非系统盘。

## 新机一条命令还原环境

在旧机器上 `hudo export` 导出档案并放到可访问的位置（私有网盘/内网/仓库），新机器上：

```powershell
# 安装 hudo + 导入档案 + 批量安装工具，全程无人值守
$env:HUDO_PROFILE = "https://example.com/hudo-profile.toml"   # 也可以是本地路径
irm hudo.zexa.cc/install.ps1 | iex
```

::: warning 档案含敏感信息
档案可能包含 Claude Code API 密钥明文（导出时会提示），不要放到公开可访问的地址。
:::

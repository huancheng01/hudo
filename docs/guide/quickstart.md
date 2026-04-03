---
title: "快速上手 - hudo 使用教程"
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

## 更新工具

```powershell
# 更新 hudo 自身
hudo update
```

## 卸载工具

```powershell
hudo uninstall git
```

## 配置档案

```powershell
# 导出当前配置
hudo profile export

# 在新电脑上还原
hudo profile import
```
